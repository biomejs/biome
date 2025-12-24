/*!
Per-rule execution time profiling facilities for the analyzer.

This module provides a lightweight, opt-in profiler that tracks the cumulative
execution time spent inside each lint rule's `Rule::run` implementation.

Guidelines and design notes:
- It only measures the time spent executing the lint rule itself, not the time
  spent querying/matching nodes or building the rule context. Integration points
  should start the timer immediately before invoking `R::run` and let it drop
  immediately after `R::run` returns.
- It is concurrency-safe and aggregates timings across threads and files.
- Profiling is disabled by default and must be explicitly enabled at runtime.
  When disabled, the overhead is near-zero (a fast boolean check).

At the end of a run, consumers can call `profiling::snapshot()` to retrieve the
aggregated metrics and print them.

This module intentionally has no output/printing logic; the CLI/reporters are
responsible for formatting and displaying the results.

To keep the public API stable and easy to use from other crates in the workspace,
all entry points are exposed as top-level functions under the `profiling` module
namespace:
- `enable`, `disable`, `is_enabled`
- `start_rule`, `start_plugin_rule`
- `record_rule_time`
- `snapshot`, `reset`, `drain_sorted_by_total`
*/

use crate::matcher::RuleKey;
use crate::rule::Rule;
use biome_console::{MarkupBuf, markup};
use rustc_hash::FxHashMap;
use std::cmp;
use std::fmt;
use std::hash::{Hash, Hasher};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

/// Identifies the origin of a rule for profiling purposes.
///
/// - Built-in rules are addressed by their group and rule name (e.g. "lint/correctness/noUnusedVars").
/// - Plugin rules are addressed by the plugin-provided name.
#[derive(Clone, Debug)]
pub enum RuleLabel {
    Builtin {
        group: &'static str,
        rule: &'static str,
    },
    Plugin(Box<str>),
}

impl RuleLabel {
    pub fn builtin(group: &'static str, rule: &'static str) -> Self {
        Self::Builtin { group, rule }
    }

    pub fn plugin(name: impl Into<Box<str>>) -> Self {
        Self::Plugin(name.into())
    }

    pub fn as_str_components(&self) -> (&str, &str) {
        match self {
            Self::Builtin { group, rule } => (group, rule),
            Self::Plugin(name) => ("plugin", name.as_ref()),
        }
    }
}

impl fmt::Display for RuleLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Builtin { group, rule } => write!(f, "{}/{}", group, rule),
            Self::Plugin(name) => write!(f, "plugin/{}", name),
        }
    }
}

// Manual Eq/Hash that treats labels with identical content as the same key.
impl PartialEq for RuleLabel {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Builtin {
                    group: g1,
                    rule: r1,
                },
                Self::Builtin {
                    group: g2,
                    rule: r2,
                },
            ) => core::ptr::eq(g1, g2) && core::ptr::eq(r1, r2) || (*g1 == *g2 && *r1 == *r2),
            (Self::Plugin(a), Self::Plugin(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for RuleLabel {}

impl Hash for RuleLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Builtin { group, rule } => {
                state.write_u8(0);
                state.write(group.as_bytes());
                state.write_u8(b'/');
                state.write(rule.as_bytes());
            }
            Self::Plugin(name) => {
                state.write_u8(1);
                state.write(name.as_bytes());
            }
        }
    }
}

/// Aggregated metrics for a single rule.
#[derive(Clone, Debug)]
pub struct RuleProfile {
    pub label: RuleLabel,
    pub total: Duration,
    pub count: u64,
    pub min: Duration,
    pub max: Duration,
}

impl RuleProfile {
    pub fn avg(&self) -> Duration {
        if self.count == 0 {
            Duration::ZERO
        } else {
            self.total / (self.count as u32)
        }
    }
}

/// Internal accumulator used by the global profiler.
#[derive(Clone, Debug)]
struct Metric {
    total: Duration,
    count: u64,
    min: Duration,
    max: Duration,
}

impl Default for Metric {
    fn default() -> Self {
        Self {
            total: Duration::ZERO,
            count: 0,
            min: Duration::MAX,
            max: Duration::ZERO,
        }
    }
}

impl Metric {
    fn record(&mut self, delta: Duration) {
        self.total += delta;
        self.count += 1;
        self.min = cmp::min(self.min, delta);
        self.max = cmp::max(self.max, delta);
    }

    fn into_profile(self, label: RuleLabel) -> RuleProfile {
        RuleProfile {
            label,
            total: self.total,
            count: self.count,
            min: if self.count > 0 {
                self.min
            } else {
                Duration::ZERO
            },
            max: self.max,
        }
    }
}

/// Global, process-wide profiler state.
/// Aggregates timings across all threads/files.
#[derive(Default)]
struct RuleProfiler {
    metrics: FxHashMap<RuleLabel, Metric>,
}

impl RuleProfiler {
    fn record(&mut self, label: RuleLabel, delta: Duration) {
        self.metrics.entry(label).or_default().record(delta);
    }

    fn snapshot(&self) -> Vec<RuleProfile> {
        self.metrics
            .iter()
            .map(|(label, metric)| metric.clone().into_profile(label.clone()))
            .collect()
    }

    fn reset(&mut self) {
        self.metrics.clear();
    }
}

#[cfg(not(target_arch = "wasm32"))]
static PROFILER: Mutex<Option<RuleProfiler>> = Mutex::new(None);
#[cfg(not(target_arch = "wasm32"))]
fn with_profiler<R>(f: impl FnOnce(&mut RuleProfiler) -> R) -> Option<R> {
    if let Ok(mut guard) = PROFILER.lock() {
        let profiler = guard.get_or_insert_with(RuleProfiler::default);
        Some(f(profiler))
    } else {
        None
    }
}

#[cfg(target_arch = "wasm32")]
fn with_profiler<R>(_f: impl FnOnce(&mut RuleProfiler) -> R) -> Option<R> {
    None
}

static ENABLED: AtomicBool = AtomicBool::new(false);

/// Enables rule execution profiling for the current process.
pub fn enable() {
    ENABLED.store(true, Ordering::Relaxed);
}

/// Disables rule execution profiling for the current process.
pub fn disable() {
    ENABLED.store(false, Ordering::Relaxed);
}

/// Returns whether profiling is currently enabled.
pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// RAII timer that records elapsed time for a rule when dropped.
pub struct RuleRunTimer {
    label: Option<RuleLabel>,
    #[cfg(not(target_arch = "wasm32"))]
    start: Instant,
}

impl RuleRunTimer {
    fn new_enabled(label: RuleLabel) -> Self {
        Self {
            label: Some(label),
            #[cfg(not(target_arch = "wasm32"))]
            start: Instant::now(),
        }
    }

    fn new_disabled() -> Self {
        // We still initialize `start` to a valid Instant to keep struct layout simple,
        // but it won't be used as `label` is None.
        Self {
            label: None,
            #[cfg(not(target_arch = "wasm32"))]
            start: Instant::now(),
        }
    }

    /// Consume the timer and manually record the elapsed time (useful when you need explicit control).
    pub fn stop(self) {
        drop(self)
    }
}

impl Drop for RuleRunTimer {
    fn drop(&mut self) {
        // We use Drop to record the elapsed time so its impossible to accidentally reuse the timer.
        if let Some(label) = self.label.take() {
            #[cfg(not(target_arch = "wasm32"))]
            let elapsed = self.start.elapsed();
            #[cfg(target_arch = "wasm32")]
            let elapsed = Duration::ZERO;
            with_profiler(|p| p.record(label, elapsed));
        }
    }
}

/// Starts measuring execution time for a built-in rule `R`.
///
/// When profiling is disabled, returns a no-op timer with near-zero overhead.
pub fn start_rule<R: Rule>() -> RuleRunTimer {
    if !is_enabled() {
        return RuleRunTimer::new_disabled();
    }
    let key: RuleKey = RuleKey::rule::<R>();
    RuleRunTimer::new_enabled(RuleLabel::builtin(key.group(), key.rule_name()))
}

/// Starts measuring execution time for a plugin rule with the specified `name`.
///
/// When profiling is disabled, returns a no-op timer with near-zero overhead.
pub fn start_plugin_rule(name: impl Into<Box<str>>) -> RuleRunTimer {
    if !is_enabled() {
        return RuleRunTimer::new_disabled();
    }
    RuleRunTimer::new_enabled(RuleLabel::plugin(name))
}

/// Records a duration for the given label, bypassing RAII timers.
///
/// Useful for one-off custom measurements.
pub fn record_rule_time(label: RuleLabel, delta: Duration) {
    if !is_enabled() {
        return;
    }
    with_profiler(|p| p.record(label, delta));
}

/// Returns a snapshot of all collected profiles in unspecified order.
pub fn snapshot() -> Vec<RuleProfile> {
    with_profiler(|p| p.snapshot()).unwrap_or_default()
}

/// Returns all profiles sorted by total time (descending).
pub fn drain_sorted_by_total(reset_after: bool) -> Vec<RuleProfile> {
    let mut profiles = with_profiler(|p| p.snapshot()).unwrap_or_default();

    profiles.sort_by(|a, b| b.total.cmp(&a.total));

    if reset_after {
        reset();
    }

    profiles
}

/// Clears all collected metrics.
pub fn reset() {
    with_profiler(|p| p.reset());
}

/// Utility for formatting a summary for display purposes.
pub fn format_profiles_table(mut profiles: Vec<RuleProfile>, top_n: Option<usize>) -> MarkupBuf {
    // Sort by total time descending
    profiles.sort_by(|a, b| b.total.cmp(&a.total));
    let limit = top_n.unwrap_or(profiles.len()).min(profiles.len());

    // minimum width of 5, or wider if needed for larger counts
    let count_column_width = profiles
        .iter()
        .map(|p| p.count)
        .max()
        .unwrap_or(0)
        .to_string()
        .len()
        .max(5);

    // Build the entire output as Markup from the beginning
    let mut out = markup!(
        <Emphasis>"Rule execution time"</Emphasis>" "<Dim>"(does not include any preprocessing)"</Dim>"\n"
        <Dim>
            "  "
            {format!("{:<10}", "total")} "  "
            {format!("{:<10}", "avg")} "  "
            {format!("{:<10}", "min")} "  "
            {format!("{:<10}", "max")} "  "
            {format!("{:<1$}", "count", count_column_width)} "  "
            {"rule"}
        </Dim>"\n"
    )
    .to_owned();

    // Determine per-column cutoffs for the largest 10% values among displayed rows
    let warn_count = ((limit as f64) * 0.10).ceil() as usize;

    // Collect column values from the displayed slice
    let displayed: Vec<&RuleProfile> = profiles.iter().take(limit).collect();

    let mut totals: Vec<_> = displayed.iter().map(|p| p.total).collect();
    let mut avgs: Vec<_> = displayed.iter().map(|p| p.avg()).collect();
    let mut mins: Vec<_> = displayed.iter().map(|p| p.min).collect();
    let mut maxs: Vec<_> = displayed.iter().map(|p| p.max).collect();
    let mut counts: Vec<_> = displayed.iter().map(|p| p.count).collect();

    // Compute cutoffs (smallest value among the top 10% when sorted descending)
    let totals_cutoff = if warn_count == 0 || totals.is_empty() {
        None
    } else {
        totals.sort_by(|a, b| b.cmp(a));
        totals.get(warn_count - 1).copied()
    };

    let avgs_cutoff = if warn_count == 0 || avgs.is_empty() {
        None
    } else {
        avgs.sort_by(|a, b| b.cmp(a));
        avgs.get(warn_count - 1).copied()
    };

    let mins_cutoff = if warn_count == 0 || mins.is_empty() {
        None
    } else {
        mins.sort_by(|a, b| b.cmp(a));
        mins.get(warn_count - 1).copied()
    };

    let maxs_cutoff = if warn_count == 0 || maxs.is_empty() {
        None
    } else {
        maxs.sort_by(|a, b| b.cmp(a));
        maxs.get(warn_count - 1).copied()
    };

    let counts_cutoff = if warn_count == 0 || counts.is_empty() {
        None
    } else {
        counts.sort_by(|a, b| b.cmp(a));
        counts.get(warn_count - 1).copied()
    };

    for p in profiles.into_iter().take(limit) {
        let total_str = format!("{:>10}", fmt_dur(p.total));
        let avg_str = format!("{:>10}", fmt_dur(p.avg()));
        let min_str = format!("{:>10}", fmt_dur(p.min));
        let max_str = format!("{:>10}", fmt_dur(p.max));
        let count_str = format!("{:>1$}", p.count, count_column_width);

        let total_markup = if totals_cutoff.is_some_and(|c| p.total >= c) {
            markup! { <Warn>{total_str}</Warn> }
        } else {
            markup! { {total_str} }
        };

        let avg_markup = if avgs_cutoff.is_some_and(|c| p.avg() >= c) {
            markup! { <Warn>{avg_str}</Warn> }
        } else {
            markup! { {avg_str} }
        };

        let min_markup = if mins_cutoff.is_some_and(|c| p.min >= c) {
            markup! { <Warn>{min_str}</Warn> }
        } else {
            markup! { {min_str} }
        };

        let max_markup = if maxs_cutoff.is_some_and(|c| p.max >= c) {
            markup! { <Warn>{max_str}</Warn> }
        } else {
            markup! { {max_str} }
        };

        let count_markup = if counts_cutoff.is_some_and(|c| p.count >= c) {
            markup! { <Warn>{count_str}</Warn> }
        } else {
            markup! { {count_str} }
        };

        let row = markup! {
            "  "
            {total_markup} "  "
            {avg_markup} "  "
            {min_markup} "  "
            {max_markup} "  "
            {count_markup} "  "
            <Info>{p.label.to_string()}</Info> "\n"
        };

        out.extend_with(row);
    }

    // Return the composed Markup
    out
}

fn fmt_dur(d: Duration) -> String {
    format!("{:.3?}", d)
}
