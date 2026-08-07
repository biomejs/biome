//! Grouped diagnostic advice for type-inference profiles.

use super::{
    DisplayDuration, FILE_LIMIT, HighlightedDuration, QUERY_LIMIT, REQUEST_LIMIT, SourceLocation,
    SourcePath, SourceRange, TimingCutoffs, TimingMetrics, WHOLE_MODULE_LIMIT,
    record_capacity_warning,
};
use crate::type_inference::profiling::{
    RequestMetadata, TypeInferenceProfileSnapshot, TypeInferenceQueryProfile,
    TypeInferenceRequestProfile, TypeInferenceWholeModuleProfile,
};
use crate::type_inference::{
    TypeInferenceCaller, TypeInferenceCodeReference, TypeInferenceQueryKind,
    TypeInferenceWholeModuleReason,
};
use biome_console::fmt::{Display, Formatter};
use biome_console::markup;
use biome_diagnostics::{Advices, LogCategory, Visit};
use camino::Utf8Path;
use std::collections::BTreeMap;
use std::io;
use std::time::Duration;

pub(super) struct CompactTypeInferenceProfile<'a> {
    snapshot: &'a TypeInferenceProfileSnapshot,
    working_directory: Option<&'a Utf8Path>,
}

impl<'a> CompactTypeInferenceProfile<'a> {
    pub(super) const fn new(
        snapshot: &'a TypeInferenceProfileSnapshot,
        working_directory: Option<&'a Utf8Path>,
    ) -> Self {
        Self {
            snapshot,
            working_directory,
        }
    }
}

impl Advices for CompactTypeInferenceProfile<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        visitor.record_log(
            LogCategory::None,
            &ProfileSummary {
                snapshot: self.snapshot,
                verbose: false,
            },
        )?;

        if self.snapshot.is_empty() {
            visitor.record_log(
                LogCategory::None,
                &"No type-inference requests or queries were recorded.",
            )?;
            return record_capacity_warning(visitor, self.snapshot);
        }

        let files = collect_files(self.snapshot);
        for file in files.iter().take(FILE_LIMIT) {
            visitor.record_group(
                &FileTitle::new(file, self.working_directory),
                &CompactFileAdvice {
                    file,
                    working_directory: self.working_directory,
                },
            )?;
        }
        record_omitted(visitor, files.len(), FILE_LIMIT, "file", "files")?;

        record_capacity_warning(visitor, self.snapshot)?;

        visitor.record_log(LogCategory::Info, &markup! {
            "To show all the information, use the "<Emphasis>"--verbose"</Emphasis>" option. The output might be very verbose, so it's advised to analyze a single file."
        })?;

        visitor
            .record_command("biome lint --profile-type-inference --verbose ./path/to/file.ts")?;

        Ok(())
    }
}

pub(super) struct ProfileSummary<'a> {
    pub(super) snapshot: &'a TypeInferenceProfileSnapshot,
    pub(super) verbose: bool,
}

impl Display for ProfileSummary<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let request_executions = completed_requests(self.snapshot);
        let query_executions = completed_queries(self.snapshot);
        let whole_module_executions = completed_whole_modules(self.snapshot);
        f.write_markup(markup! {
            "Completed executions: "{request_executions}" requests, "{query_executions}
            " tracked queries, "{whole_module_executions}" whole-module inferences.\n"
        })?;
        if self.verbose {
            f.write_markup(markup! {
                "Showing the top "{REQUEST_LIMIT}" requests, top "{QUERY_LIMIT}" queries, and top "
                {WHOLE_MODULE_LIMIT}" whole-module records per file.\n"
            })?;
        } else {
            f.write_markup(markup! {
                "Showing the "{FILE_LIMIT}" files with the highest cumulative request time.\n"
            })?;
        }
        f.write_str(
            "Inclusive timings are non-additive; the highest 10% are highlighted in sections with at least ten entries.\n",
        )?;
        f.write_str("Ranges are zero-based, half-open UTF-8 byte offsets.")
    }
}

fn completed_requests(snapshot: &TypeInferenceProfileSnapshot) -> u64 {
    snapshot
        .requests
        .iter()
        .map(|profile| u64::from(profile.completed))
        .sum()
}

fn completed_queries(snapshot: &TypeInferenceProfileSnapshot) -> u64 {
    snapshot
        .queries
        .iter()
        .map(|profile| u64::from(profile.completed))
        .sum()
}

fn completed_whole_modules(snapshot: &TypeInferenceProfileSnapshot) -> u64 {
    snapshot
        .whole_module_inferences
        .iter()
        .map(|profile| u64::from(profile.completed))
        .sum()
}

#[derive(Default)]
pub(super) struct FileProfiles<'a> {
    pub(super) path: &'a str,
    pub(super) requests: Vec<&'a TypeInferenceRequestProfile>,
    pub(super) queries: Vec<&'a TypeInferenceQueryProfile>,
    pub(super) whole_modules: Vec<&'a TypeInferenceWholeModuleProfile>,
}

impl FileProfiles<'_> {
    pub(super) fn cumulative_request_time(&self) -> Duration {
        self.requests.iter().fold(Duration::ZERO, |total, profile| {
            total.saturating_add(profile.total)
        })
    }
}

pub(super) fn collect_files(snapshot: &TypeInferenceProfileSnapshot) -> Vec<FileProfiles<'_>> {
    let mut files = BTreeMap::<&str, FileProfiles>::new();
    for profile in &snapshot.requests {
        files
            .entry(&profile.location.path)
            .or_insert_with(|| FileProfiles {
                path: &profile.location.path,
                ..FileProfiles::default()
            })
            .requests
            .push(profile);
    }
    for profile in &snapshot.queries {
        files
            .entry(&profile.location.path)
            .or_insert_with(|| FileProfiles {
                path: &profile.location.path,
                ..FileProfiles::default()
            })
            .queries
            .push(profile);
    }
    for profile in &snapshot.whole_module_inferences {
        files
            .entry(&profile.root.path)
            .or_insert_with(|| FileProfiles {
                path: &profile.root.path,
                ..FileProfiles::default()
            })
            .whole_modules
            .push(profile);
    }

    let mut files = files.into_values().collect::<Vec<_>>();
    files.sort_by(|left, right| {
        right
            .cumulative_request_time()
            .cmp(&left.cumulative_request_time())
            .then_with(|| left.path.cmp(right.path))
    });
    files
}

pub(super) struct FileTitle<'a> {
    file: &'a FileProfiles<'a>,
    working_directory: Option<&'a Utf8Path>,
}

impl<'a> FileTitle<'a> {
    pub(super) const fn new(
        file: &'a FileProfiles<'a>,
        working_directory: Option<&'a Utf8Path>,
    ) -> Self {
        Self {
            file,
            working_directory,
        }
    }
}

impl Display for FileTitle<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let path = Utf8Path::new(self.file.path);
        let target = if path.is_absolute() {
            format!("file://{path}")
        } else if let Some(working_directory) = self.working_directory {
            format!("file://{}", working_directory.join(path))
        } else {
            self.file.path.to_string()
        };
        f.write_markup(markup! {
            <Hyperlink href={target.as_str()}>
                {SourcePath::new(self.file.path, self.working_directory)}
            </Hyperlink>
            " ("{DisplayDuration(self.file.cumulative_request_time())}")"
        })
    }
}

struct CompactFileAdvice<'a> {
    file: &'a FileProfiles<'a>,
    working_directory: Option<&'a Utf8Path>,
}

impl Advices for CompactFileAdvice<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        if !self.file.requests.is_empty() {
            let groups = aggregate_request_profiles(&self.file.requests);
            visitor.record_group(
                &CompactSectionTitle::new(
                    "Requests",
                    groups.len().min(REQUEST_LIMIT),
                    groups.len(),
                    self.file.requests.len(),
                    completed_request_profiles(&self.file.requests),
                ),
                &CompactRequestProfiles { groups: &groups },
            )?;
        }

        if !self.file.queries.is_empty() {
            let groups = aggregate_query_profiles(&self.file.queries);
            visitor.record_group(
                &CompactSectionTitle::new(
                    "Queries",
                    groups.len().min(QUERY_LIMIT),
                    groups.len(),
                    self.file.queries.len(),
                    completed_query_profiles(&self.file.queries),
                ),
                &CompactQueryProfiles { groups: &groups },
            )?;
        }

        let groups = aggregate_whole_module_profiles(&self.file.whole_modules);
        let advice = CompactWholeModuleProfiles {
            groups: &groups,
            working_directory: self.working_directory,
        };
        if groups.is_empty() {
            visitor.record_group(&"Whole-module inference", &advice)
        } else {
            visitor.record_group(
                &CompactSectionTitle::new(
                    "Whole-module inference",
                    groups.len().min(WHOLE_MODULE_LIMIT),
                    groups.len(),
                    self.file.whole_modules.len(),
                    completed_whole_module_profiles(&self.file.whole_modules),
                ),
                &advice,
            )
        }
    }
}

fn completed_request_profiles(profiles: &[&TypeInferenceRequestProfile]) -> u64 {
    profiles
        .iter()
        .map(|profile| u64::from(profile.completed))
        .sum()
}

fn completed_query_profiles(profiles: &[&TypeInferenceQueryProfile]) -> u64 {
    profiles
        .iter()
        .map(|profile| u64::from(profile.completed))
        .sum()
}

fn completed_whole_module_profiles(profiles: &[&TypeInferenceWholeModuleProfile]) -> u64 {
    profiles
        .iter()
        .map(|profile| u64::from(profile.completed))
        .sum()
}

struct CompactSectionTitle {
    label: &'static str,
    shown: usize,
    groups: usize,
    source_records: usize,
    executions: u64,
}

impl CompactSectionTitle {
    const fn new(
        label: &'static str,
        shown: usize,
        groups: usize,
        source_records: usize,
        executions: u64,
    ) -> Self {
        Self {
            label,
            shown,
            groups,
            source_records,
            executions,
        }
    }
}

impl Display for CompactSectionTitle {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_markup(markup! {
            {self.label}" (top "{self.shown}" of "{Count::groups(self.groups)}"; "
            {Count::source_records(self.source_records)}"; "{Count::executions(self.executions)}")"
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
    groups: &'a [RequestGroup<'a>],
}

impl Advices for CompactRequestProfiles<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let cutoffs = TimingCutoffs::new(self.groups.iter().map(|group| group.timing.metrics()));
        for (index, group) in self.groups.iter().take(REQUEST_LIMIT).enumerate() {
            visitor.record_log(
                LogCategory::None,
                &CompactRequestProfileRecord {
                    index: index + 1,
                    group,
                    cutoffs,
                },
            )?;
        }
        record_omitted(
            visitor,
            self.groups.len(),
            REQUEST_LIMIT,
            "request group",
            "request groups",
        )
    }
}

struct CompactRequestProfileRecord<'a> {
    index: usize,
    group: &'a RequestGroup<'a>,
    cutoffs: TimingCutoffs,
}

impl Display for CompactRequestProfileRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let group = self.group;
        f.write_markup(markup! {
            {self.index}". "<Success>{group.key.metadata.label()}</Success><Dim>" <- "</Dim>
            {group.key.caller.group()}"/"{group.key.caller.name()}"\n"
            {CompactTimingMetricsDisplay::new(group.timing.metrics(), self.cutoffs)}"\n"
            <Info>"Hottest range: "</Info>{SourceRange(group.hottest.location.range)}
        })?;
        Ok(())
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
    groups: &'a [QueryGroup<'a>],
}

impl Advices for CompactQueryProfiles<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let cutoffs = TimingCutoffs::new(self.groups.iter().map(|group| group.timing.metrics()));
        for (index, group) in self.groups.iter().take(QUERY_LIMIT).enumerate() {
            visitor.record_log(
                LogCategory::None,
                &CompactQueryProfileRecord {
                    index: index + 1,
                    group,
                    cutoffs,
                },
            )?;
        }
        record_omitted(
            visitor,
            self.groups.len(),
            QUERY_LIMIT,
            "query group",
            "query groups",
        )
    }
}

struct CompactQueryProfileRecord<'a> {
    index: usize,
    group: &'a QueryGroup<'a>,
    cutoffs: TimingCutoffs,
}

impl Display for CompactQueryProfileRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let group = self.group;
        f.write_markup(markup! {
            {self.index}". "<Success>{group.key.kind.label()}</Success><Dim>" / "</Dim>
            {group.key.implementation.symbol()}"\n"
            {CompactTimingMetricsDisplay::new(group.timing.metrics(), self.cutoffs)}"\n"
        })?;
        if let Some(consumer) = group.top_consumer() {
            f.write_markup(markup! {
                <Info>"Consumer: "</Info>{consumer.request.label()}"\n"
                <Info>"Caller: "</Info>{consumer.caller.group()}"/"{consumer.caller.name()}"\n"
            })?;
        }
        f.write_markup(markup! {
            <Info>"Hottest range: "</Info>{SourceRange(group.hottest.location.range)}
        })?;
        Ok(())
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
    groups: &'a [WholeModuleGroup<'a>],
    working_directory: Option<&'a Utf8Path>,
}

impl Advices for CompactWholeModuleProfiles<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        if self.groups.is_empty() {
            return visitor.record_log(
                LogCategory::None,
                &"No requests triggered whole-module inference.",
            );
        }

        let cutoffs = TimingCutoffs::new(self.groups.iter().map(|group| group.timing.metrics()));
        for (index, group) in self.groups.iter().take(WHOLE_MODULE_LIMIT).enumerate() {
            visitor.record_log(
                LogCategory::None,
                &CompactWholeModuleProfileRecord {
                    index: index + 1,
                    group,
                    working_directory: self.working_directory,
                    cutoffs,
                },
            )?;
        }
        record_omitted(
            visitor,
            self.groups.len(),
            WHOLE_MODULE_LIMIT,
            "whole-module group",
            "whole-module groups",
        )
    }
}

struct CompactWholeModuleProfileRecord<'a> {
    index: usize,
    group: &'a WholeModuleGroup<'a>,
    working_directory: Option<&'a Utf8Path>,
    cutoffs: TimingCutoffs,
}

impl Display for CompactWholeModuleProfileRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let group = self.group;
        f.write_markup(markup! {
            {self.index}". "<Success>{group.reason.label()}</Success>"\n"
            {CompactTimingMetricsDisplay::new(group.timing.metrics(), self.cutoffs)}"\n"
            <Info>"Maximum breadth: "</Info>{group.modules_max}" modules\n"
            <Info>"Maximum slots: "</Info>{group.type_slots_max}" types, "
            {group.expression_slots_max}" expressions, "{group.binding_slots_max}" bindings\n"
        })?;
        if group.cycle_recoveries > 0 {
            f.write_markup(markup! {
                <Info>"Cycle recoveries: "</Info>{group.cycle_recoveries}"\n"
            })?;
        }
        f.write_markup(markup! {
            <Info>"Hottest transition: "</Info>{SourceRange(group.hottest.root.range)}
            <Info>" -> "</Info>{SourceLocation::new(&group.hottest.trigger, self.working_directory)}
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
            <Info>"Total "</Info>{HighlightedDuration::new(self.metrics.total, self.cutoffs.total)}
            <Info>", "</Info>{Count::executions(u64::from(self.metrics.completed))}
            <Info>", Average "</Info>{HighlightedDuration::new(self.metrics.average, self.cutoffs.average)}
            <Info>", Max "</Info>{HighlightedDuration::new(self.metrics.max, self.cutoffs.max)}
        })?;
        if self.metrics.aborted > 0 {
            f.write_markup(markup! {
                <Info>", "</Info>{self.metrics.aborted}<Info>" aborted"</Info>
            })?;
        }
        Ok(())
    }
}

fn aggregate_request_profiles<'a>(
    profiles: &[&'a TypeInferenceRequestProfile],
) -> Vec<RequestGroup<'a>> {
    let mut groups = BTreeMap::<RequestGroupKey, RequestGroup>::new();
    for &profile in profiles {
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

fn aggregate_query_profiles<'a>(profiles: &[&'a TypeInferenceQueryProfile]) -> Vec<QueryGroup<'a>> {
    let mut groups = BTreeMap::<QueryGroupKey, QueryGroup>::new();
    for &profile in profiles {
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

fn aggregate_whole_module_profiles<'a>(
    profiles: &[&'a TypeInferenceWholeModuleProfile],
) -> Vec<WholeModuleGroup<'a>> {
    let mut groups = BTreeMap::<TypeInferenceWholeModuleReason, WholeModuleGroup>::new();
    for &profile in profiles {
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

impl crate::type_inference::profiling::TypeInferenceProfileLocation {
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

fn record_omitted(
    visitor: &mut dyn Visit,
    total: usize,
    limit: usize,
    singular: &'static str,
    plural: &'static str,
) -> io::Result<()> {
    let omitted = total.saturating_sub(limit);
    if omitted > 0 {
        let label = if omitted == 1 { singular } else { plural };
        visitor.record_log(
            LogCategory::None,
            &markup! { <Dim>{omitted}" "{label}" omitted."</Dim> },
        )?;
    }
    Ok(())
}

struct Count {
    count: u64,
    singular: &'static str,
    plural: &'static str,
}

impl Count {
    fn source_records(count: usize) -> Self {
        Self {
            count: count as u64,
            singular: "source record",
            plural: "source records",
        }
    }

    fn groups(count: usize) -> Self {
        Self {
            count: count as u64,
            singular: "group",
            plural: "groups",
        }
    }

    const fn executions(count: u64) -> Self {
        Self {
            count,
            singular: "execution",
            plural: "executions",
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
