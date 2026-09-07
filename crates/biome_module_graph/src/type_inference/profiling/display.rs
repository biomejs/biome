//! Diagnostic output for type-inference profiles.
mod compact;
mod verbose;

use biome_console::fmt::{Display, Formatter};
use biome_console::markup;
use biome_diagnostics::{
    Advices, Category, Diagnostic, DiagnosticTags, LogCategory, Severity, Visit, category,
};
use camino::Utf8Path;
use std::fmt;
use std::io;
use std::time::Duration;

use super::{
    TypeInferenceLocationAttribution, TypeInferenceProfileLocation, TypeInferenceProfileSnapshot,
    TypeInferenceQueryProfile, TypeInferenceRequestProfile, TypeInferenceWholeModuleProfile,
};

const REQUEST_LIMIT: usize = 5;
const QUERY_LIMIT: usize = 8;
const WHOLE_MODULE_LIMIT: usize = 5;
const FILE_LIMIT: usize = 10;

/// Diagnostic for one type-inference profile.
pub struct TypeInferenceProfileDiagnostic<'a> {
    snapshot: &'a TypeInferenceProfileSnapshot,
    working_directory: Option<&'a Utf8Path>,
    version: &'a str,
    verbose: bool,
}

impl<'a> TypeInferenceProfileDiagnostic<'a> {
    pub fn new(
        snapshot: &'a TypeInferenceProfileSnapshot,
        working_directory: Option<&'a Utf8Path>,
        version: &'a str,
    ) -> Self {
        Self {
            snapshot,
            working_directory,
            version,
            verbose: false,
        }
    }

    /// Selects whether source records include detailed metrics and code references.
    pub const fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

impl fmt::Debug for TypeInferenceProfileDiagnostic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypeInferenceProfileDiagnostic")
            .field("version", &self.version)
            .field("verbose", &self.verbose)
            .finish_non_exhaustive()
    }
}

impl Diagnostic for TypeInferenceProfileDiagnostic<'_> {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("reporter/profiler"))
    }

    fn severity(&self) -> Severity {
        Severity::Information
    }

    fn description(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type inference profile (Biome {})", self.version)
    }

    fn tags(&self) -> DiagnosticTags {
        if self.verbose {
            DiagnosticTags::VERBOSE
        } else {
            DiagnosticTags::empty()
        }
    }

    fn message(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_markup(markup! {
            "Type inference profile "<Dim>"(Biome "{self.version}")"</Dim>
        })
    }

    fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        if self.verbose {
            verbose::VerboseTypeInferenceProfile::new(self.snapshot, self.working_directory)
                .record(visitor)
        } else {
            compact::CompactTypeInferenceProfile::new(self.snapshot, self.working_directory)
                .record(visitor)
        }
    }
}

#[derive(Clone, Copy)]
struct TimingMetrics {
    total: Duration,
    average: Duration,
    min: Duration,
    max: Duration,
    completed: u32,
    aborted: u32,
}

impl From<&TypeInferenceRequestProfile> for TimingMetrics {
    fn from(profile: &TypeInferenceRequestProfile) -> Self {
        Self {
            total: profile.total,
            average: profile.average,
            min: profile.min,
            max: profile.max,
            completed: profile.completed,
            aborted: profile.aborted,
        }
    }
}

impl From<&TypeInferenceQueryProfile> for TimingMetrics {
    fn from(profile: &TypeInferenceQueryProfile) -> Self {
        Self {
            total: profile.total,
            average: profile.average,
            min: profile.min,
            max: profile.max,
            completed: profile.completed,
            aborted: profile.aborted,
        }
    }
}

impl From<&TypeInferenceWholeModuleProfile> for TimingMetrics {
    fn from(profile: &TypeInferenceWholeModuleProfile) -> Self {
        Self {
            total: profile.total,
            average: profile.average,
            min: profile.min,
            max: profile.max,
            completed: profile.completed,
            aborted: profile.aborted,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct TimingCutoffs {
    total: Option<Duration>,
    average: Option<Duration>,
    min: Option<Duration>,
    max: Option<Duration>,
}

impl TimingCutoffs {
    fn new(metrics: impl IntoIterator<Item = TimingMetrics>) -> Self {
        let metrics = metrics.into_iter().collect::<Vec<_>>();
        let warn_count = metrics.len() / 10;
        Self {
            total: Self::cutoff(metrics.iter().map(|metrics| metrics.total), warn_count),
            average: Self::cutoff(metrics.iter().map(|metrics| metrics.average), warn_count),
            min: Self::cutoff(metrics.iter().map(|metrics| metrics.min), warn_count),
            max: Self::cutoff(metrics.iter().map(|metrics| metrics.max), warn_count),
        }
    }

    fn cutoff(values: impl IntoIterator<Item = Duration>, warn_count: usize) -> Option<Duration> {
        if warn_count == 0 {
            return None;
        }
        let mut values = values.into_iter().collect::<Vec<_>>();
        values.sort_unstable_by(|left, right| right.cmp(left));
        values.get(warn_count - 1).copied()
    }
}

struct HighlightedDuration {
    duration: Duration,
    cutoff: Option<Duration>,
}

impl HighlightedDuration {
    const fn new(duration: Duration, cutoff: Option<Duration>) -> Self {
        Self { duration, cutoff }
    }
}

impl Display for HighlightedDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let duration = DisplayDuration(self.duration);
        if self.cutoff.is_some_and(|cutoff| self.duration >= cutoff) {
            f.write_markup(markup! { <Warn>{duration}</Warn> })
        } else {
            duration.fmt(f)
        }
    }
}

struct DisplayDuration(Duration);

impl Display for DisplayDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_fmt(format_args!("{:.3?}", self.0))
    }
}

impl Display for TypeInferenceLocationAttribution {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        f.write_str(match self {
            Self::Exact => "exact",
            Self::RequestOrigin => "request origin",
            Self::Document => "document-wide",
        })
    }
}

struct SourcePath<'a> {
    path: &'a str,
    working_directory: Option<&'a Utf8Path>,
}

impl<'a> SourcePath<'a> {
    const fn new(path: &'a str, working_directory: Option<&'a Utf8Path>) -> Self {
        Self {
            path,
            working_directory,
        }
    }
}

impl Display for SourcePath<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let path = Utf8Path::new(self.path);
        if !path.is_absolute() {
            f.write_str(path.as_str())
        } else if let Some(relative) = self
            .working_directory
            .and_then(|working_directory| path.strip_prefix(working_directory).ok())
        {
            f.write_str(relative.as_str())
        } else {
            f.write_str("<external>/")?;
            f.write_str(path.file_name().unwrap_or("unknown"))
        }
    }
}

struct SourceRange(Option<biome_rowan::TextRange>);

impl Display for SourceRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        if let Some(range) = self.0 {
            f.write_fmt(format_args!(
                "{}..{}",
                u32::from(range.start()),
                u32::from(range.end())
            ))
        } else {
            f.write_str("document-wide")
        }
    }
}

struct SourceLocation<'a> {
    location: &'a TypeInferenceProfileLocation,
    working_directory: Option<&'a Utf8Path>,
}

impl<'a> SourceLocation<'a> {
    const fn new(
        location: &'a TypeInferenceProfileLocation,
        working_directory: Option<&'a Utf8Path>,
    ) -> Self {
        Self {
            location,
            working_directory,
        }
    }
}

impl Display for SourceLocation<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        SourcePath::new(&self.location.path, self.working_directory).fmt(f)?;
        if self.location.range.is_some() {
            f.write_str(":")?;
            SourceRange(self.location.range).fmt(f)?;
        }
        Ok(())
    }
}

fn record_capacity_warning(
    visitor: &mut dyn Visit,
    snapshot: &TypeInferenceProfileSnapshot,
) -> io::Result<()> {
    let dropped_metric_events = snapshot
        .dropped_request_keys
        .saturating_add(snapshot.dropped_query_keys)
        .saturating_add(snapshot.dropped_whole_module_keys);
    let dropped_documents = snapshot.dropped_documents;
    if dropped_metric_events > 0 || dropped_documents > 0 {
        visitor.record_log(
            LogCategory::Warn,
            &markup! {
                "Profile capacity was reached; uncaptured metric events: "{dropped_metric_events}
                ", uncaptured document paths: "{dropped_documents}". Rerun on a narrower path."
            },
        )?;
    }
    Ok(())
}
