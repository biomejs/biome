use std::{
    any::Any,
    cell::Cell,
    panic::{PanicHookInfo, UnwindSafe},
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

type PanicHook = dyn for<'a> Fn(&PanicHookInfo<'a>) + Send + Sync + 'static;

#[derive(Default, Debug)]
pub struct PanicError {
    pub info: String,
    pub backtrace: Option<std::backtrace::Backtrace>,
}

thread_local! {
    static CAPTURE_DEPTH: Cell<usize> = const { Cell::new(0) };
    static LAST_PANIC: Cell<Option<PanicError>> = const { Cell::new(None) };
}

static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
static HOOK_LOCK: Mutex<()> = Mutex::new(());

impl std::fmt::Display for PanicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = f.write_fmt(format_args!("{}\n", self.info));
        if let Some(backtrace) = &self.backtrace {
            f.write_fmt(format_args!("Backtrace: {backtrace}"))
        } else {
            result
        }
    }
}

fn panic_error_from_info(info: &PanicHookInfo<'_>) -> PanicError {
    PanicError {
        info: info.to_string(),
        backtrace: Some(std::backtrace::Backtrace::capture()),
    }
}

fn panic_error_from_payload(payload: &(dyn Any + Send)) -> PanicError {
    let info = if let Some(message) = payload.downcast_ref::<&'static str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "panic with a non-string payload".to_string()
    };

    PanicError {
        info,
        backtrace: None,
    }
}

fn capture_panic(info: &PanicHookInfo<'_>) -> bool {
    CAPTURE_DEPTH.with(|depth| {
        if depth.get() == 0 {
            return false;
        }

        LAST_PANIC.with(|cell| cell.set(Some(panic_error_from_info(info))));
        true
    })
}

fn dispatching_hook(fallback: Box<PanicHook>) -> Box<PanicHook> {
    Box::new(move |info| {
        if !capture_panic(info) {
            fallback(info);
        }
    })
}

fn ensure_hook_installed() {
    if HOOK_INSTALLED.load(Ordering::Acquire) {
        return;
    }

    let _guard = HOOK_LOCK.lock().unwrap_or_else(|error| error.into_inner());
    if HOOK_INSTALLED.load(Ordering::Relaxed) {
        return;
    }

    let fallback = std::panic::take_hook();
    std::panic::set_hook(dispatching_hook(fallback));
    HOOK_INSTALLED.store(true, Ordering::Release);
}

/// Installs a process-wide panic hook that cooperates with [`catch_unwind`].
///
/// Panics caught by [`catch_unwind`] are recorded on the panicking thread.
/// Other panics are forwarded to `hook`.
pub fn set_hook(hook: Box<PanicHook>) {
    let _guard = HOOK_LOCK.lock().unwrap_or_else(|error| error.into_inner());
    std::panic::set_hook(dispatching_hook(hook));
    HOOK_INSTALLED.store(true, Ordering::Release);
}

struct CaptureGuard;

impl CaptureGuard {
    fn new() -> Self {
        CAPTURE_DEPTH.with(|depth| depth.set(depth.get() + 1));
        Self
    }
}

impl Drop for CaptureGuard {
    fn drop(&mut self) {
        CAPTURE_DEPTH.with(|depth| {
            let current = depth.get();
            debug_assert!(current > 0);
            depth.set(current.saturating_sub(1));
        });
    }
}

/// Runs `f` and returns information about any unwinding panic it raises.
///
/// The fallback panic hook is not invoked for a panic caught on the current
/// thread. Panics on other threads continue to use the fallback hook.
pub fn catch_unwind<F, R>(f: F) -> Result<R, PanicError>
where
    F: FnOnce() -> R + UnwindSafe,
{
    ensure_hook_installed();
    LAST_PANIC.with(Cell::take);

    let guard = CaptureGuard::new();
    let result = std::panic::catch_unwind(f);
    drop(guard);

    match result {
        Ok(value) => {
            LAST_PANIC.with(Cell::take);
            Ok(value)
        }
        Err(payload) => Err(LAST_PANIC
            .with(Cell::take)
            .unwrap_or_else(|| panic_error_from_payload(payload.as_ref()))),
    }
}

#[cfg(test)]
mod tests {
    use super::catch_unwind;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn captures_concurrent_panics_on_their_originating_threads() {
        let (first_ready_tx, first_ready_rx) = mpsc::channel();
        let (first_go_tx, first_go_rx) = mpsc::channel();
        let (first_done_tx, first_done_rx) = mpsc::channel();

        let first = thread::spawn(move || {
            let result = catch_unwind(|| {
                first_ready_tx.send(()).unwrap();
                first_go_rx.recv().unwrap();
                panic!("first panic");
            });
            first_done_tx.send(()).unwrap();
            result
        });

        first_ready_rx.recv().unwrap();

        let (second_ready_tx, second_ready_rx) = mpsc::channel();
        let (second_go_tx, second_go_rx) = mpsc::channel();
        let second = thread::spawn(move || {
            catch_unwind(|| {
                second_ready_tx.send(()).unwrap();
                second_go_rx.recv().unwrap();
                panic!("second panic");
            })
        });

        second_ready_rx.recv().unwrap();
        first_go_tx.send(()).unwrap();
        first_done_rx.recv().unwrap();
        second_go_tx.send(()).unwrap();

        let first_error = first.join().unwrap().unwrap_err();
        let second_error = second.join().unwrap().unwrap_err();

        assert!(first_error.info.contains("first panic"));
        assert!(second_error.info.contains("second panic"));
    }
}
