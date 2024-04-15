use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tracing::subscriber::Interest;
use tracing::Metadata;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::{Context, Filter, SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{registry, Layer};

pub fn setup_cli_subscriber(level: LoggingLevel, kind: LoggingKind) {
    if level == LoggingLevel::None {
        return;
    }
    let format = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_target(false)
        .with_thread_names(true)
        .with_file(true)
        .with_ansi(true);
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
    };
}

#[derive(Copy, Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum LoggingLevel {
    /// No logs should be shown
    #[default]
    None,
    Debug,
    Info,
    Warn,
    Error,
}

impl LoggingLevel {
    fn to_filter_level(self) -> Option<LevelFilter> {
        match self {
            LoggingLevel::None => None,
            LoggingLevel::Info => Some(LevelFilter::INFO),
            LoggingLevel::Warn => Some(LevelFilter::WARN),
            LoggingLevel::Error => Some(LevelFilter::ERROR),
            LoggingLevel::Debug => Some(LevelFilter::DEBUG),
        }
    }
}

impl FromStr for LoggingLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            "debug" => Ok(Self::Debug),
            _ => Err("Unexpected value".to_string()),
        }
    }
}

impl Display for LoggingLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoggingLevel::None => write!(f, "none"),
            LoggingLevel::Debug => write!(f, "debug"),
            LoggingLevel::Info => write!(f, "info"),
            LoggingLevel::Warn => write!(f, "warn"),
            LoggingLevel::Error => write!(f, "error"),
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
            LoggingKind::Pretty => write!(f, "pretty"),
            LoggingKind::Compact => write!(f, "compact"),
            LoggingKind::Json => write!(f, "json"),
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
