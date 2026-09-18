//! Opt-in profiling for analyzer-facing type-inference requests.
//!
//! A **request** is one operation started by an analyzer, including all database
//! work it calls. A **query** record is created when a tracked database query
//! body actually runs; a cached result does not run the body. A **whole-module**
//! record marks a request that had to infer complete type tables instead of
//! answering with individual lookups.
//!
//! Timings are inclusive: a request's time includes its queries, and a query's
//! time includes nested queries. These values must not be added together.
//! **Aborted** means the operation unwound before completion. **Breadth** counts
//! modules and type-table slots visited by completed whole-module operations.
//! The request **origin** is the analyzer's source range; the **trigger** is the
//! narrower query location that caused whole-module inference when one exists.
//!
//! Recording state and metrics are shared by the process. Thread-local stacks
//! connect nested operations to the request running on each thread. Document
//! paths are captured before tracked queries run so profiling does not create
//! extra database dependencies.

mod display;

pub use display::DisplayTypeInferenceProfile;

use std::cell::{Cell, RefCell};
use std::cmp;
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

use crate::{ModuleDb, ModuleInfo, ModuleInfoKind};
use biome_rowan::TextRange;
use parking_lot::Mutex;

use super::{
    TypeInferenceCaller, TypeInferenceCodeReference, TypeInferenceRequestMetadata,
    TypeInferenceRequestOrigin,
};
pub(crate) use super::{TypeInferenceQueryKind, TypeInferenceWholeModuleReason};

const MAX_PROFILE_KEYS: usize = 10_000;
const MAX_DOCUMENTS: usize = 10_000;
static RECORDING: AtomicBool = AtomicBool::new(false);
static PROFILER: LazyLock<Mutex<TypeInferenceProfiler>> =
    LazyLock::new(|| Mutex::new(TypeInferenceProfiler::default()));
static PROFILE_DOCUMENT_INITIALIZATION: Mutex<()> = Mutex::new(());

thread_local! {
    static REQUEST_STACK: RefCell<Vec<RequestKey>> = const { RefCell::new(Vec::new()) };
    static QUERY_STACK: RefCell<Vec<RecordedLocation>> = const { RefCell::new(Vec::new()) };
    static WHOLE_MODULE_STACK: RefCell<Vec<WholeModuleFrame>> = const { RefCell::new(Vec::new()) };
    static PROFILING_OVERHEAD: Cell<Duration> = const { Cell::new(Duration::ZERO) };
}

/// Source attribution available at a profiled query or whole-module boundary.
#[derive(Clone, Copy)]
pub(crate) enum TypeInferenceProfileOrigin {
    /// Source range supplied by the profiled operation.
    Exact(TypeInferenceRequestOrigin),
    /// Source range inherited from the enclosing analyzer request.
    Inherited,
    /// Module-wide operation without a narrower source range.
    Document(ModuleInfo),
}

impl TypeInferenceProfileOrigin {
    pub(crate) const fn exact(module: ModuleInfo, range: TextRange) -> Self {
        Self::Exact(TypeInferenceRequestOrigin::new(module, range))
    }

    pub(crate) const fn document(module: ModuleInfo) -> Self {
        Self::Document(module)
    }
}

/// Whether a location came from its input or enclosing request.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum TypeInferenceLocationAttribution {
    /// The profiled operation supplied its own module and source range.
    Exact,
    /// The operation inherited the module and range from its enclosing request.
    RequestOrigin,
    /// The operation identified a module without a narrower source range.
    Document,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct RecordedLocation {
    module: ModuleInfo,
    range: Option<TextRange>,
    attribution: TypeInferenceLocationAttribution,
}

impl RecordedLocation {
    const fn exact(origin: TypeInferenceRequestOrigin) -> Self {
        Self {
            module: origin.module(),
            range: Some(origin.range()),
            attribution: TypeInferenceLocationAttribution::Exact,
        }
    }

    const fn inherited(origin: TypeInferenceRequestOrigin) -> Self {
        Self {
            module: origin.module(),
            range: Some(origin.range()),
            attribution: TypeInferenceLocationAttribution::RequestOrigin,
        }
    }

    const fn document(module: ModuleInfo) -> Self {
        Self {
            module,
            range: None,
            attribution: TypeInferenceLocationAttribution::Document,
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct RequestMetadata {
    id: &'static str,
    label: &'static str,
}

impl RequestMetadata {
    fn of<R: TypeInferenceRequestMetadata>() -> Self {
        Self {
            id: R::ID,
            label: R::LABEL,
        }
    }

    const fn id(self) -> &'static str {
        self.id
    }

    const fn label(self) -> &'static str {
        self.label
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct RequestKey {
    caller: TypeInferenceCaller,
    metadata: RequestMetadata,
    origin: TypeInferenceRequestOrigin,
    implementation: TypeInferenceCodeReference,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct QueryKey {
    kind: TypeInferenceQueryKind,
    request: RequestKey,
    location: RecordedLocation,
    implementation: TypeInferenceCodeReference,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct WholeModuleKey {
    reason: TypeInferenceWholeModuleReason,
    request_origin: TypeInferenceRequestOrigin,
    trigger: RecordedLocation,
    implementation: TypeInferenceCodeReference,
}

struct DurationMetric {
    completed: u32,
    aborted: u32,
    total: Duration,
    min: Duration,
    max: Duration,
}

impl Default for DurationMetric {
    fn default() -> Self {
        Self {
            completed: 0,
            aborted: 0,
            total: Duration::ZERO,
            min: Duration::MAX,
            max: Duration::ZERO,
        }
    }
}

impl DurationMetric {
    fn record_completed(&mut self, duration: Duration) {
        self.completed = self.completed.saturating_add(1);
        self.total = self.total.saturating_add(duration);
        self.min = cmp::min(self.min, duration);
        self.max = cmp::max(self.max, duration);
    }

    fn record_aborted(&mut self) {
        self.aborted = self.aborted.saturating_add(1);
    }

    fn min(&self) -> Duration {
        if self.completed == 0 {
            Duration::ZERO
        } else {
            self.min
        }
    }

    fn average(&self) -> Duration {
        if self.completed == 0 {
            Duration::ZERO
        } else {
            self.total / self.completed
        }
    }
}

#[derive(Default)]
struct BreadthMetric {
    total: u64,
    min: u64,
    max: u64,
    count: u32,
}

impl BreadthMetric {
    fn record(&mut self, value: u64) {
        self.total = self.total.saturating_add(value);
        self.min = if self.count == 0 {
            value
        } else {
            cmp::min(self.min, value)
        };
        self.max = cmp::max(self.max, value);
        self.count = self.count.saturating_add(1);
    }

    fn average(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.total as f64 / f64::from(self.count)
        }
    }
}

#[derive(Default)]
struct WholeModuleMetric {
    duration: DurationMetric,
    modules: BreadthMetric,
    type_slots: BreadthMetric,
    expression_slots: BreadthMetric,
    binding_slots: BreadthMetric,
    cycle_recoveries: u32,
}

struct DocumentInfo {
    path: Box<str>,
}

#[derive(Default)]
struct TypeInferenceProfiler {
    documents: Option<HashMap<ModuleInfo, DocumentInfo>>,
    requests: HashMap<RequestKey, DurationMetric>,
    queries: HashMap<QueryKey, DurationMetric>,
    whole_module_inferences: HashMap<WholeModuleKey, WholeModuleMetric>,
    dropped_request_keys: u32,
    dropped_query_keys: u32,
    dropped_whole_module_keys: u32,
    dropped_documents: u32,
}

impl TypeInferenceProfiler {
    fn request_metric(&mut self, key: RequestKey) -> Option<&mut DurationMetric> {
        if !self.requests.contains_key(&key) && self.requests.len() >= MAX_PROFILE_KEYS {
            self.dropped_request_keys = self.dropped_request_keys.saturating_add(1);
            return None;
        }
        Some(self.requests.entry(key).or_default())
    }

    fn query_metric(&mut self, key: &QueryKey) -> Option<&mut DurationMetric> {
        if !self.queries.contains_key(key) && self.queries.len() >= MAX_PROFILE_KEYS {
            self.dropped_query_keys = self.dropped_query_keys.saturating_add(1);
            return None;
        }
        Some(self.queries.entry(*key).or_default())
    }

    fn whole_module_metric(&mut self, key: &WholeModuleKey) -> Option<&mut WholeModuleMetric> {
        if !self.whole_module_inferences.contains_key(key)
            && self.whole_module_inferences.len() >= MAX_PROFILE_KEYS
        {
            self.dropped_whole_module_keys = self.dropped_whole_module_keys.saturating_add(1);
            return None;
        }
        Some(self.whole_module_inferences.entry(*key).or_default())
    }

    fn resolve_location(&self, location: RecordedLocation) -> TypeInferenceProfileLocation {
        let document = self
            .documents
            .as_ref()
            .and_then(|documents| documents.get(&location.module));
        let path = document.map_or_else(
            || "<unregistered module>".into(),
            |document| document.path.clone(),
        );
        TypeInferenceProfileLocation {
            path,
            range: location.range,
            attribution: location.attribution,
        }
    }

    fn snapshot(&self) -> TypeInferenceProfileSnapshot {
        let requests = self
            .requests
            .iter()
            .map(|(key, metric)| TypeInferenceRequestProfile {
                caller: key.caller,
                metadata: key.metadata,
                location: self.resolve_location(RecordedLocation::exact(key.origin)),
                implementation: key.implementation,
                completed: metric.completed,
                aborted: metric.aborted,
                total: metric.total,
                average: metric.average(),
                min: metric.min(),
                max: metric.max,
            })
            .collect();
        let queries = self
            .queries
            .iter()
            .map(|(key, metric)| TypeInferenceQueryProfile {
                kind: key.kind,
                request: key.request.metadata,
                caller: key.request.caller,
                location: self.resolve_location(key.location),
                implementation: key.implementation,
                completed: metric.completed,
                aborted: metric.aborted,
                total: metric.total,
                average: metric.average(),
                min: metric.min(),
                max: metric.max,
            })
            .collect();
        let whole_module_inferences = self
            .whole_module_inferences
            .iter()
            .map(|(key, metric)| TypeInferenceWholeModuleProfile {
                reason: key.reason,
                root: self.resolve_location(RecordedLocation::exact(key.request_origin)),
                trigger: self.resolve_location(key.trigger),
                implementation: key.implementation,
                completed: metric.duration.completed,
                aborted: metric.duration.aborted,
                total: metric.duration.total,
                average: metric.duration.average(),
                min: metric.duration.min(),
                max: metric.duration.max,
                modules: breadth_profile(&metric.modules),
                type_slots: breadth_profile(&metric.type_slots),
                expression_slots: breadth_profile(&metric.expression_slots),
                binding_slots: breadth_profile(&metric.binding_slots),
                cycle_recoveries: metric.cycle_recoveries,
            })
            .collect();

        TypeInferenceProfileSnapshot {
            requests,
            queries,
            whole_module_inferences,
            dropped_request_keys: self.dropped_request_keys,
            dropped_query_keys: self.dropped_query_keys,
            dropped_whole_module_keys: self.dropped_whole_module_keys,
            dropped_documents: self.dropped_documents,
        }
    }
}

fn breadth_profile(metric: &BreadthMetric) -> TypeInferenceBreadthProfile {
    TypeInferenceBreadthProfile {
        average: metric.average(),
        min: metric.min,
        max: metric.max,
    }
}

fn with_profiler<R>(f: impl FnOnce(&mut TypeInferenceProfiler) -> R) -> Option<R> {
    #[cfg(not(target_arch = "wasm32"))]
    let start = Instant::now();
    let mut profiler = PROFILER.lock();
    let result = is_recording().then(|| f(&mut profiler));
    #[cfg(not(target_arch = "wasm32"))]
    PROFILING_OVERHEAD.with(|overhead| {
        overhead.set(overhead.get().saturating_add(start.elapsed()));
    });
    result
}

#[cfg(not(target_arch = "wasm32"))]
fn elapsed_without_profiling_overhead(start: Instant, overhead_at_start: Duration) -> Duration {
    let overhead = PROFILING_OVERHEAD.with(Cell::get);
    start
        .elapsed()
        .saturating_sub(overhead.saturating_sub(overhead_at_start))
}

fn current_request() -> Option<RequestKey> {
    if !is_recording() {
        return None;
    }
    REQUEST_STACK.with(|stack| stack.borrow().last().copied())
}

fn resolve_profile_origin(
    location: TypeInferenceProfileOrigin,
    request: RequestKey,
) -> RecordedLocation {
    match location {
        TypeInferenceProfileOrigin::Exact(origin) => RecordedLocation::exact(origin),
        TypeInferenceProfileOrigin::Inherited => RecordedLocation::inherited(request.origin),
        TypeInferenceProfileOrigin::Document(module) => RecordedLocation::document(module),
    }
}

fn current_trigger(request: RequestKey) -> RecordedLocation {
    QUERY_STACK
        .with(|stack| stack.borrow().last().copied())
        .unwrap_or_else(|| RecordedLocation::inherited(request.origin))
}

#[inline]
pub(crate) fn is_recording() -> bool {
    RECORDING.load(Ordering::Acquire)
}

fn profile_documents_are_initialized() -> bool {
    with_profiler(|profiler| profiler.documents.is_some()).unwrap_or(false)
}

/// Captures document paths before tracked query instrumentation can run.
///
/// Reading a `ModuleInfo` path from a tracked query would add a Salsa dependency.
/// Capturing the complete map at the outer request boundary also gives imported
/// modules path attribution without reading their paths from nested queries.
fn initialize_profile_documents(db: &dyn ModuleDb) -> bool {
    if profile_documents_are_initialized() {
        return true;
    }

    let initialization_guard = PROFILE_DOCUMENT_INITIALIZATION.lock();
    if profile_documents_are_initialized() {
        return true;
    }
    if !is_recording() {
        return false;
    }

    let (documents, dropped_documents) = collect_profile_documents(db);
    let initialized = with_profiler(|profiler| {
        profiler.documents = Some(documents);
        profiler.dropped_documents = dropped_documents;
        true
    })
    .unwrap_or(false);
    drop(initialization_guard);
    initialized
}

fn collect_profile_documents(db: &dyn ModuleDb) -> (HashMap<ModuleInfo, DocumentInfo>, u32) {
    let mut modules = Vec::new();
    let mut dropped_documents = 0u32;
    db.for_each_module(&mut |module| {
        if matches!(module.kind(db), ModuleInfoKind::Js(_)) {
            if modules.len() < MAX_DOCUMENTS {
                modules.push(module);
            } else {
                dropped_documents = dropped_documents.saturating_add(1);
            }
        }
    });
    let documents = modules
        .into_iter()
        .map(|module| {
            let path = module.path(db).as_str().into();
            (module, DocumentInfo { path })
        })
        .collect();
    (documents, dropped_documents)
}

/// Guard for the process-wide type-inference profiler.
///
/// Callers must keep at most one guard alive and keep it alive until all
/// profiled work on every thread has finished. The type system does not enforce
/// this invariant. Starting another guard resets shared metrics, and dropping
/// either guard disables recording for the entire process. Dropping a guard also
/// discards undrained metrics and clears profiling stacks on the thread that
/// drops it; no profiling scope may remain active on another thread at that
/// point.
pub struct TypeInferenceProfilerGuard(());

impl TypeInferenceProfilerGuard {
    /// Enables process-wide recording after discarding any existing metrics.
    ///
    /// The returned guard must outlive all work intended for the profile. This
    /// function does not reject a second live guard; callers must uphold the
    /// single-guard invariant documented on [`TypeInferenceProfilerGuard`].
    pub fn start() -> Self {
        let mut profiler = PROFILER.lock();
        *profiler = TypeInferenceProfiler::default();
        RECORDING.store(true, Ordering::Release);
        Self(())
    }

    /// Returns the recorded metrics and resets the profiler for another interval.
    ///
    /// Records in the snapshot are sorted by total time. Recording remains
    /// enabled, so later requests accumulate in a new profile. If recording was
    /// already disabled, this returns an empty snapshot.
    pub fn drain(&self) -> TypeInferenceProfileSnapshot {
        if !is_recording() {
            return TypeInferenceProfileSnapshot::default();
        }
        let mut snapshot = with_profiler(|profiler| {
            let snapshot = profiler.snapshot();
            *profiler = TypeInferenceProfiler::default();
            snapshot
        })
        .unwrap_or_default();
        snapshot.sort_by_total();
        snapshot
    }
}

impl Drop for TypeInferenceProfilerGuard {
    fn drop(&mut self) {
        clear_profiler();
    }
}

fn clear_profiler() {
    let mut profiler = PROFILER.lock();
    RECORDING.store(false, Ordering::Release);
    *profiler = TypeInferenceProfiler::default();
    REQUEST_STACK.with(|stack| stack.borrow_mut().clear());
    QUERY_STACK.with(|stack| stack.borrow_mut().clear());
    WHOLE_MODULE_STACK.with(|stack| stack.borrow_mut().clear());
    PROFILING_OVERHEAD.with(|overhead| overhead.set(Duration::ZERO));
}

struct TypeInferenceRequestProfileGuard {
    key: Option<RequestKey>,
    #[cfg(not(target_arch = "wasm32"))]
    start: Option<Instant>,
    #[cfg(not(target_arch = "wasm32"))]
    overhead_at_start: Duration,
    completed: bool,
}

impl TypeInferenceRequestProfileGuard {
    fn complete(mut self) {
        self.completed = true;
    }
}

impl Drop for TypeInferenceRequestProfileGuard {
    fn drop(&mut self) {
        let Some(key) = self.key.take() else {
            return;
        };
        let popped = REQUEST_STACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            if stack.last() == Some(&key) {
                stack.pop()
            } else {
                None
            }
        });
        if is_recording() {
            debug_assert!(popped == Some(key));
        }
        #[cfg(not(target_arch = "wasm32"))]
        let elapsed = self.start.take().map_or(Duration::ZERO, |start| {
            elapsed_without_profiling_overhead(start, self.overhead_at_start)
        });
        with_profiler(|profiler| {
            let Some(metric) = profiler.request_metric(key) else {
                return;
            };
            if self.completed {
                #[cfg(not(target_arch = "wasm32"))]
                metric.record_completed(elapsed);
                #[cfg(target_arch = "wasm32")]
                metric.record_completed(Duration::ZERO);
            } else {
                metric.record_aborted();
            }
        });
    }
}

/// Executes one analyzer-facing request while profiling is active.
///
/// This outer boundary initializes document paths before `operation` enters any
/// tracked query. Unwinding records an aborted request before leaving the
/// synchronous profiling scope.
pub(crate) fn profile_request<R, T>(
    db: &dyn ModuleDb,
    caller: TypeInferenceCaller,
    origin: TypeInferenceRequestOrigin,
    implementation: TypeInferenceCodeReference,
    operation: impl FnOnce() -> T,
) -> T
where
    R: TypeInferenceRequestMetadata,
{
    if !is_recording() || !initialize_profile_documents(db) {
        return operation();
    }
    let key = RequestKey {
        caller,
        metadata: RequestMetadata::of::<R>(),
        origin,
        implementation,
    };
    REQUEST_STACK.with(|stack| stack.borrow_mut().push(key));
    let guard = TypeInferenceRequestProfileGuard {
        key: Some(key),
        #[cfg(not(target_arch = "wasm32"))]
        start: Some(Instant::now()),
        #[cfg(not(target_arch = "wasm32"))]
        overhead_at_start: PROFILING_OVERHEAD.with(Cell::get),
        completed: false,
    };
    let output = operation();
    guard.complete();
    output
}

struct TypeInferenceQueryProfileGuard {
    key: Option<QueryKey>,
    #[cfg(not(target_arch = "wasm32"))]
    start: Option<Instant>,
    #[cfg(not(target_arch = "wasm32"))]
    overhead_at_start: Duration,
    completed: bool,
}

impl TypeInferenceQueryProfileGuard {
    fn complete(mut self) {
        self.completed = true;
    }
}

impl Drop for TypeInferenceQueryProfileGuard {
    fn drop(&mut self) {
        let Some(key) = self.key.take() else {
            return;
        };
        let stack_entry = key.location;
        let popped = QUERY_STACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            if stack.last() == Some(&stack_entry) {
                stack.pop()
            } else {
                None
            }
        });
        if is_recording() {
            debug_assert!(popped == Some(stack_entry));
        }
        #[cfg(not(target_arch = "wasm32"))]
        let elapsed = self.start.take().map_or(Duration::ZERO, |start| {
            elapsed_without_profiling_overhead(start, self.overhead_at_start)
        });
        with_profiler(|profiler| {
            let Some(metric) = profiler.query_metric(&key) else {
                return;
            };
            if self.completed {
                #[cfg(not(target_arch = "wasm32"))]
                metric.record_completed(elapsed);
                #[cfg(target_arch = "wasm32")]
                metric.record_completed(Duration::ZERO);
            } else {
                metric.record_aborted();
            }
        });
    }
}

#[track_caller]
pub(crate) fn execute_query<T>(
    kind: TypeInferenceQueryKind,
    location: TypeInferenceProfileOrigin,
    symbol: &'static str,
    operation: impl FnOnce() -> T,
) -> T {
    if !is_recording() {
        return operation();
    }
    let Some(request) = current_request() else {
        return operation();
    };
    let caller = std::panic::Location::caller();
    let implementation = TypeInferenceCodeReference::new(caller.file(), caller.line(), symbol);
    let location = resolve_profile_origin(location, request);
    let key = QueryKey {
        kind,
        request,
        location,
        implementation,
    };
    QUERY_STACK.with(|stack| stack.borrow_mut().push(location));
    let guard = TypeInferenceQueryProfileGuard {
        key: Some(key),
        #[cfg(not(target_arch = "wasm32"))]
        start: Some(Instant::now()),
        #[cfg(not(target_arch = "wasm32"))]
        overhead_at_start: PROFILING_OVERHEAD.with(Cell::get),
        completed: false,
    };
    let output = operation();
    guard.complete();
    output
}

#[derive(Default)]
struct WholeModuleBreadth {
    modules: u64,
    type_slots: u64,
    expression_slots: u64,
    binding_slots: u64,
    cycle_recoveries: u32,
}

struct WholeModuleFrame {
    key: WholeModuleKey,
    breadth: WholeModuleBreadth,
}

/// RAII scope for one complete-module inference operation.
pub(crate) struct TypeInferenceWholeModuleGuard {
    key: Option<WholeModuleKey>,
    #[cfg(not(target_arch = "wasm32"))]
    start: Option<Instant>,
    #[cfg(not(target_arch = "wasm32"))]
    overhead_at_start: Duration,
    completed: bool,
}

impl TypeInferenceWholeModuleGuard {
    fn disabled() -> Self {
        Self {
            key: None,
            #[cfg(not(target_arch = "wasm32"))]
            start: None,
            #[cfg(not(target_arch = "wasm32"))]
            overhead_at_start: Duration::ZERO,
            completed: false,
        }
    }

    pub(crate) fn complete(mut self) {
        self.completed = true;
    }
}

impl Drop for TypeInferenceWholeModuleGuard {
    fn drop(&mut self) {
        let Some(key) = self.key.take() else {
            return;
        };
        let Some(frame) = WHOLE_MODULE_STACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            if stack.last().map(|frame| frame.key) == Some(key) {
                stack.pop()
            } else {
                None
            }
        }) else {
            return;
        };
        #[cfg(not(target_arch = "wasm32"))]
        let elapsed = self.start.take().map_or(Duration::ZERO, |start| {
            elapsed_without_profiling_overhead(start, self.overhead_at_start)
        });
        with_profiler(|profiler| {
            let Some(metric) = profiler.whole_module_metric(&frame.key) else {
                return;
            };
            if self.completed {
                #[cfg(not(target_arch = "wasm32"))]
                metric.duration.record_completed(elapsed);
                #[cfg(target_arch = "wasm32")]
                metric.duration.record_completed(Duration::ZERO);
                metric.modules.record(frame.breadth.modules);
                metric.type_slots.record(frame.breadth.type_slots);
                metric
                    .expression_slots
                    .record(frame.breadth.expression_slots);
                metric.binding_slots.record(frame.breadth.binding_slots);
                metric.cycle_recoveries = metric
                    .cycle_recoveries
                    .saturating_add(frame.breadth.cycle_recoveries);
            } else {
                metric.duration.record_aborted();
            }
        });
    }
}

#[track_caller]
pub(crate) fn start_whole_module_inference(
    reason: TypeInferenceWholeModuleReason,
    trigger: TypeInferenceProfileOrigin,
    symbol: &'static str,
) -> TypeInferenceWholeModuleGuard {
    let caller = std::panic::Location::caller();
    start_whole_module_inference_at(
        reason,
        trigger,
        TypeInferenceCodeReference::new(caller.file(), caller.line(), symbol),
    )
}

pub(crate) fn start_whole_module_inference_at(
    reason: TypeInferenceWholeModuleReason,
    trigger: TypeInferenceProfileOrigin,
    implementation: TypeInferenceCodeReference,
) -> TypeInferenceWholeModuleGuard {
    if !is_recording() {
        return TypeInferenceWholeModuleGuard::disabled();
    }
    let Some(request) = current_request() else {
        return TypeInferenceWholeModuleGuard::disabled();
    };
    if WHOLE_MODULE_STACK.with(|stack| !stack.borrow().is_empty()) {
        return TypeInferenceWholeModuleGuard::disabled();
    }
    let key = WholeModuleKey {
        reason,
        request_origin: request.origin,
        trigger: match trigger {
            TypeInferenceProfileOrigin::Inherited => current_trigger(request),
            trigger => resolve_profile_origin(trigger, request),
        },
        implementation,
    };
    WHOLE_MODULE_STACK.with(|stack| {
        stack.borrow_mut().push(WholeModuleFrame {
            key,
            breadth: WholeModuleBreadth::default(),
        });
    });
    TypeInferenceWholeModuleGuard {
        key: Some(key),
        #[cfg(not(target_arch = "wasm32"))]
        start: Some(Instant::now()),
        #[cfg(not(target_arch = "wasm32"))]
        overhead_at_start: PROFILING_OVERHEAD.with(Cell::get),
        completed: false,
    }
}

pub(crate) fn record_inferred_module(
    type_slots: usize,
    expression_slots: usize,
    binding_slots: usize,
) {
    if !is_recording() {
        return;
    }
    WHOLE_MODULE_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        let Some(frame) = stack.last_mut() else {
            return;
        };
        frame.breadth.modules = frame.breadth.modules.saturating_add(1);
        frame.breadth.type_slots = frame
            .breadth
            .type_slots
            .saturating_add(u64::try_from(type_slots).unwrap_or(u64::MAX));
        frame.breadth.expression_slots = frame
            .breadth
            .expression_slots
            .saturating_add(u64::try_from(expression_slots).unwrap_or(u64::MAX));
        frame.breadth.binding_slots = frame
            .breadth
            .binding_slots
            .saturating_add(u64::try_from(binding_slots).unwrap_or(u64::MAX));
    });
}

pub(crate) fn record_cycle_recovery() {
    if !is_recording() {
        return;
    }
    WHOLE_MODULE_STACK.with(|stack| {
        if let Some(frame) = stack.borrow_mut().last_mut() {
            frame.breadth.cycle_recoveries = frame.breadth.cycle_recoveries.saturating_add(1);
        }
    });
}

/// Resolved source location in a profile snapshot.
#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct TypeInferenceProfileLocation {
    path: Box<str>,
    range: Option<TextRange>,
    attribution: TypeInferenceLocationAttribution,
}

struct TypeInferenceRequestProfile {
    caller: TypeInferenceCaller,
    metadata: RequestMetadata,
    location: TypeInferenceProfileLocation,
    implementation: TypeInferenceCodeReference,
    completed: u32,
    aborted: u32,
    total: Duration,
    average: Duration,
    min: Duration,
    max: Duration,
}

struct TypeInferenceQueryProfile {
    kind: TypeInferenceQueryKind,
    request: RequestMetadata,
    caller: TypeInferenceCaller,
    location: TypeInferenceProfileLocation,
    implementation: TypeInferenceCodeReference,
    completed: u32,
    aborted: u32,
    total: Duration,
    average: Duration,
    min: Duration,
    max: Duration,
}

#[derive(Clone, Copy)]
struct TypeInferenceBreadthProfile {
    average: f64,
    min: u64,
    max: u64,
}

struct TypeInferenceWholeModuleProfile {
    reason: TypeInferenceWholeModuleReason,
    root: TypeInferenceProfileLocation,
    trigger: TypeInferenceProfileLocation,
    implementation: TypeInferenceCodeReference,
    completed: u32,
    aborted: u32,
    total: Duration,
    average: Duration,
    min: Duration,
    max: Duration,
    modules: TypeInferenceBreadthProfile,
    type_slots: TypeInferenceBreadthProfile,
    expression_slots: TypeInferenceBreadthProfile,
    binding_slots: TypeInferenceBreadthProfile,
    cycle_recoveries: u32,
}

#[derive(Default)]
pub struct TypeInferenceProfileSnapshot {
    requests: Vec<TypeInferenceRequestProfile>,
    queries: Vec<TypeInferenceQueryProfile>,
    whole_module_inferences: Vec<TypeInferenceWholeModuleProfile>,
    dropped_request_keys: u32,
    dropped_query_keys: u32,
    dropped_whole_module_keys: u32,
    dropped_documents: u32,
}

impl TypeInferenceProfileSnapshot {
    fn is_empty(&self) -> bool {
        self.requests.is_empty()
            && self.queries.is_empty()
            && self.whole_module_inferences.is_empty()
    }

    fn sort_by_total(&mut self) {
        self.requests.sort_by(|left, right| {
            right
                .total
                .cmp(&left.total)
                .then_with(|| left.metadata.cmp(&right.metadata))
                .then_with(|| left.caller.cmp(&right.caller))
                .then_with(|| left.location.cmp(&right.location))
        });
        self.queries.sort_by(|left, right| {
            right
                .total
                .cmp(&left.total)
                .then_with(|| left.kind.cmp(&right.kind))
                .then_with(|| left.implementation.cmp(&right.implementation))
                .then_with(|| left.request.cmp(&right.request))
                .then_with(|| left.caller.cmp(&right.caller))
                .then_with(|| left.location.cmp(&right.location))
        });
        self.whole_module_inferences.sort_by(|left, right| {
            right
                .total
                .cmp(&left.total)
                .then_with(|| left.reason.cmp(&right.reason))
                .then_with(|| left.root.cmp(&right.root))
                .then_with(|| left.trigger.cmp(&right.trigger))
        });
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{BreadthMetric, DurationMetric};

    #[test]
    fn duration_and_breadth_metrics_exclude_aborted_work() {
        let mut duration = DurationMetric::default();
        duration.record_completed(Duration::from_millis(1));
        duration.record_aborted();
        duration.record_completed(Duration::from_millis(3));
        assert_eq!(duration.completed, 2);
        assert_eq!(duration.aborted, 1);
        assert_eq!(duration.min(), Duration::from_millis(1));
        assert_eq!(duration.average(), Duration::from_millis(2));
        assert_eq!(duration.max, Duration::from_millis(3));

        let mut breadth = BreadthMetric::default();
        breadth.record(2);
        breadth.record(4);
        assert_eq!(breadth.min, 2);
        assert_eq!(breadth.average(), 3.0);
        assert_eq!(breadth.max, 4);
    }
}
