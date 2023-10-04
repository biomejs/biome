use biome_console::{markup, Console, ConsoleExt, EnvConsole};
use biome_service::VERSION;
use std::fmt;
use std::str::FromStr;
use std::sync::RwLock;
use std::time::Instant;
use time::{format_description, OffsetDateTime, PrimitiveDateTime, Time};
use tracing::field::AsField;
use tracing::span::Attributes;
use tracing::subscriber::Interest;
use tracing::{Event, Id, Level, Metadata, Subscriber};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{format, FmtContext, FormatEvent, FormatFields, FormattedFields};
use tracing_subscriber::layer::{Context, Filter, SubscriberExt};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{registry, Layer};

pub fn setup_cli_subscriber(level: LoggingLevel) {
    let console = EnvConsole::default();
    // Configure a custom event formatter
    let format = tracing_subscriber::fmt::layer()
        .with_level(true) // don't include levels in formatted output
        .with_target(false) // don't include targets
        .with_thread_names(true) // include the name of the current thread
        .with_file(false)
        .with_ansi(true)
        .compact(); // use the `Compact` formatting style.

    // Create a `fmt` subscriber that uses our custom event format, and set it
    // as the default.
    // let subscriber = tracing_subscriber::fmt::layer()
    //     .with_span_events(FmtSpan::CLOSE)
    //     .with_max_level(match level {
    //         LoggingLevel::Info => LevelFilter::INFO,
    //         LoggingLevel::Warn => LevelFilter::WARN,
    //         LoggingLevel::Error => LevelFilter::ERROR,
    //         LoggingLevel::Debug => LevelFilter::DEBUG,
    //     })
    //     .event_format(format)
    //     .finish();
    // subscriber.with(CustomLayer);
    registry()
        .with(format.event_format(MyFormatter))
        .with(CustomLayer {
            level: level.clone(),
            console: RwLock::new(console),
        })
        //     // .with(
        //     //     HierarchicalLayer::default()
        //     //         .with_indent_lines(false)
        //     //         .with_indent_amount(2)
        //     //         .with_bracketed_fields(true)
        //     //         .with_targets(true)
        //     //         .with_ansi(false)
        //     //         .with_filter(LoggingFilter { level }),
        //     // )
        .init();
}

struct EventFormatter;

impl<S, N> FormatEvent<S, N> for EventFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let metadata = event.metadata();

        writeln!(writer)
    }
}

#[derive(Debug, Default, Clone)]
pub enum LoggingLevel {
    #[default]
    None,
    Info,
    Warn,
    Error,
    Debug,
}

impl LoggingLevel {
    fn to_filter_level(&self) -> Option<LevelFilter> {
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
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            "debug" => Ok(Self::Debug),
            _ => Err("Unexpected value".to_string()),
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
            if VERSION == "0.0.0" {
                LevelFilter::TRACE
            } else {
                if let Some(level) = self.level.to_filter_level() {
                    level
                } else {
                    return false;
                }
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

struct Timing {
    started_at: Instant,
}

pub struct CustomLayer {
    level: LoggingLevel,
    console: RwLock<EnvConsole>,
}

impl CustomLayer {
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

impl<S> Layer<S> for CustomLayer
where
    S: Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
{
    fn enabled(&self, meta: &Metadata<'_>, _cx: Context<'_, S>) -> bool {
        self.is_enabled(meta)
    }
    fn max_level_hint(&self) -> Option<LevelFilter> {
        self.level.to_filter_level()
    }
    fn on_new_span(&self, _attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).unwrap();

        span.extensions_mut().insert(Timing {
            started_at: Instant::now(),
        });
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).unwrap();

        let started_at = span.extensions().get::<Timing>().unwrap().started_at;
        let now = OffsetDateTime::now_utc();

        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]")
                .expect("");
        let time = now.format(&format).expect("");

        let mut console = self.console.write().unwrap();
        let level = span.metadata().level();
        let level = match level {
            &Level::INFO => markup!(<Info>{{level.as_str()}}</Info>).to_owned(),
            &Level::WARN => markup!(<Warn>{{level.as_str()}}</Warn>).to_owned(),
            &Level::ERROR => markup!(<Error>{{level.as_str()}}</Error>).to_owned(),
            _ => markup!({ level.as_str() }).to_owned(),
        };

        console.log(markup! {
            "["{time}"] " {level} " The "<Emphasis>{span.metadata().name()}</Emphasis>" took "{(Instant::now() - started_at).as_millis()}"ms"
        });
    }
}

struct MyFormatter;

impl<S, N> FormatEvent<S, N> for MyFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();
        write!(&mut writer, "{} {}: ", metadata.level(), metadata.target())?;

        // Format all the spans in the event's span context.
        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name())?;

                // `FormattedFields` is a formatted representation of the span's
                // fields, which is stored in its extensions by the `fmt` layer's
                // `new_span` method. The fields will have been formatted
                // by the same field formatter that's provided to the event
                // formatter in the `FmtContext`.
                let ext = span.extensions();
                let fields = ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");

                // Skip formatting the fields if the span had no fields.
                if !fields.is_empty() {
                    write!(writer, " {}", fields)?;
                }
                write!(writer, ": ")?;
            }
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
