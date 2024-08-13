use std::{
    borrow::Cow,
    hash::Hash,
    ops::Sub,
    ptr,
    time::{Duration, Instant},
};

use hdrhistogram::Histogram;
use rustc_hash::FxHashMap;
use std::sync::{LazyLock, Mutex, RwLock};
use tracing::{span, subscriber::Interest, Level, Metadata, Subscriber};
use tracing_subscriber::{
    layer::Context,
    prelude::*,
    registry::{LookupSpan, SpanRef},
    Layer,
};

/// Implementation of a tracing [Layer] that collects timing information for spans into [Histogram]s
struct MetricsLayer;

static METRICS: LazyLock<RwLock<FxHashMap<CallsiteKey, Mutex<CallsiteEntry>>>> =
    LazyLock::new(RwLock::default);

/// Static pointer to the metadata of a callsite, used as a unique identifier
/// for collecting spans created from there in the global metrics map
struct CallsiteKey(&'static Metadata<'static>);

impl PartialEq for CallsiteKey {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.0, other.0)
    }
}

impl Eq for CallsiteKey {}

impl Hash for CallsiteKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(self.0, state);
    }
}

/// Single entry in the global callsite storage, containing handles to the
/// histograms associated with this callsite
enum CallsiteEntry {
    /// Spans with the debug level only count their total duration
    Debug { total: Histogram<u64> },
    /// Spans with the trace level count their total duration as well as
    /// individual busy and idle times
    Trace {
        total: Histogram<u64>,
        busy: Histogram<u64>,
        idle: Histogram<u64>,
    },
}

impl CallsiteEntry {
    fn from_level(level: &Level) -> Self {
        /// Number of significant figures retained by the histogram
        const SIGNIFICANT_FIGURES: u8 = 3;

        match level {
            &Level::TRACE => Self::Trace {
                // SAFETY: Histogram::new only returns an error if the value of
                // SIGNIFICANT_FIGURES is invalid, 3 is statically known to work
                total: Histogram::new(SIGNIFICANT_FIGURES).unwrap(),
                busy: Histogram::new(SIGNIFICANT_FIGURES).unwrap(),
                idle: Histogram::new(SIGNIFICANT_FIGURES).unwrap(),
            },
            _ => Self::Debug {
                total: Histogram::new(SIGNIFICANT_FIGURES).unwrap(),
            },
        }
    }

    fn into_histograms(self, name: &str) -> Vec<(Cow<str>, Histogram<u64>)> {
        match self {
            CallsiteEntry::Debug { total } => vec![(Cow::Borrowed(name), total)],
            CallsiteEntry::Trace { total, busy, idle } => vec![
                (Cow::Borrowed(name), total),
                (Cow::Owned(format!("{name}.busy")), busy),
                (Cow::Owned(format!("{name}.idle")), idle),
            ],
        }
    }
}

/// Extension data attached to tracing spans to keep track of their idle and busy time
///
/// Most of the associated code is based on the similar logic found in `tracing-subscriber`
/// for printing span timings to the console:
/// https://github.com/tokio-rs/tracing/blob/6f23c128fced6409008838a3223d76d7332d79e9/tracing-subscriber/src/fmt/fmt_subscriber.rs#L973
struct Timings<I = Instant> {
    idle: u64,
    busy: u64,
    last: I,
}

trait Timepoint: Sub<Self, Output = Duration> + Copy + Sized {
    fn now() -> Self;
}

impl Timepoint for Instant {
    fn now() -> Self {
        Instant::now()
    }
}

impl<I: Timepoint> Timings<I> {
    fn new() -> Self {
        Self {
            idle: 0,
            busy: 0,
            last: I::now(),
        }
    }

    /// Count the time between the last update and now as idle
    fn enter(&mut self, now: I) {
        self.idle += (now - self.last).as_nanos() as u64;
        self.last = now;
    }

    /// Count the time between the last update and now as busy
    fn exit(&mut self, now: I) {
        self.busy += (now - self.last).as_nanos() as u64;
        self.last = now;
    }

    /// Exit the timing for this span, and record it into a callsite entry
    fn record(mut self, now: I, entry: &mut CallsiteEntry) {
        self.exit(now);

        match entry {
            CallsiteEntry::Debug { total } => {
                total.record(self.busy + self.idle).unwrap();
            }
            CallsiteEntry::Trace { total, busy, idle } => {
                busy.record(self.busy).unwrap();
                idle.record(self.idle).unwrap();
                total.record(self.busy + self.idle).unwrap();
            }
        }
    }
}

fn read_span<'ctx, S>(ctx: &'ctx Context<'_, S>, id: &span::Id) -> SpanRef<'ctx, S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    ctx.span(id)
        .expect("Span not found, it should have been stored in the registry")
}

impl<S> Layer<S> for MetricsLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    /// Only express interest in span callsites, disabling collection of events,
    /// and create new histogram for the spans created by this callsite
    fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest {
        if !metadata.is_span() {
            return Interest::never();
        }

        let entry = CallsiteEntry::from_level(metadata.level());

        METRICS
            .write()
            .unwrap()
            .insert(CallsiteKey(metadata), Mutex::new(entry));

        Interest::always()
    }

    /// When a new span is created, attach the timing data extension to it
    fn on_new_span(&self, _attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = read_span(&ctx, id);
        let mut extensions = span.extensions_mut();

        if extensions.get_mut::<Timings>().is_none() {
            extensions.insert(Timings::<Instant>::new());
        }
    }

    /// When a span is entered, start counting idle time for the parent span if
    /// it exists and busy time for the entered span itself
    fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
        let span = read_span(&ctx, id);

        let now = Instant::now();
        if let Some(parent) = span.parent() {
            let mut extensions = parent.extensions_mut();
            if let Some(timings) = extensions.get_mut::<Timings>() {
                // The parent span was busy until now
                timings.exit(now);
            }
        }

        let mut extensions = span.extensions_mut();
        if let Some(timings) = extensions.get_mut::<Timings>() {
            // The child span was idle until now
            timings.enter(now);
        }
    }

    /// When a span is exited, stop it from counting busy time and start
    /// counting the parent as busy instead
    fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>) {
        let span = read_span(&ctx, id);

        let now = Instant::now();
        let mut extensions = span.extensions_mut();
        if let Some(timings) = extensions.get_mut::<Timings>() {
            // Child span was busy until now
            timings.exit(now);
        }

        // Re-enter parent
        if let Some(parent) = span.parent() {
            let mut extensions = parent.extensions_mut();
            if let Some(timings) = extensions.get_mut::<Timings>() {
                // Parent span was idle until now
                timings.enter(now);
            }
        }
    }

    /// When a span is closed, extract its timing information and write it to
    /// the associated histograms
    fn on_close(&self, id: span::Id, ctx: Context<'_, S>) {
        let span = read_span(&ctx, &id);
        let mut extensions = span.extensions_mut();
        if let Some(timing) = extensions.remove::<Timings>() {
            let now = Instant::now();

            // Acquire a read lock on the metrics storage, access the metrics entry
            // associated with this call site and acquire a write lock on it
            let metrics = METRICS.read().unwrap();
            let entry = metrics
                .get(&CallsiteKey(span.metadata()))
                .expect("callsite not found, it should have been registered in register_callsite");

            let mut entry = entry.lock().unwrap();
            timing.record(now, &mut entry);
        }
    }
}

/// Initializes metrics recording
pub fn init_metrics() {
    // Create and injects the metrics recording layer with the tracing library
    tracing_subscriber::registry().with(MetricsLayer).init();
}

/// Flush and print the recorded metrics to the console
pub fn print_metrics() {
    let mut write_guard = METRICS.write().unwrap();
    let mut histograms: Vec<_> = write_guard
        .drain()
        .flat_map(|(key, entry)| entry.into_inner().unwrap().into_histograms(key.0.name()))
        .collect();

    histograms.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    for (key, histogram) in histograms {
        // Print the header line for the histogram with its name, mean sample
        // duration and standard deviation
        println!(
            "{}: mean = {:.1?}, stdev = {:.1?}",
            key,
            Duration::from_nanos(histogram.mean().round() as u64),
            Duration::from_nanos(histogram.stdev().round() as u64),
        );

        // For each quantile bucket in the histogram print out the associated
        // duration, a bar corresponding to the percentage of the total number
        // of samples falling within this bucket and the percentile
        // corresponding to this bucket
        let total = histogram.len() as f64;
        for v in histogram.iter_quantiles(1) {
            let duration = Duration::from_nanos(v.value_iterated_to());

            let count = v.count_since_last_iteration() as f64;
            let bar_length = (count * 40.0 / total).ceil() as usize;

            println!(
                "{: >7.1?} | {:40} | {:5.1}%",
                duration,
                "*".repeat(bar_length),
                v.quantile_iterated_to() * 100.0,
            );
        }

        // Print an empty line after each histogram
        println!();
    }
}

#[cfg(test)]
mod tests {
    use std::{ops::Sub, thread, time::Duration};

    use tracing::Level;
    use tracing_subscriber::prelude::*;

    use super::{CallsiteEntry, CallsiteKey, MetricsLayer, Timepoint, Timings, METRICS};

    #[derive(Clone, Copy)]
    struct TestTime(u64);

    impl Sub for TestTime {
        type Output = Duration;

        fn sub(self, rhs: Self) -> Self::Output {
            Duration::from_nanos(self.0 - rhs.0)
        }
    }

    impl Timepoint for TestTime {
        fn now() -> Self {
            Self(0)
        }
    }

    #[test]
    fn test_timing() {
        let mut entry = CallsiteEntry::from_level(&Level::TRACE);

        for i in 1..=5 {
            let mut timing = Timings::<TestTime>::new();

            timing.enter(TestTime(i));

            timing.record(TestTime(i * 2), &mut entry);
        }

        let histograms = entry.into_histograms("test");
        for (name, histogram) in histograms {
            let scale = match name.as_ref() {
                "test" => 2.0,
                "test.idle" | "test.busy" => 1.0,
                _ => unreachable!(),
            };

            let sample_count = 5;
            assert_eq!(histogram.len(), sample_count);

            let mean = 3.0 * scale;
            assert_eq!(histogram.mean(), mean);

            let sum = (1..=5).fold(0.0, |sum, i| {
                let sample = i as f64 * scale;
                sum + (sample - mean).powi(2)
            });

            let stddev = (sum / sample_count as f64).sqrt();
            assert_eq!(histogram.stdev(), stddev);

            let s = scale as u64 - 1;
            let expected_buckets = [
                (0, s, 0.0),
                (1, 2 * s + 1, 0.2),
                (1, 3 * s + 2, 0.4),
                (1, 4 * s + 3, 0.6),
                (1, 5 * s + 4, 0.8),
                (1, 6 * s + 5, 1.0),
            ];

            for (bucket, expected) in histogram.iter_linear(scale as u64).zip(&expected_buckets) {
                let (count, value, quantile) = *expected;

                assert_eq!(bucket.count_since_last_iteration(), count);
                assert_eq!(bucket.value_iterated_to(), value);
                assert_eq!(bucket.quantile_iterated_to(), quantile);
            }
        }
    }

    #[test]
    fn test_layer() {
        let _guard = tracing_subscriber::registry()
            .with(MetricsLayer)
            .set_default();

        let key = {
            let span = tracing::trace_span!("test_layer");
            span.in_scope(|| {
                thread::sleep(Duration::from_millis(1));
            });

            span.metadata().expect("span is disabled")
        };

        let entry = {
            let mut metrics = METRICS.write().unwrap();
            metrics.remove(&CallsiteKey(key))
        };

        let entry = entry.expect("callsite does not exist in metrics storage");

        let entry = entry.into_inner().unwrap();
        let histograms = entry.into_histograms(key.name());

        for (_, histogram) in histograms {
            assert_eq!(histogram.len(), 1);
        }
    }
}
