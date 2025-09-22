use std::fmt::{Display, Formatter};
use std::fs::File;
use std::str::FromStr;

use crate::cli_options::ColorsArg;
use tracing::Metadata;
use tracing::subscriber::Interest;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::{Context, Filter, SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer as _, registry};

pub fn setup_cli_subscriber(
    file: Option<&str>,
    level: LoggingLevel,
    kind: LoggingKind,
    colors: Option<&ColorsArg>,
) {
    if level == LoggingLevel::None {
        return;
    }

    let mut format = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_target(false)
        .with_thread_names(true)
        .with_file(true)
        .with_ansi(colors.is_none_or(|c| c.is_enabled()));

    if level == LoggingLevel::Tracing {
        format = format.with_span_events(FmtSpan::CLOSE);
    }

    // FIXME: I hate the duplication here, and I tried to make a function that
    //        could take `impl Layer<Registry>` so the compiler could expand
    //        this for us... but I got dragged into a horrible swamp of generic
    //        constraints...
    if let Some(file) = file {
        let file = File::create(file).expect("Failed to create log file");
        let format = format.with_writer(file);
        match kind {
            LoggingKind::Pretty => {
                let format = format.pretty();
                registry()
                    .with(format.with_filter(LoggingFilter { level }))
                    .init()
            }
            LoggingKind::Compact => {
                let format = format.compact();
                registry()
                    .with(format.with_filter(LoggingFilter { level }))
                    .init()
            }
            LoggingKind::Json => {
                let format = format.json().flatten_event(true);
                registry()
                    .with(format.with_filter(LoggingFilter { level }))
                    .init()
            }
        }
    } else {
        match kind {
            LoggingKind::Pretty => {
                let format = format.pretty();
                registry()
                    .with(format.with_filter(LoggingFilter { level }))
                    .init()
            }
            LoggingKind::Compact => {
                let format = format.compact();
                registry()
                    .with(format.with_filter(LoggingFilter { level }))
                    .init()
            }
            LoggingKind::Json => {
                let format = format.json().flatten_event(true);
                registry()
                    .with(format.with_filter(LoggingFilter { level }))
                    .init()
            }
        }
    };
}

#[derive(Copy, Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum LoggingLevel {
    /// No logs should be shown
    #[default]
    None,
    Tracing,
    Debug,
    Info,
    Warn,
    Error,
}

impl LoggingLevel {
    fn to_filter_level(self) -> Option<LevelFilter> {
        match self {
            Self::None => None,
            Self::Tracing => Some(LevelFilter::TRACE),
            Self::Debug => Some(LevelFilter::DEBUG),
            Self::Info => Some(LevelFilter::INFO),
            Self::Warn => Some(LevelFilter::WARN),
            Self::Error => Some(LevelFilter::ERROR),
        }
    }
}

impl FromStr for LoggingLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "tracing" => Ok(Self::Tracing),
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            _ => Err("Unexpected value".to_string()),
        }
    }
}

impl Display for LoggingLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Tracing => write!(f, "tracing"),
            Self::Debug => write!(f, "debug"),
            Self::Info => write!(f, "info"),
            Self::Warn => write!(f, "warn"),
            Self::Error => write!(f, "error"),
        }
    }
}

/// Tracing filter enabling:
/// - All spans and events at level info or higher
/// - All spans and events at level debug in crates whose name starts with `biome`
struct LoggingFilter {
    level: LoggingLevel,
}

/// Tracing filter used for spans emitted by `biome*` crates
const SELF_FILTER: LevelFilter = if cfg!(debug_assertions) {
    LevelFilter::TRACE
} else {
    LevelFilter::DEBUG
};

impl LoggingFilter {
    fn is_enabled(&self, meta: &Metadata<'_>) -> bool {
        let filter = if meta.target().starts_with("biome") {
            if let Some(level) = self.level.to_filter_level() {
                level
            } else {
                return false;
            }
        } else {
            LevelFilter::INFO
        };

        meta.level() <= &filter
    }
}

impl<S> Filter<S> for LoggingFilter {
    fn enabled(&self, meta: &Metadata<'_>, _cx: &Context<'_, S>) -> bool {
        self.is_enabled(meta)
    }

    fn callsite_enabled(&self, meta: &'static Metadata<'static>) -> Interest {
        if self.is_enabled(meta) {
            Interest::always()
        } else {
            Interest::never()
        }
    }

    fn max_level_hint(&self) -> Option<LevelFilter> {
        Some(SELF_FILTER)
    }
}

/// The kind of logging
#[derive(Copy, Debug, Default, Clone, Eq, PartialEq)]
pub enum LoggingKind {
    /// A pretty log on multiple lines with nice colours
    #[default]
    Pretty,
    /// A more cluttered logging
    Compact,
    /// Logs are emitted in JSON format
    Json,
}

impl Display for LoggingKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pretty => write!(f, "pretty"),
            Self::Compact => write!(f, "compact"),
            Self::Json => write!(f, "json"),
        }
    }
}

impl FromStr for LoggingKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "compact" => Ok(Self::Compact),
            "pretty" => Ok(Self::Pretty),
            "json" => Ok(Self::Json),
            _ => Err("This log kind doesn't exist".to_string()),
        }
    }
}
