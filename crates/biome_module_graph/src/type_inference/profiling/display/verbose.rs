//! Detailed diagnostic advice for type-inference profiles.

use std::io;

use biome_console::fmt::{Display, Formatter};
use biome_console::markup;
use biome_diagnostics::{Advices, LogCategory, Visit};
use camino::Utf8Path;

use super::compact::{FileProfiles, FileTitle, ProfileSummary, collect_files};
use super::{
    HighlightedDuration, QUERY_LIMIT, REQUEST_LIMIT, SourceLocation, SourceRange, TimingCutoffs,
    TimingMetrics, WHOLE_MODULE_LIMIT, record_capacity_warning,
};
use crate::type_inference::TypeInferenceCodeReference;
use crate::type_inference::profiling::{
    TypeInferenceBreadthProfile, TypeInferenceProfileSnapshot, TypeInferenceQueryProfile,
    TypeInferenceRequestProfile, TypeInferenceWholeModuleProfile,
};

pub(super) struct VerboseTypeInferenceProfile<'a> {
    snapshot: &'a TypeInferenceProfileSnapshot,
    working_directory: Option<&'a Utf8Path>,
}

impl<'a> VerboseTypeInferenceProfile<'a> {
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

impl Advices for VerboseTypeInferenceProfile<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        visitor.record_log(
            LogCategory::None,
            &ProfileSummary {
                snapshot: self.snapshot,
                verbose: true,
            },
        )?;

        if self.snapshot.is_empty() {
            visitor.record_log(
                LogCategory::None,
                &"No type-inference requests or queries were recorded.",
            )?;
            return record_capacity_warning(visitor, self.snapshot);
        }

        for file in collect_files(self.snapshot) {
            visitor.record_group(
                &FileTitle::new(&file, self.working_directory),
                &VerboseFileAdvice {
                    file: &file,
                    working_directory: self.working_directory,
                },
            )?;
        }

        record_capacity_warning(visitor, self.snapshot)
    }
}

struct VerboseFileAdvice<'a> {
    file: &'a FileProfiles<'a>,
    working_directory: Option<&'a Utf8Path>,
}

impl Advices for VerboseFileAdvice<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        if !self.file.requests.is_empty() {
            visitor.record_group(
                &VerboseSectionTitle::new(
                    "Requests",
                    self.file.requests.len().min(REQUEST_LIMIT),
                    self.file.requests.len(),
                    completed_requests(&self.file.requests),
                ),
                &VerboseRequestProfiles {
                    profiles: &self.file.requests,
                },
            )?;
        }

        if !self.file.queries.is_empty() {
            visitor.record_group(
                &VerboseSectionTitle::new(
                    "Queries",
                    self.file.queries.len().min(QUERY_LIMIT),
                    self.file.queries.len(),
                    completed_queries(&self.file.queries),
                ),
                &VerboseQueryProfiles {
                    profiles: &self.file.queries,
                },
            )?;
        }

        let advice = VerboseWholeModuleProfiles {
            profiles: &self.file.whole_modules,
            working_directory: self.working_directory,
        };
        if self.file.whole_modules.is_empty() {
            visitor.record_group(&"Whole-module inference", &advice)
        } else {
            visitor.record_group(
                &VerboseSectionTitle::new(
                    "Whole-module inference",
                    self.file.whole_modules.len().min(WHOLE_MODULE_LIMIT),
                    self.file.whole_modules.len(),
                    completed_whole_modules(&self.file.whole_modules),
                ),
                &advice,
            )
        }
    }
}

fn completed_requests(profiles: &[&TypeInferenceRequestProfile]) -> u64 {
    profiles
        .iter()
        .map(|profile| u64::from(profile.completed))
        .sum()
}

fn completed_queries(profiles: &[&TypeInferenceQueryProfile]) -> u64 {
    profiles
        .iter()
        .map(|profile| u64::from(profile.completed))
        .sum()
}

fn completed_whole_modules(profiles: &[&TypeInferenceWholeModuleProfile]) -> u64 {
    profiles
        .iter()
        .map(|profile| u64::from(profile.completed))
        .sum()
}

struct VerboseSectionTitle {
    label: &'static str,
    shown: usize,
    source_records: usize,
    executions: u64,
}

impl VerboseSectionTitle {
    const fn new(
        label: &'static str,
        shown: usize,
        source_records: usize,
        executions: u64,
    ) -> Self {
        Self {
            label,
            shown,
            source_records,
            executions,
        }
    }
}

impl Display for VerboseSectionTitle {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_markup(markup! {
            {self.label}" (top "{self.shown}" of "{SourceRecordCount(self.source_records)}"; "
            {ExecutionCount(self.executions)}")"
        })
    }
}

struct SourceRecordCount(usize);

impl Display for SourceRecordCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_fmt(format_args!(
            "{} source {}",
            self.0,
            if self.0 == 1 { "record" } else { "records" }
        ))
    }
}

struct ExecutionCount(u64);

impl Display for ExecutionCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_fmt(format_args!(
            "{} {}",
            self.0,
            if self.0 == 1 {
                "execution"
            } else {
                "executions"
            }
        ))
    }
}

struct VerboseRequestProfiles<'a> {
    profiles: &'a [&'a TypeInferenceRequestProfile],
}

impl Advices for VerboseRequestProfiles<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let cutoffs = TimingCutoffs::new(
            self.profiles
                .iter()
                .map(|profile| TimingMetrics::from(*profile)),
        );
        for (index, profile) in self.profiles.iter().take(REQUEST_LIMIT).enumerate() {
            visitor.record_log(
                LogCategory::None,
                &RequestProfileRecord {
                    index: index + 1,
                    profile,
                    cutoffs,
                },
            )?;
        }
        record_omitted(
            visitor,
            self.profiles.len(),
            REQUEST_LIMIT,
            "request source record",
            "request source records",
        )
    }
}

struct RequestProfileRecord<'a> {
    index: usize,
    profile: &'a TypeInferenceRequestProfile,
    cutoffs: TimingCutoffs,
}

impl Display for RequestProfileRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let profile = self.profile;
        f.write_markup(markup! {
            {self.index}". "<Success>{profile.metadata.label()}</Success>"\n"
            <Info>"Range: "</Info>{SourceRange(profile.location.range)}"\n"
            <Info>"Caller: "</Info>{profile.caller.group()}"/"{profile.caller.name()}"\n"
            {TimingMetricsDisplay::new(TimingMetrics::from(profile), self.cutoffs)}"\n"
            {CodeReference::new(profile.implementation)}
        })
    }
}

struct VerboseQueryProfiles<'a> {
    profiles: &'a [&'a TypeInferenceQueryProfile],
}

impl Advices for VerboseQueryProfiles<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let cutoffs = TimingCutoffs::new(
            self.profiles
                .iter()
                .map(|profile| TimingMetrics::from(*profile)),
        );
        for (index, profile) in self.profiles.iter().take(QUERY_LIMIT).enumerate() {
            visitor.record_log(
                LogCategory::None,
                &QueryProfileRecord {
                    index: index + 1,
                    profile,
                    cutoffs,
                },
            )?;
        }
        record_omitted(
            visitor,
            self.profiles.len(),
            QUERY_LIMIT,
            "query source record",
            "query source records",
        )
    }
}

struct QueryProfileRecord<'a> {
    index: usize,
    profile: &'a TypeInferenceQueryProfile,
    cutoffs: TimingCutoffs,
}

impl Display for QueryProfileRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let profile = self.profile;
        f.write_markup(markup! {
            {self.index}". "<Success>{profile.kind.label()}</Success><Dim>" / "</Dim>
            {profile.implementation.symbol()}"\n"
            <Info>"Range: "</Info>{SourceRange(profile.location.range)}"\n"
            <Info>"Attribution: "</Info>{profile.location.attribution}"\n"
            <Info>"Request: "</Info>{profile.request.label()}"\n"
            <Info>"Caller: "</Info>{profile.caller.group()}"/"{profile.caller.name()}"\n"
            {TimingMetricsDisplay::new(TimingMetrics::from(profile), self.cutoffs)}"\n"
            {CodeReference::new(profile.implementation)}
        })
    }
}

struct VerboseWholeModuleProfiles<'a> {
    profiles: &'a [&'a TypeInferenceWholeModuleProfile],
    working_directory: Option<&'a Utf8Path>,
}

impl Advices for VerboseWholeModuleProfiles<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        if self.profiles.is_empty() {
            return visitor.record_log(
                LogCategory::None,
                &"No requests triggered whole-module inference.",
            );
        }

        let cutoffs = TimingCutoffs::new(
            self.profiles
                .iter()
                .map(|profile| TimingMetrics::from(*profile)),
        );
        for (index, profile) in self.profiles.iter().take(WHOLE_MODULE_LIMIT).enumerate() {
            visitor.record_log(
                LogCategory::None,
                &WholeModuleProfileRecord {
                    index: index + 1,
                    profile,
                    working_directory: self.working_directory,
                    cutoffs,
                },
            )?;
        }
        record_omitted(
            visitor,
            self.profiles.len(),
            WHOLE_MODULE_LIMIT,
            "whole-module source record",
            "whole-module source records",
        )
    }
}

struct WholeModuleProfileRecord<'a> {
    index: usize,
    profile: &'a TypeInferenceWholeModuleProfile,
    working_directory: Option<&'a Utf8Path>,
    cutoffs: TimingCutoffs,
}

impl Display for WholeModuleProfileRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let profile = self.profile;
        f.write_markup(markup! {
            {self.index}". "<Success>{profile.reason.label()}</Success>"\n"
            <Info>"Root range: "</Info>{SourceRange(profile.root.range)}"\n"
            <Info>"Trigger: "</Info>{SourceLocation::new(&profile.trigger, self.working_directory)}"\n"
            {TimingMetricsDisplay::new(TimingMetrics::from(profile), self.cutoffs)}"\n"
            {BreadthMetrics::new("Modules", profile.modules)}"\n"
            {BreadthMetrics::new("Type slots", profile.type_slots)}"\n"
            {BreadthMetrics::new("Expression slots", profile.expression_slots)}"\n"
            {BreadthMetrics::new("Binding slots", profile.binding_slots)}"\n"
        })?;
        if profile.cycle_recoveries > 0 {
            f.write_markup(markup! {
                <Info>"Cycle recoveries: "</Info>{profile.cycle_recoveries}"\n"
            })?;
        }
        CodeReference::new(profile.implementation).fmt(f)
    }
}

struct TimingMetricsDisplay {
    metrics: TimingMetrics,
    cutoffs: TimingCutoffs,
}

impl TimingMetricsDisplay {
    const fn new(metrics: TimingMetrics, cutoffs: TimingCutoffs) -> Self {
        Self { metrics, cutoffs }
    }
}

impl Display for TimingMetricsDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_markup(markup! {
            <Info>"Total "</Info>{HighlightedDuration::new(self.metrics.total, self.cutoffs.total)}
            <Info>", Average "</Info>{HighlightedDuration::new(self.metrics.average, self.cutoffs.average)}
            <Info>", Min "</Info>{HighlightedDuration::new(self.metrics.min, self.cutoffs.min)}
            <Info>", Max "</Info>{HighlightedDuration::new(self.metrics.max, self.cutoffs.max)}"\n"
            <Info>"Executions: "</Info>{self.metrics.completed}<Info>" completed, "</Info>
            {self.metrics.aborted}<Info>" aborted"</Info>
        })
    }
}

struct BreadthMetrics {
    label: &'static str,
    metrics: TypeInferenceBreadthProfile,
}

impl BreadthMetrics {
    const fn new(label: &'static str, metrics: TypeInferenceBreadthProfile) -> Self {
        Self { label, metrics }
    }
}

impl Display for BreadthMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_markup(markup! {
            <Info>{self.label}": Min "</Info>{self.metrics.min}
            <Info>", Average "</Info>{DisplayAverage(self.metrics.average)}
            <Info>", Max "</Info>{self.metrics.max}
        })
    }
}

struct DisplayAverage(f64);

impl Display for DisplayAverage {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_fmt(format_args!("{:.1}", self.0))
    }
}

struct CodeReference {
    reference: TypeInferenceCodeReference,
}

impl CodeReference {
    const fn new(reference: TypeInferenceCodeReference) -> Self {
        Self { reference }
    }

    fn repository_path(&self) -> &'static str {
        let file = self.reference.file();
        if let Some(index) = file.find("crates/").or_else(|| file.find("crates\\")) {
            &file[index..]
        } else {
            Utf8Path::new(file).file_name().unwrap_or("<unknown>")
        }
    }
}

impl Display for CodeReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_markup(markup! {
            <Info>"Implementation: "</Info>{self.repository_path()}":"{self.reference.line()}"\n"
            <Info>"Symbol: "</Info>{self.reference.symbol()}
        })
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
