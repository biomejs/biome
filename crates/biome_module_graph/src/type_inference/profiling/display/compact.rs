//! Grouped terminal output for type-inference profiles.
//!
//! The report groups requests by analyzer and queries by the Rust function that
//! ran. Each group shows its slowest source location. Request and query groups
//! have separate display limits.

use std::collections::BTreeMap;
use std::io;
use std::time::Duration;

use biome_console::fmt::{Display, Formatter};
use biome_console::{HARD_LINE, HorizontalLine, Padding, markup};
use camino::Utf8Path;

use super::{
    CapacityWarning, HighlightedDuration, RECORD_INDENT, SourceLocation, TimingCutoffs,
    TimingMetrics,
};
use crate::type_inference::profiling::{
    RequestMetadata, TypeInferenceProfileLocation, TypeInferenceProfileSnapshot,
    TypeInferenceQueryProfile, TypeInferenceRequestProfile, TypeInferenceWholeModuleProfile,
};
use crate::type_inference::{
    TypeInferenceCaller, TypeInferenceCodeReference, TypeInferenceQueryKind,
    TypeInferenceWholeModuleReason,
};

const REQUEST_GROUP_LIMIT: usize = 5;
const QUERY_GROUP_LIMIT: usize = 8;

pub(super) struct CompactTypeInferenceProfile<'a> {
    snapshot: &'a TypeInferenceProfileSnapshot,
    working_directory: Option<&'a Utf8Path>,
    version: &'a str,
}

impl<'a> CompactTypeInferenceProfile<'a> {
    pub(super) const fn new(
        snapshot: &'a TypeInferenceProfileSnapshot,
        working_directory: Option<&'a Utf8Path>,
        version: &'a str,
    ) -> Self {
        Self {
            snapshot,
            working_directory,
            version,
        }
    }
}

impl Display for CompactTypeInferenceProfile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let request_runs = self
            .snapshot
            .requests
            .iter()
            .map(|profile| u64::from(profile.completed))
            .sum::<u64>();
        let query_runs = self
            .snapshot
            .queries
            .iter()
            .map(|profile| u64::from(profile.completed))
            .sum::<u64>();
        let whole_module_runs = self
            .snapshot
            .whole_module_inferences
            .iter()
            .map(|profile| u64::from(profile.completed))
            .sum::<u64>();

        f.write_markup(markup! {
            {HorizontalLine::new(20)}
            <Emphasis>"Type inference profile"</Emphasis>" "<Dim>"(Biome "{self.version}")"</Dim>"\n"
            <Dim>"Timings are inclusive and non-additive; the highest 10% are highlighted in sections with at least ten groups."</Dim>"\n"
            <Dim>"Completed runs: requests "{request_runs}", tracked queries "{query_runs}", whole-module "{whole_module_runs}"."</Dim>"\n"
            <Dim>"Showing ranked aggregates; use --verbose for every source record and code reference."</Dim>"\n"
            <Dim>"Ranges use zero-based UTF-8 byte offsets; paths may reveal project structure."</Dim>
            {HARD_LINE}
        })?;

        if self.snapshot.is_empty() {
            f.write_markup(markup! {
                "No type-inference requests or queries were recorded.\n"
                {CapacityWarning(self.snapshot)}
            })?;
            return Ok(());
        }

        f.write_markup(markup! {
            {CompactRequestProfiles {
                profiles: &self.snapshot.requests,
                working_directory: self.working_directory,
            }}
            {CompactQueryProfiles {
                profiles: &self.snapshot.queries,
                working_directory: self.working_directory,
            }}
            {CompactWholeModuleProfiles {
                profiles: &self.snapshot.whole_module_inferences,
                working_directory: self.working_directory,
            }}
            {CapacityWarning(self.snapshot)}
        })
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct RequestGroupKey {
    metadata: RequestMetadata,
    caller: TypeInferenceCaller,
    implementation: TypeInferenceCodeReference,
}

struct RequestGroup<'a> {
    key: RequestGroupKey,
    timing: TimingAccumulator,
    hottest: &'a TypeInferenceRequestProfile,
}

impl<'a> RequestGroup<'a> {
    fn new(profile: &'a TypeInferenceRequestProfile) -> Self {
        let mut timing = TimingAccumulator::default();
        timing.add(TimingMetrics::from(profile));
        Self {
            key: RequestGroupKey {
                metadata: profile.metadata,
                caller: profile.caller,
                implementation: profile.implementation,
            },
            timing,
            hottest: profile,
        }
    }

    fn add(&mut self, profile: &'a TypeInferenceRequestProfile) {
        self.timing.add(TimingMetrics::from(profile));
        if profile.location.is_hotter_than(
            profile.total,
            profile.max,
            &self.hottest.location,
            self.hottest.total,
            self.hottest.max,
        ) {
            self.hottest = profile;
        }
    }
}

struct CompactRequestProfiles<'a> {
    profiles: &'a [TypeInferenceRequestProfile],
    working_directory: Option<&'a Utf8Path>,
}

impl Display for CompactRequestProfiles<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        if self.profiles.is_empty() {
            return Ok(());
        }

        let groups = aggregate_request_profiles(self.profiles);
        let shown = groups.len().min(REQUEST_GROUP_LIMIT);
        let cutoffs = TimingCutoffs::new(groups.iter().map(|group| group.timing.metrics()));
        f.write_markup(markup! {
            <Emphasis>"Requests by consumer"</Emphasis>" "<Dim>"(top "{shown}" of "{Count::groups(groups.len())}"; "{Count::source_records(self.profiles.len())}")"</Dim>"\n"
        })?;
        for (index, group) in groups.iter().take(shown).enumerate() {
            f.write_markup(markup! {
                {CompactRequestProfileRecord {
                    index: index + 1,
                    group,
                    working_directory: self.working_directory,
                    cutoffs,
                }}
            })?;
        }
        let omitted = groups.len().saturating_sub(shown);
        if omitted > 0 {
            f.write_markup(markup! {
                {Padding::new(RECORD_INDENT)}<Dim>{Count::groups(omitted)}" omitted; use --verbose for every source record."</Dim>"\n"
            })?;
        }
        f.write_markup(markup! {{HARD_LINE}})
    }
}

struct CompactRequestProfileRecord<'a> {
    index: usize,
    group: &'a RequestGroup<'a>,
    working_directory: Option<&'a Utf8Path>,
    cutoffs: TimingCutoffs,
}

impl Display for CompactRequestProfileRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let group = self.group;
        f.write_markup(markup! {
            {self.index}". "<Info>{group.key.metadata.label()}</Info><Dim>" <- "</Dim>{group.key.caller.group()}"/"{group.key.caller.name()}"\n"
            {CompactTimingMetricsDisplay::new(group.timing.metrics(), self.cutoffs)}
            {Padding::new(RECORD_INDENT)}<Dim>"hottest origin: "</Dim>{SourceLocation::new(&group.hottest.location, self.working_directory)}"\n"
        })
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct QueryGroupKey {
    kind: TypeInferenceQueryKind,
    implementation: TypeInferenceCodeReference,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct QueryConsumerKey {
    request: RequestMetadata,
    caller: TypeInferenceCaller,
}

struct QueryGroup<'a> {
    key: QueryGroupKey,
    timing: TimingAccumulator,
    consumers: BTreeMap<QueryConsumerKey, TimingAccumulator>,
    hottest: &'a TypeInferenceQueryProfile,
}

impl<'a> QueryGroup<'a> {
    fn new(profile: &'a TypeInferenceQueryProfile) -> Self {
        let mut group = Self {
            key: QueryGroupKey {
                kind: profile.kind,
                implementation: profile.implementation,
            },
            timing: TimingAccumulator::default(),
            consumers: BTreeMap::new(),
            hottest: profile,
        };
        group.add(profile);
        group
    }

    fn add(&mut self, profile: &'a TypeInferenceQueryProfile) {
        let metrics = TimingMetrics::from(profile);
        self.timing.add(metrics);
        self.consumers
            .entry(QueryConsumerKey {
                request: profile.request,
                caller: profile.caller,
            })
            .or_default()
            .add(metrics);
        if profile.location.is_hotter_than(
            profile.total,
            profile.max,
            &self.hottest.location,
            self.hottest.total,
            self.hottest.max,
        ) {
            self.hottest = profile;
        }
    }

    fn top_consumer(&self) -> Option<QueryConsumerKey> {
        self.consumers
            .iter()
            .max_by(|(left_key, left), (right_key, right)| {
                left.cmp(right).then_with(|| right_key.cmp(left_key))
            })
            .map(|(key, _)| *key)
    }
}

struct CompactQueryProfiles<'a> {
    profiles: &'a [TypeInferenceQueryProfile],
    working_directory: Option<&'a Utf8Path>,
}

impl Display for CompactQueryProfiles<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        if self.profiles.is_empty() {
            return Ok(());
        }

        let groups = aggregate_query_profiles(self.profiles);
        let shown = groups.len().min(QUERY_GROUP_LIMIT);
        let cutoffs = TimingCutoffs::new(groups.iter().map(|group| group.timing.metrics()));
        f.write_markup(markup! {
            <Emphasis>"Query bodies"</Emphasis>" "<Dim>"(top "{shown}" of "{Count::groups(groups.len())}"; "{Count::source_records(self.profiles.len())}")"</Dim>"\n"
        })?;
        for (index, group) in groups.iter().take(shown).enumerate() {
            f.write_markup(markup! {
                {CompactQueryProfileRecord {
                    index: index + 1,
                    group,
                    working_directory: self.working_directory,
                    cutoffs,
                }}
            })?;
        }
        let omitted = groups.len().saturating_sub(shown);
        if omitted > 0 {
            f.write_markup(markup! {
                {Padding::new(RECORD_INDENT)}<Dim>{Count::groups(omitted)}" omitted; use --verbose for every source record."</Dim>"\n"
            })?;
        }
        f.write_markup(markup! {{HARD_LINE}})
    }
}

struct CompactQueryProfileRecord<'a> {
    index: usize,
    group: &'a QueryGroup<'a>,
    working_directory: Option<&'a Utf8Path>,
    cutoffs: TimingCutoffs,
}

impl Display for CompactQueryProfileRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let group = self.group;
        f.write_markup(markup! {
            {self.index}". "<Info>{group.key.kind.label()}</Info><Dim>" / "</Dim>{group.key.implementation.symbol()}"\n"
            {CompactTimingMetricsDisplay::new(group.timing.metrics(), self.cutoffs)}
        })?;
        if let Some(consumer) = group.top_consumer() {
            f.write_markup(markup! {
                {Padding::new(RECORD_INDENT)}<Dim>"top consumer: "</Dim>{QueryConsumer(consumer)}"\n"
            })?;
        }
        f.write_markup(markup! {
            {Padding::new(RECORD_INDENT)}<Dim>"hottest source: "</Dim>{SourceLocation::new(&group.hottest.location, self.working_directory)}"\n"
        })
    }
}

struct QueryConsumer(QueryConsumerKey);

impl Display for QueryConsumer {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_markup(markup! {
            {self.0.request.label()}<Dim>" <- "</Dim>{self.0.caller.group()}"/"{self.0.caller.name()}
        })
    }
}

struct WholeModuleGroup<'a> {
    reason: TypeInferenceWholeModuleReason,
    timing: TimingAccumulator,
    hottest: &'a TypeInferenceWholeModuleProfile,
    modules_max: u64,
    type_slots_max: u64,
    expression_slots_max: u64,
    binding_slots_max: u64,
    cycle_recoveries: u32,
}

impl<'a> WholeModuleGroup<'a> {
    fn new(profile: &'a TypeInferenceWholeModuleProfile) -> Self {
        let mut group = Self {
            reason: profile.reason,
            timing: TimingAccumulator::default(),
            hottest: profile,
            modules_max: 0,
            type_slots_max: 0,
            expression_slots_max: 0,
            binding_slots_max: 0,
            cycle_recoveries: 0,
        };
        group.add(profile);
        group
    }

    fn add(&mut self, profile: &'a TypeInferenceWholeModuleProfile) {
        self.timing.add(TimingMetrics::from(profile));
        self.modules_max = self.modules_max.max(profile.modules.max);
        self.type_slots_max = self.type_slots_max.max(profile.type_slots.max);
        self.expression_slots_max = self.expression_slots_max.max(profile.expression_slots.max);
        self.binding_slots_max = self.binding_slots_max.max(profile.binding_slots.max);
        self.cycle_recoveries = self
            .cycle_recoveries
            .saturating_add(profile.cycle_recoveries);
        if profile.trigger.is_hotter_than(
            profile.total,
            profile.max,
            &self.hottest.trigger,
            self.hottest.total,
            self.hottest.max,
        ) {
            self.hottest = profile;
        }
    }
}

struct CompactWholeModuleProfiles<'a> {
    profiles: &'a [TypeInferenceWholeModuleProfile],
    working_directory: Option<&'a Utf8Path>,
}

impl Display for CompactWholeModuleProfiles<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_markup(markup! {
            <Emphasis>"Whole-module widening"</Emphasis>"\n"
        })?;
        if self.profiles.is_empty() {
            f.write_markup(markup! {
                {Padding::new(RECORD_INDENT)}"No widening to complete module tables was recorded."
                {HARD_LINE}
            })?;
            return Ok(());
        }

        let groups = aggregate_whole_module_profiles(self.profiles);
        let cutoffs = TimingCutoffs::new(groups.iter().map(|group| group.timing.metrics()));
        for group in &groups {
            f.write_markup(markup! {
                {CompactWholeModuleProfileRecord {
                    group,
                    working_directory: self.working_directory,
                    cutoffs,
                }}
            })?;
        }
        f.write_markup(markup! {
            {Padding::new(RECORD_INDENT)}<Dim>"Breadth values are independent maxima across completed runs."</Dim>
            {HARD_LINE}
        })
    }
}

struct CompactWholeModuleProfileRecord<'a> {
    group: &'a WholeModuleGroup<'a>,
    working_directory: Option<&'a Utf8Path>,
    cutoffs: TimingCutoffs,
}

impl Display for CompactWholeModuleProfileRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let group = self.group;
        f.write_markup(markup! {
            <Info>{group.reason.label()}</Info>"\n"
            {CompactTimingMetricsDisplay::new(group.timing.metrics(), self.cutoffs)}
            {Padding::new(RECORD_INDENT)}<Dim>"maximum breadth: "</Dim>{group.modules_max}<Dim>" modules, slots T/E/B "</Dim>{group.type_slots_max}"/"{group.expression_slots_max}"/"{group.binding_slots_max}"\n"
        })?;
        if group.cycle_recoveries > 0 {
            f.write_markup(markup! {
                {Padding::new(RECORD_INDENT)}<Dim>"cycle recoveries: "</Dim>{group.cycle_recoveries}"\n"
            })?;
        }
        f.write_markup(markup! {
            {Padding::new(RECORD_INDENT)}<Dim>"hottest transition: "</Dim>{SourceLocation::new(&group.hottest.root, self.working_directory)}<Dim>" -> "</Dim>{SourceLocation::new(&group.hottest.trigger, self.working_directory)}"\n"
        })
    }
}

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
struct TimingAccumulator {
    total: Duration,
    max: Duration,
    completed: u32,
    min: Option<Duration>,
    aborted: u32,
}

impl TimingAccumulator {
    fn add(&mut self, metrics: TimingMetrics) {
        self.total = self.total.saturating_add(metrics.total);
        self.completed = self.completed.saturating_add(metrics.completed);
        self.aborted = self.aborted.saturating_add(metrics.aborted);
        if metrics.completed > 0 {
            self.min = Some(
                self.min
                    .map_or(metrics.min, |current| current.min(metrics.min)),
            );
            self.max = self.max.max(metrics.max);
        }
    }

    fn metrics(self) -> TimingMetrics {
        TimingMetrics {
            total: self.total,
            average: if self.completed == 0 {
                Duration::ZERO
            } else {
                self.total / self.completed
            },
            min: self.min.unwrap_or_default(),
            max: self.max,
            completed: self.completed,
            aborted: self.aborted,
        }
    }
}

struct CompactTimingMetricsDisplay {
    metrics: TimingMetrics,
    cutoffs: TimingCutoffs,
}

impl CompactTimingMetricsDisplay {
    const fn new(metrics: TimingMetrics, cutoffs: TimingCutoffs) -> Self {
        Self { metrics, cutoffs }
    }
}

impl Display for CompactTimingMetricsDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_markup(markup! {
            {Padding::new(RECORD_INDENT)}<Dim>"time: total "</Dim>{HighlightedDuration::new(self.metrics.total, self.cutoffs.total)}
            <Dim>", "</Dim>{self.metrics.completed}<Dim>" runs, average "</Dim>{HighlightedDuration::new(self.metrics.average, self.cutoffs.average)}
            <Dim>", max "</Dim>{HighlightedDuration::new(self.metrics.max, self.cutoffs.max)}
        })?;
        if self.metrics.aborted > 0 {
            f.write_markup(markup! {
                <Dim>", "</Dim>{self.metrics.aborted}<Dim>" aborted"</Dim>
            })?;
        }
        f.write_str("\n")
    }
}

fn aggregate_request_profiles(profiles: &[TypeInferenceRequestProfile]) -> Vec<RequestGroup<'_>> {
    let mut groups = BTreeMap::<RequestGroupKey, RequestGroup>::new();
    for profile in profiles {
        let key = RequestGroupKey {
            metadata: profile.metadata,
            caller: profile.caller,
            implementation: profile.implementation,
        };
        groups
            .entry(key)
            .and_modify(|group| group.add(profile))
            .or_insert_with(|| RequestGroup::new(profile));
    }
    let mut groups = groups.into_values().collect::<Vec<_>>();
    groups.sort_by(|left, right| {
        right
            .timing
            .cmp(&left.timing)
            .then_with(|| left.key.cmp(&right.key))
    });
    groups
}

fn aggregate_query_profiles(profiles: &[TypeInferenceQueryProfile]) -> Vec<QueryGroup<'_>> {
    let mut groups = BTreeMap::<QueryGroupKey, QueryGroup>::new();
    for profile in profiles {
        let key = QueryGroupKey {
            kind: profile.kind,
            implementation: profile.implementation,
        };
        groups
            .entry(key)
            .and_modify(|group| group.add(profile))
            .or_insert_with(|| QueryGroup::new(profile));
    }
    let mut groups = groups.into_values().collect::<Vec<_>>();
    groups.sort_by(|left, right| {
        right
            .timing
            .cmp(&left.timing)
            .then_with(|| left.key.cmp(&right.key))
    });
    groups
}

fn aggregate_whole_module_profiles(
    profiles: &[TypeInferenceWholeModuleProfile],
) -> Vec<WholeModuleGroup<'_>> {
    let mut groups = BTreeMap::<TypeInferenceWholeModuleReason, WholeModuleGroup>::new();
    for profile in profiles {
        groups
            .entry(profile.reason)
            .and_modify(|group| group.add(profile))
            .or_insert_with(|| WholeModuleGroup::new(profile));
    }
    let mut groups = groups.into_values().collect::<Vec<_>>();
    groups.sort_by(|left, right| {
        right
            .timing
            .cmp(&left.timing)
            .then_with(|| left.reason.cmp(&right.reason))
    });
    groups
}

impl TypeInferenceProfileLocation {
    /// Returns whether a record at this location ranks ahead of another record.
    ///
    /// Total time takes precedence over maximum time. Equal timings use source
    /// order so repeated reports choose the same representative location.
    fn is_hotter_than(
        &self,
        total: Duration,
        max: Duration,
        other: &Self,
        other_total: Duration,
        other_max: Duration,
    ) -> bool {
        total > other_total
            || (total == other_total && (max > other_max || (max == other_max && self < other)))
    }
}

struct Count {
    count: usize,
    singular: &'static str,
    plural: &'static str,
}

impl Count {
    const fn source_records(count: usize) -> Self {
        Self {
            count,
            singular: "source record",
            plural: "source records",
        }
    }

    const fn groups(count: usize) -> Self {
        Self {
            count,
            singular: "group",
            plural: "groups",
        }
    }
}

impl Display for Count {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_fmt(format_args!(
            "{} {}",
            self.count,
            if self.count == 1 {
                self.singular
            } else {
                self.plural
            }
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use biome_console::fmt::{Display, Formatter, Termcolor};
    use biome_console::markup;
    use biome_diagnostics::termcolor::NoColor;
    use biome_rowan::TextRange;

    use super::CompactTypeInferenceProfile;
    use crate::type_inference::profiling::{
        RequestMetadata, TypeInferenceBreadthProfile, TypeInferenceLocationAttribution,
        TypeInferenceProfileLocation, TypeInferenceProfileSnapshot, TypeInferenceQueryProfile,
        TypeInferenceRequestProfile, TypeInferenceWholeModuleProfile,
    };
    use crate::type_inference::{
        ArrayOfPromisesClassificationRequest, FunctionReturnTypeRequest, MemberReturnTypeRequest,
        NormalizedBindingTypeRequest, NormalizedExpressionTypeRequest,
        PromiseClassificationRequest, TypeInferenceCaller, TypeInferenceCodeReference,
        TypeInferenceQueryKind, TypeInferenceWholeModuleReason,
    };

    const IMPLEMENTATION: TypeInferenceCodeReference = TypeInferenceCodeReference::new(
        "crates/biome_module_graph/src/test.rs",
        1,
        "test_implementation",
    );

    fn render(display: impl Display) -> String {
        let mut buffer = Vec::new();
        let mut writer = Termcolor(NoColor::new(&mut buffer));
        let mut f = Formatter::new(&mut writer);
        f.write_markup(markup! {{ display }}).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    fn location(path: &str, start: u32) -> TypeInferenceProfileLocation {
        TypeInferenceProfileLocation {
            path: path.into(),
            range: Some(TextRange::new(start.into(), (start + 1).into())),
            attribution: TypeInferenceLocationAttribution::Exact,
        }
    }

    fn request_profile(
        metadata: RequestMetadata,
        caller: TypeInferenceCaller,
        path: &str,
        total: Duration,
        completed: u32,
    ) -> TypeInferenceRequestProfile {
        TypeInferenceRequestProfile {
            caller,
            metadata,
            location: location(path, 1),
            implementation: IMPLEMENTATION,
            completed,
            aborted: 0,
            total,
            average: total / completed,
            min: total / completed,
            max: total,
        }
    }

    #[test]
    fn groups_request_origins_and_uses_a_weighted_average() {
        let caller = TypeInferenceCaller::new("nursery", "noFloatingPromises");
        let snapshot = TypeInferenceProfileSnapshot {
            requests: vec![
                request_profile(
                    RequestMetadata::of::<PromiseClassificationRequest>(),
                    caller,
                    "src/first.ts",
                    Duration::from_millis(2),
                    2,
                ),
                request_profile(
                    RequestMetadata::of::<PromiseClassificationRequest>(),
                    caller,
                    "src/hottest.ts",
                    Duration::from_millis(6),
                    3,
                ),
            ],
            ..TypeInferenceProfileSnapshot::default()
        };

        let output = render(CompactTypeInferenceProfile::new(&snapshot, None, "test"));

        assert!(output.contains("Requests by consumer (top 1 of 1 group; 2 source records)"));
        assert!(output.contains("time: total 8.000ms, 5 runs, average 1.600ms"));
        assert!(output.contains("hottest origin: src/hottest.ts:1..2"));
        assert!(!output.contains("Code references"));
    }

    #[test]
    fn request_groups_are_bounded() {
        let caller = TypeInferenceCaller::new("nursery", "rule");
        let metadata = [
            RequestMetadata::of::<NormalizedExpressionTypeRequest>(),
            RequestMetadata::of::<NormalizedBindingTypeRequest>(),
            RequestMetadata::of::<PromiseClassificationRequest>(),
            RequestMetadata::of::<ArrayOfPromisesClassificationRequest>(),
            RequestMetadata::of::<FunctionReturnTypeRequest>(),
            RequestMetadata::of::<MemberReturnTypeRequest<'static>>(),
        ];
        let snapshot = TypeInferenceProfileSnapshot {
            requests: metadata
                .into_iter()
                .enumerate()
                .map(|(index, metadata)| {
                    request_profile(
                        metadata,
                        caller,
                        "src/file.ts",
                        Duration::from_millis((index + 1) as u64),
                        1,
                    )
                })
                .collect(),
            ..TypeInferenceProfileSnapshot::default()
        };

        let output = render(CompactTypeInferenceProfile::new(&snapshot, None, "test"));

        assert!(output.contains("Requests by consumer (top 5 of 6 groups; 6 source records)"));
        assert!(output.contains("1 group omitted"));
        assert!(!output.contains("Normalized expression type <- nursery/rule"));
    }

    #[test]
    fn query_group_reports_its_largest_consumer() {
        let first_caller = TypeInferenceCaller::new("nursery", "firstRule");
        let second_caller = TypeInferenceCaller::new("nursery", "secondRule");
        let query = |request, caller, path, millis| TypeInferenceQueryProfile {
            kind: TypeInferenceQueryKind::Promises,
            request,
            caller,
            location: location(path, 4),
            implementation: IMPLEMENTATION,
            completed: 1,
            aborted: 0,
            total: Duration::from_millis(millis),
            average: Duration::from_millis(millis),
            min: Duration::from_millis(millis),
            max: Duration::from_millis(millis),
        };
        let snapshot = TypeInferenceProfileSnapshot {
            queries: vec![
                query(
                    RequestMetadata::of::<PromiseClassificationRequest>(),
                    first_caller,
                    "src/first.ts",
                    2,
                ),
                query(
                    RequestMetadata::of::<ArrayOfPromisesClassificationRequest>(),
                    second_caller,
                    "src/second.ts",
                    4,
                ),
            ],
            ..TypeInferenceProfileSnapshot::default()
        };

        let output = render(CompactTypeInferenceProfile::new(&snapshot, None, "test"));

        assert!(output.contains("Promises / test_implementation"));
        assert!(output.contains("time: total 6.000ms, 2 runs, average 3.000ms"));
        assert!(
            output.contains("top consumer: Array of Promises classification <- nursery/secondRule")
        );
        assert!(output.contains("hottest source: src/second.ts:4..5"));
    }

    #[test]
    fn query_groups_are_bounded() {
        let snapshot = TypeInferenceProfileSnapshot {
            queries: (1..=9)
                .map(|line| TypeInferenceQueryProfile {
                    kind: TypeInferenceQueryKind::Promises,
                    request: RequestMetadata::of::<PromiseClassificationRequest>(),
                    caller: TypeInferenceCaller::new("nursery", "rule"),
                    location: location("src/file.ts", line),
                    implementation: TypeInferenceCodeReference::new(
                        "crates/biome_module_graph/src/test.rs",
                        line,
                        "query",
                    ),
                    completed: 1,
                    aborted: 0,
                    total: Duration::from_millis(u64::from(line)),
                    average: Duration::from_millis(u64::from(line)),
                    min: Duration::from_millis(u64::from(line)),
                    max: Duration::from_millis(u64::from(line)),
                })
                .collect(),
            ..TypeInferenceProfileSnapshot::default()
        };

        let output = render(CompactTypeInferenceProfile::new(&snapshot, None, "test"));

        assert!(output.contains("Query bodies (top 8 of 9 groups; 9 source records)"));
        assert!(output.contains("1 group omitted"));
    }

    #[test]
    fn whole_module_groups_report_maximum_breadth_and_hottest_transition() {
        let whole_module =
            |path, millis, modules, cycle_recoveries| TypeInferenceWholeModuleProfile {
                reason: TypeInferenceWholeModuleReason::ImportDepthLimit,
                root: location("src/root.ts", 1),
                trigger: location(path, 2),
                implementation: IMPLEMENTATION,
                completed: 1,
                aborted: 0,
                total: Duration::from_millis(millis),
                average: Duration::from_millis(millis),
                min: Duration::from_millis(millis),
                max: Duration::from_millis(millis),
                modules: TypeInferenceBreadthProfile {
                    average: modules as f64,
                    min: modules,
                    max: modules,
                },
                type_slots: TypeInferenceBreadthProfile {
                    average: 0.0,
                    min: 0,
                    max: modules * 10,
                },
                expression_slots: TypeInferenceBreadthProfile {
                    average: 0.0,
                    min: 0,
                    max: modules * 20,
                },
                binding_slots: TypeInferenceBreadthProfile {
                    average: 0.0,
                    min: 0,
                    max: modules * 5,
                },
                cycle_recoveries,
            };
        let snapshot = TypeInferenceProfileSnapshot {
            whole_module_inferences: vec![
                whole_module("src/first.ts", 2, 3, 1),
                whole_module("src/hottest.ts", 4, 8, 2),
            ],
            ..TypeInferenceProfileSnapshot::default()
        };

        let output = render(CompactTypeInferenceProfile::new(&snapshot, None, "test"));

        assert!(output.contains("Import depth limit"));
        assert!(output.contains("time: total 6.000ms, 2 runs, average 3.000ms"));
        assert!(output.contains("maximum breadth: 8 modules, slots T/E/B 80/160/40"));
        assert!(output.contains("cycle recoveries: 3"));
        assert!(output.contains("hottest transition: src/root.ts:1..2 -> src/hottest.ts:2..3"));
    }
}
