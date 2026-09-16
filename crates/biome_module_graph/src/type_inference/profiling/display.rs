//! Terminal output for type-inference profiles.
//!
//! The default report groups related records and limits the number shown. The
//! verbose report prints every captured source record and the Rust location of
//! the code that produced it.

mod compact;
mod verbose;

use std::io;
use std::time::Duration;

use biome_console::fmt::{Display, Formatter};
use biome_console::markup;
use camino::Utf8Path;

use super::{
    TypeInferenceLocationAttribution, TypeInferenceProfileLocation, TypeInferenceProfileSnapshot,
    TypeInferenceQueryProfile, TypeInferenceRequestProfile, TypeInferenceWholeModuleProfile,
};

const RECORD_INDENT: usize = 2;

/// Renders one type-inference profile for terminal-oriented CLI output.
pub struct DisplayTypeInferenceProfile<'a> {
    snapshot: &'a TypeInferenceProfileSnapshot,
    working_directory: Option<&'a Utf8Path>,
    version: &'a str,
    verbose: bool,
}

impl<'a> DisplayTypeInferenceProfile<'a> {
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

    /// Selects the report format.
    ///
    /// `true` prints every captured source record and its code reference.
    /// `false` groups records and limits the ranked request and query sections.
    pub const fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

impl Display for DisplayTypeInferenceProfile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        if self.verbose {
            verbose::VerboseTypeInferenceProfile::new(
                self.snapshot,
                self.working_directory,
                self.version,
            )
            .fmt(f)
        } else {
            compact::CompactTypeInferenceProfile::new(
                self.snapshot,
                self.working_directory,
                self.version,
            )
            .fmt(f)
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
        let path = Utf8Path::new(&self.location.path);
        if !path.is_absolute() {
            f.write_str(path.as_str())?;
        } else if let Some(relative) = self
            .working_directory
            .and_then(|working_directory| path.strip_prefix(working_directory).ok())
        {
            f.write_str(relative.as_str())?;
        } else {
            f.write_str("<external>/")?;
            f.write_str(path.file_name().unwrap_or("unknown"))?;
        }
        if let Some(range) = self.location.range {
            f.write_fmt(format_args!(
                ":{}..{}",
                u32::from(range.start()),
                u32::from(range.end())
            ))?;
        }
        Ok(())
    }
}

struct CapacityWarning<'a>(&'a TypeInferenceProfileSnapshot);

impl Display for CapacityWarning<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> io::Result<()> {
        let dropped_metric_events = self
            .0
            .dropped_request_keys
            .saturating_add(self.0.dropped_query_keys)
            .saturating_add(self.0.dropped_whole_module_keys);
        let dropped_documents = self.0.dropped_documents;
        if dropped_metric_events > 0 || dropped_documents > 0 {
            f.write_markup(markup! {
                <Warn>"Profile capacity was reached; uncaptured metric events: "{dropped_metric_events}", uncaptured document paths: "{dropped_documents}". Rerun on a narrower path."</Warn>"\n"
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use biome_console::fmt::{Formatter, Termcolor};
    use biome_console::markup;
    use biome_diagnostics::termcolor::NoColor;
    use biome_rowan::TextRange;
    use camino::Utf8Path;

    use super::{SourceLocation, TimingCutoffs, TimingMetrics, TypeInferenceProfileLocation};
    use crate::type_inference::profiling::TypeInferenceLocationAttribution;

    fn render(display: SourceLocation<'_>) -> String {
        let mut buffer = Vec::new();
        let mut writer = Termcolor(NoColor::new(&mut buffer));
        let mut f = Formatter::new(&mut writer);
        f.write_markup(markup! {{ display }}).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    #[test]
    fn timing_cutoffs_select_the_highest_ten_percent() {
        let cutoffs = TimingCutoffs::new((1..=10).map(|seconds| TimingMetrics {
            total: Duration::from_secs(seconds),
            average: Duration::from_secs(seconds * 2),
            min: Duration::from_secs(seconds * 3),
            max: Duration::from_secs(seconds * 4),
            completed: 1,
            aborted: 0,
        }));

        assert_eq!(cutoffs.total, Some(Duration::from_secs(10)));
        assert_eq!(cutoffs.average, Some(Duration::from_secs(20)));
        assert_eq!(cutoffs.min, Some(Duration::from_secs(30)));
        assert_eq!(cutoffs.max, Some(Duration::from_secs(40)));
    }

    #[test]
    fn location_uses_relative_path_and_text_range() {
        let workspace = if cfg!(windows) {
            Utf8Path::new("C:\\workspace")
        } else {
            Utf8Path::new("/workspace")
        };
        let other = if cfg!(windows) {
            Utf8Path::new("C:\\other")
        } else {
            Utf8Path::new("/other")
        };
        let location = TypeInferenceProfileLocation {
            path: workspace
                .join("packages")
                .join("example.ts")
                .into_string()
                .into(),
            range: Some(TextRange::new(2.into(), 8.into())),
            attribution: TypeInferenceLocationAttribution::Exact,
        };

        assert_eq!(
            render(SourceLocation::new(&location, Some(workspace))),
            format!("{}:2..8", Utf8Path::new("packages").join("example.ts"))
        );
        assert_eq!(
            render(SourceLocation::new(&location, Some(other))),
            "<external>/example.ts:2..8"
        );
    }
}
