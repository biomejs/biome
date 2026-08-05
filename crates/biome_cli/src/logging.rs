use crate::cli_options::ColorsArg;
use bpaf::Bpaf;
use camino::Utf8PathBuf;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::str::FromStr;
use tracing::subscriber::Interest;
use tracing::{Level, Metadata};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::{Context, Filter, SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer as _, registry};

/// Options that control internal CLI and daemon server logging.
#[derive(Debug, Clone, Bpaf)]
pub struct LogOptions {
    /// For file-processing commands, writes internal CLI logs to this file instead of standard output.
    #[bpaf(long("log-file"), env("BIOME_LOG_FILE"), argument("PATH"), hide_usage)]
    pub log_file: Option<String>,

    /// When starting the daemon server, sets the file name prefix used for rotated log files.
    #[bpaf(
        env("BIOME_LOG_PREFIX_NAME"),
        long("log-prefix-name"),
        argument("STRING"),
        fallback(String::from("server.log")),
        display_fallback
    )]
    pub log_prefix_name: String,

    /// When starting the daemon server, sets the directory where log files are stored.
    #[bpaf(
        env("BIOME_LOG_PATH"),
        long("log-path"),
        argument("PATH"),
        fallback(biome_fs::ensure_cache_dir().join("biome-logs")),
        display_fallback
    )]
    pub log_path: Utf8PathBuf,

    /// For file-processing commands, sets the internal CLI logging level to `tracing`, `debug`, `info`, `warn`, `error`, or `none`.
    #[bpaf(
        long("log-level"),
        env("BIOME_LOG_LEVEL"),
        argument("none|tracing|debug|info|warn|error"),
        fallback(LoggingLevel::default()),
        display_fallback
    )]
    pub log_level: LoggingLevel,

    /// For file-processing commands, selects the internal CLI log format: `pretty`, `compact`, or `json`.
    #[bpaf(
        long("log-kind"),
        env("BIOME_LOG_KIND"),
        argument("pretty|compact|json"),
        fallback(LoggingKind::default()),
        display_fallback
    )]
    pub log_kind: LoggingKind,
}

impl Default for LogOptions {
    fn default() -> Self {
        Self {
            log_file: None,
            log_prefix_name: String::from("server.log"),
            log_path: biome_fs::ensure_cache_dir().join("biome-logs"),
            log_level: LoggingLevel::default(),
            log_kind: LoggingKind::default(),
        }
    }
}

pub fn setup_cli_subscriber(
    file: Option<&str>,
    level: LoggingLevel,
    kind: LoggingKind,
    colors: Option<&ColorsArg>,
) {
    use tracing_subscriber_ext::*;

    if level == LoggingLevel::None {
        return;
    }

    let fmt_span = matches!(level, LoggingLevel::Tracing)
        .then_some(FmtSpan::CLOSE)
        .unwrap_or(FmtSpan::NONE);

    let make_writer = file
        .map(File::create)
        .transpose()
        .expect("Failed to create log file")
        .optional()
        .or_else(std::io::stdout);

    let layer = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_target(false)
        .with_thread_names(true)
        .with_file(true)
        .with_ansi(colors.is_none_or(|c| c.is_enabled()))
        .with_span_events(fmt_span)
        .with_writer(make_writer);

    let registry = registry();
    let filter = LoggingFilter { level };
    match kind {
        LoggingKind::Pretty => registry.with(layer.pretty().with_filter(filter)).init(),
        LoggingKind::Compact => registry.with(layer.compact().with_filter(filter)).init(),
        LoggingKind::Json => registry
            .with(layer.json().flatten_event(true).with_filter(filter))
            .init(),
    };
}

#[derive(Copy, Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum LoggingLevel {
    /// Disables logging.
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
/// - Spans and events at the requested level in `biome*` crates.
/// - Spans and events at level info or higher in dependencies.
/// - Spans and events at level warn or higher in Salsa.
struct LoggingFilter {
    level: LoggingLevel,
}

/// Tracing filter used for spans emitted by `biome*` crates.
const SELF_FILTER: LevelFilter = if cfg!(debug_assertions) {
    LevelFilter::TRACE
} else {
    LevelFilter::DEBUG
};

impl LoggingFilter {
    fn is_enabled(&self, meta: &Metadata<'_>) -> bool {
        self.is_enabled_for(meta.target(), meta.level())
    }

    fn is_enabled_for(&self, target: &str, level: &Level) -> bool {
        let filter = if target.starts_with("biome") {
            if let Some(level) = self.level.to_filter_level() {
                level
            } else {
                return false;
            }
        } else if target == "salsa" || target.starts_with("salsa::") {
            LevelFilter::WARN
        } else {
            LevelFilter::INFO
        };

        level <= &filter
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

/// The log output format.
#[derive(Copy, Debug, Default, Clone, Eq, PartialEq)]
pub enum LoggingKind {
    /// Displays human-readable, multiline logs with ANSI styling when enabled.
    #[default]
    Pretty,
    /// Displays compact logs.
    Compact,
    /// Emits logs as JSON.
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

mod tracing_subscriber_ext {
    //! Extensions module for [tracing_subscriber].
    //!
    //! This module is kept private to preserve API flexibility.

    use tracing::Metadata;
    use tracing_subscriber::fmt::{MakeWriter, writer::OptionalWriter};

    /// A wrapper type for an optional [MakeWriter].
    ///
    /// Implements [MakeWriter] for `Option<M>` where `M: MakeWriter`.
    ///
    /// XXX: Remove after [PR](https://github.com/tokio-rs/tracing/pull/3196) is merged.
    pub(super) struct OptionMakeWriter<M>(Option<M>);

    impl<'a, M> MakeWriter<'a> for OptionMakeWriter<M>
    where
        M: MakeWriter<'a> + 'a,
    {
        type Writer = OptionalWriter<M::Writer>;

        fn make_writer(&'a self) -> Self::Writer {
            match &self.0 {
                Some(inner) => OptionalWriter::some(inner.make_writer()),
                None => OptionalWriter::none(),
            }
        }

        fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer {
            match &self.0 {
                Some(inner) => OptionalWriter::some(inner.make_writer_for(meta)),
                None => OptionalWriter::none(),
            }
        }
    }

    /// Extension trait for creating [OptionMakeWriter].
    pub(super) trait OptionMakeWriterExt<M> {
        fn optional(self) -> OptionMakeWriter<M>
        where
            Self: Sized;
    }

    impl<M> OptionMakeWriterExt<M> for Option<M> {
        fn optional(self) -> OptionMakeWriter<M> {
            OptionMakeWriter(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LoggingFilter, LoggingLevel};
    use std::{
        io::Write,
        sync::{Arc, Mutex},
    };
    use tracing::Level;

    struct MockWriter {
        bytes: Mutex<Vec<u8>>,
    }

    impl MockWriter {
        /// Creates a new, empty `Arc<Self>`.
        fn new() -> Arc<Self> {
            Arc::new(Self {
                bytes: Mutex::new(Vec::new()),
            })
        }

        /// Wraps `Arc<Self>` in `Some`.
        fn some(self: Arc<Self>) -> Option<Arc<Self>> {
            Some(self)
        }

        /// Wraps `Arc<Self>` in `None`.
        fn none(self: Arc<Self>) -> Option<Arc<Self>> {
            None
        }

        /// Asserts that something was written to this writer.
        fn assert_written(&self) {
            assert!(!self.bytes.lock().unwrap().is_empty())
        }
    }

    impl std::io::Write for &MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.bytes.lock().unwrap().extend_from_slice(buf);

            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn optional_writer_or_else_test() {
        use super::tracing_subscriber_ext::*;
        use tracing_subscriber::fmt::{MakeWriter, writer::MakeWriterExt};

        let writer_one = MockWriter::new();
        let writer_two = MockWriter::new();

        let make_writer = writer_one
            .clone()
            .some()
            .optional()
            .or_else(writer_two.clone());

        let mut writer = make_writer.make_writer();

        writer.write_all(b"Hello, world!").unwrap();

        writer_one.assert_written();

        let writer_one = MockWriter::new();
        let writer_two = MockWriter::new();

        let make_writer = writer_one.none().optional().or_else(writer_two.clone());
        let mut writer = make_writer.make_writer();

        writer.write_all(b"Hello, world!").unwrap();

        writer_two.assert_written();
    }

    #[test]
    fn logging_filter_hides_salsa_info_events() {
        let filter = LoggingFilter {
            level: LoggingLevel::Debug,
        };

        assert!(!filter.is_enabled_for("salsa::runtime", &Level::INFO));
        assert!(!filter.is_enabled_for("salsa", &Level::INFO));
        assert!(filter.is_enabled_for("salsa::runtime", &Level::WARN));
        assert!(filter.is_enabled_for("biome_service", &Level::DEBUG));
        assert!(filter.is_enabled_for("other_dependency", &Level::INFO));
    }
}
