//! Detailed terminal output for type-inference profiles.
//!
//! The report prints every captured source record, a short interpretation of
//! the largest timings, and the Rust locations that produced the records.

use std::collections::BTreeSet;
use std::io;

use biome_console::fmt::{Display, Formatter};
use biome_console::{HARD_LINE, HorizontalLine, Padding, markup};
use camino::Utf8Path;

use super::{
    CapacityWarning, DisplayDuration, HighlightedDuration, RECORD_INDENT, SourceLocation,
    TimingCutoffs, TimingMetrics,
};
use crate::type_inference::TypeInferenceCodeReference;
use crate::type_inference::profiling::{
    TypeInferenceBreadthProfile, TypeInferenceProfileSnapshot, TypeInferenceQueryProfile,
    TypeInferenceRequestProfile, TypeInferenceWholeModuleProfile,
};

pub(super) struct VerboseTypeInferenceProfile<'a> {
    snapshot: &'a TypeInferenceProfileSnapshot,
    working_directory: Option<&'a Utf8Path>,
    version: &'a str,
}

impl<'a> VerboseTypeInferenceProfile<'a> {
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

impl Display for VerboseTypeInferenceProfile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_markup(markup! {
            {HorizontalLine::new(20)}
            <Emphasis>"Type inference profile"</Emphasis>" "<Dim>"(Biome "{self.version}")"</Dim>"\n"
            <Dim>"Times are inclusive; nested query times must not be added together."</Dim>"\n"
            <Dim>"Ranges are zero-based, half-open UTF-8 byte offsets."</Dim>"\n"
            <Dim>"Timings exclude aborted work and profiler aggregation. In sections with at least ten records, the highest 10% are highlighted."</Dim>"\n"
            <Dim>"Paths may reveal project structure."</Dim>
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
            {RequestProfiles {
                profiles: &self.snapshot.requests,
                working_directory: self.working_directory,
            }}
            {QueryProfiles {
                profiles: &self.snapshot.queries,
                working_directory: self.working_directory,
            }}
            {WholeModuleProfiles {
                profiles: &self.snapshot.whole_module_inferences,
                working_directory: self.working_directory,
            }}
            {ProfileInterpretation {
                snapshot: self.snapshot,
                working_directory: self.working_directory,
            }}
            {CodeReferences(self.snapshot)}
            {CapacityWarning(self.snapshot)}
        })
    }
}

struct RequestProfiles<'a> {
    profiles: &'a [TypeInferenceRequestProfile],
    working_directory: Option<&'a Utf8Path>,
}

impl Display for RequestProfiles<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        if self.profiles.is_empty() {
            return Ok(());
        }

        f.write_markup(markup! {
            <Emphasis>"Hot request origins"</Emphasis>" "<Dim>"(sorted by cumulative request time)"</Dim>"\n"
        })?;
        let cutoffs = TimingCutoffs::new(self.profiles.iter().map(TimingMetrics::from));
        for profile in self.profiles.iter() {
            f.write_markup(markup! {
                {RequestProfileRecord {
                    profile,
                    working_directory: self.working_directory,
                    cutoffs,
                }}
            })?;
        }
        f.write_markup(markup! {
            {HARD_LINE}
        })
    }
}

struct RequestProfileRecord<'a> {
    profile: &'a TypeInferenceRequestProfile,
    working_directory: Option<&'a Utf8Path>,
    cutoffs: TimingCutoffs,
}

impl Display for RequestProfileRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let profile = self.profile;
        f.write_markup(markup! {
            <Info>{profile.metadata.label()}</Info>"\n"
            {TimingMetricsDisplay::new(TimingMetrics::from(profile), self.cutoffs)}
            {Padding::new(RECORD_INDENT)}<Dim>"source: "</Dim>{SourceLocation::new(&profile.location, self.working_directory)}"\n"
            {Padding::new(RECORD_INDENT)}<Dim>"caller: "</Dim>{profile.caller.group()}"/"{profile.caller.name()}"\n"
        })
    }
}

struct QueryProfiles<'a> {
    profiles: &'a [TypeInferenceQueryProfile],
    working_directory: Option<&'a Utf8Path>,
}

impl Display for QueryProfiles<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        if self.profiles.is_empty() {
            return Ok(());
        }

        f.write_markup(markup! {
            <Emphasis>"Hot inference queries"</Emphasis>" "<Dim>"(tracked query bodies)"</Dim>"\n"
        })?;
        let cutoffs = TimingCutoffs::new(self.profiles.iter().map(TimingMetrics::from));
        for profile in self.profiles.iter() {
            f.write_markup(markup! {
                {QueryProfileRecord {
                    profile,
                    working_directory: self.working_directory,
                    cutoffs,
                }}
            })?;
        }
        f.write_markup(markup! {
            {HARD_LINE}
        })
    }
}

struct QueryProfileRecord<'a> {
    profile: &'a TypeInferenceQueryProfile,
    working_directory: Option<&'a Utf8Path>,
    cutoffs: TimingCutoffs,
}

impl Display for QueryProfileRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let profile = self.profile;
        f.write_markup(markup! {
            <Info>{profile.kind.label()}</Info><Dim>" / "</Dim>{profile.implementation.symbol()}"\n"
            {TimingMetricsDisplay::new(TimingMetrics::from(profile), self.cutoffs)}
            {Padding::new(RECORD_INDENT)}<Dim>"source: "</Dim>{SourceLocation::new(&profile.location, self.working_directory)}<Dim>" ("{profile.location.attribution}")"</Dim>"\n"
            {Padding::new(RECORD_INDENT)}<Dim>"request: "</Dim>{profile.request.label()}"\n"
            {Padding::new(RECORD_INDENT)}<Dim>"caller: "</Dim>{profile.caller.group()}"/"{profile.caller.name()}"\n"
        })
    }
}

struct WholeModuleProfiles<'a> {
    profiles: &'a [TypeInferenceWholeModuleProfile],
    working_directory: Option<&'a Utf8Path>,
}

impl Display for WholeModuleProfiles<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_markup(markup! {
            <Emphasis>"Whole-module inference"</Emphasis>"\n"
        })?;
        if self.profiles.is_empty() {
            f.write_markup(markup! {
                {Padding::new(RECORD_INDENT)}"No request required complete module tables."
                {HARD_LINE}
            })?;
            return Ok(());
        }

        let cutoffs = TimingCutoffs::new(self.profiles.iter().map(TimingMetrics::from));
        for profile in self.profiles.iter() {
            f.write_markup(markup! {
                {WholeModuleProfileRecord {
                    profile,
                    working_directory: self.working_directory,
                    cutoffs,
                }}
            })?;
        }
        f.write_markup(markup! {
            {HARD_LINE}
        })
    }
}

struct WholeModuleProfileRecord<'a> {
    profile: &'a TypeInferenceWholeModuleProfile,
    working_directory: Option<&'a Utf8Path>,
    cutoffs: TimingCutoffs,
}

impl Display for WholeModuleProfileRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let profile = self.profile;
        f.write_markup(markup! {
            <Info>{profile.reason.label()}</Info>"\n"
            {TimingMetricsDisplay::new(TimingMetrics::from(profile), self.cutoffs)}
            {Padding::new(RECORD_INDENT)}<Dim>"root: "</Dim>{SourceLocation::new(&profile.root, self.working_directory)}"\n"
            {Padding::new(RECORD_INDENT)}<Dim>"trigger: "</Dim>{SourceLocation::new(&profile.trigger, self.working_directory)}"\n"
            {BreadthMetrics::new("modules", profile.modules)}
            {BreadthMetrics::new("type slots", profile.type_slots)}
            {BreadthMetrics::new("expression slots", profile.expression_slots)}
            {BreadthMetrics::new("binding slots", profile.binding_slots)}
        })?;
        if profile.cycle_recoveries > 0 {
            f.write_markup(markup! {
                {Padding::new(RECORD_INDENT)}<Dim>"cycle recoveries: "</Dim>{profile.cycle_recoveries}"\n"
            })?;
        }
        Ok(())
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
            {Padding::new(RECORD_INDENT)}<Dim>"time: total "</Dim>{HighlightedDuration::new(self.metrics.total, self.cutoffs.total)}
            <Dim>", average "</Dim>{HighlightedDuration::new(self.metrics.average, self.cutoffs.average)}
            <Dim>", min "</Dim>{HighlightedDuration::new(self.metrics.min, self.cutoffs.min)}
            <Dim>", max "</Dim>{HighlightedDuration::new(self.metrics.max, self.cutoffs.max)}"\n"
            {Padding::new(RECORD_INDENT)}<Dim>"runs: "</Dim>{self.metrics.completed}<Dim>" completed, "</Dim>{self.metrics.aborted}<Dim>" aborted"</Dim>"\n"
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
            {Padding::new(RECORD_INDENT)}<Dim>{self.label}": min "</Dim>{self.metrics.min}
            <Dim>", average "</Dim>{DisplayAverage(self.metrics.average)}
            <Dim>", max "</Dim>{self.metrics.max}"\n"
        })
    }
}

struct DisplayAverage(f64);

impl Display for DisplayAverage {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_fmt(format_args!("{:.1}", self.0))
    }
}

struct ProfileInterpretation<'a> {
    snapshot: &'a TypeInferenceProfileSnapshot,
    working_directory: Option<&'a Utf8Path>,
}

impl Display for ProfileInterpretation<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_markup(markup! { <Emphasis>"Interpretation"</Emphasis>"\n" })?;
        if let Some(request) = self.snapshot.requests.first() {
            f.write_markup(markup! {
                {Padding::new(RECORD_INDENT)}"The largest cumulative request was "{request.metadata.label()}" at "
                {SourceLocation::new(&request.location, self.working_directory)}" ("
                {DisplayDuration(request.total)}" total, "{DisplayDuration(request.max)}" max).\n"
            })?;
        }
        if let Some(query) = self.snapshot.queries.first() {
            f.write_markup(markup! {
                {Padding::new(RECORD_INDENT)}"The largest inference query was "{query.implementation.symbol()}
                " in "{query.kind.label()}" ("{DisplayDuration(query.total)}" total, "
                {DisplayDuration(query.max)}" max).\n"
            })?;
        }
        if let Some(whole_module) = self.snapshot.whole_module_inferences.first() {
            f.write_markup(markup! {
                {Padding::new(RECORD_INDENT)}"Complete module tables were inferred because of "
                {whole_module.reason.label()}" at "
                {SourceLocation::new(&whole_module.trigger, self.working_directory)}
                ". The widest run covered "{whole_module.modules.max}" modules.\n"
            })?;
        } else {
            f.write_markup(markup! {
                {Padding::new(RECORD_INDENT)}"No request required complete module tables.\n"
            })?;
        }
        f.write_markup(markup! {
            {Padding::new(RECORD_INDENT)}"A high max with a low min indicates an outlier; a high min indicates consistently expensive work."
            {HARD_LINE}
        })
    }
}

struct CodeReferences<'a>(&'a TypeInferenceProfileSnapshot);

impl Display for CodeReferences<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let mut references = BTreeSet::new();
        for profile in self.0.requests.iter() {
            references.insert((profile.metadata.id(), profile.implementation));
        }
        for profile in self.0.queries.iter() {
            references.insert((profile.kind.id(), profile.implementation));
        }
        for profile in self.0.whole_module_inferences.iter() {
            references.insert((profile.reason.id(), profile.implementation));
        }
        if references.is_empty() {
            return Ok(());
        }

        f.write_markup(markup! { <Emphasis>"Code references"</Emphasis>"\n" })?;
        for (id, reference) in references {
            f.write_markup(markup! {
                {CodeReference { id, reference }}
            })?;
        }
        Ok(())
    }
}

struct CodeReference {
    id: &'static str,
    reference: TypeInferenceCodeReference,
}

impl CodeReference {
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
            {Padding::new(RECORD_INDENT)}{self.id}"\n"
            {Padding::new(RECORD_INDENT * 2)}{self.repository_path()}":"{self.reference.line()}
            " ("{self.reference.symbol()}")\n"
        })
    }
}
