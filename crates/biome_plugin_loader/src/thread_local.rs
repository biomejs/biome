use parking_lot::Mutex;
use std::cell::{RefCell, RefMut};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::{self, ThreadId};

#[cfg(windows)]
mod platform {
    use std::ffi::c_void;
    use std::marker::PhantomData;
    use windows::Win32::System::Threading as win32;

    pub(super) struct Key<T> {
        inner: u32,
        _phantom: PhantomData<fn() -> T>,
    }

    impl<T> Key<T> {
        pub(super) unsafe fn new() -> Self {
            let inner = unsafe { win32::FlsAlloc(None) };
            // FlsAlloc returns FLS_OUT_OF_INDEXES (u32::MAX) on failure.
            assert!(inner != u32::MAX, "FlsAlloc failed: out of FLS indexes");

            Self {
                inner,
                _phantom: PhantomData,
            }
        }
        pub(super) unsafe fn get(&self) -> *mut T {
            unsafe { win32::FlsGetValue(self.inner) as *mut T }
        }

        pub(super) unsafe fn set(&self, value: *mut T) {
            let result = unsafe { win32::FlsSetValue(self.inner, Some(value as *const c_void)) };

            debug_assert!(result.is_ok());
        }
    }

    impl<T> Drop for Key<T> {
        fn drop(&mut self) {
            let result = unsafe { win32::FlsFree(self.inner) };

            debug_assert!(result.is_ok());
        }
    }
}

#[cfg(unix)]
mod platform {
    use std::ffi::c_void;
    use std::marker::PhantomData;
    use std::mem::MaybeUninit;

    pub(super) struct Key<T> {
        inner: libc::pthread_key_t,
        _phantom: PhantomData<fn() -> T>,
    }

    impl<T> Key<T> {
        pub(super) unsafe fn new() -> Self {
            let inner = unsafe {
                let mut inner = MaybeUninit::uninit();
                let result = libc::pthread_key_create(inner.as_mut_ptr(), None);

                assert_eq!(result, 0);

                inner.assume_init()
            };

            Self {
                inner,
                _phantom: PhantomData,
            }
        }

        pub(super) unsafe fn get(&self) -> *mut T {
            unsafe { libc::pthread_getspecific(self.inner) as *mut T }
        }

        pub(super) unsafe fn set(&self, value: *mut T) {
            let result = unsafe { libc::pthread_setspecific(self.inner, value as *mut c_void) };

            debug_assert_eq!(result, 0);
        }
    }

    impl<T> Drop for Key<T> {
        fn drop(&mut self) {
            let result = unsafe { libc::pthread_key_delete(self.inner) };

            debug_assert_eq!(result, 0);
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod platform {
    use std::cell::Cell;
    use std::marker::PhantomData;

    /// Fallback for targets without native TLS, i.e. `wasm32-unknown-unknown`,
    /// which only ever runs on a single thread.
    pub(super) struct Key<T> {
        inner: Cell<*mut T>,
        _phantom: PhantomData<fn() -> T>,
    }

    // SAFETY: this fallback is only compiled for single-threaded targets, so
    // the cell can never actually be accessed from more than one thread.
    unsafe impl<T> Send for Key<T> {}
    unsafe impl<T> Sync for Key<T> {}

    impl<T> Key<T> {
        pub(super) unsafe fn new() -> Self {
            Self {
                inner: Cell::new(std::ptr::null_mut()),
                _phantom: PhantomData,
            }
        }

        pub(super) unsafe fn get(&self) -> *mut T {
            self.inner.get()
        }

        pub(super) unsafe fn set(&self, value: *mut T) {
            self.inner.set(value);
        }
    }
}

/// A value stored in a [`ThreadLocalCell`], together with the thread that owns it.
struct OwnedValue {
    thread: ThreadId,
    value: *mut (),
    free: unsafe fn(*mut ()),
}

// SAFETY: `value` is only freed or dereferenced on `thread`. Other threads only
// move the pointer around.
unsafe impl Send for OwnedValue {}

impl OwnedValue {
    /// # Safety
    ///
    /// Must be called on `self.thread`, at most once.
    unsafe fn free(self) {
        unsafe { (self.free)(self.value) }
    }
}

unsafe fn free_value<T>(value: *mut ()) {
    drop(unsafe { Box::from_raw(value.cast::<RefCell<T>>()) });
}

/// Values of dropped cells that belong to threads other than the one that
/// dropped the cell. Each thread frees its own values the next time it
/// accesses any [`ThreadLocalCell`].
static ORPHANED_VALUES: Mutex<Vec<OwnedValue>> = Mutex::new(Vec::new());

/// Length of [`ORPHANED_VALUES`], readable without taking the lock.
static ORPHANED_VALUES_LEN: AtomicUsize = AtomicUsize::new(0);

fn free_orphaned_values_of_current_thread() {
    if ORPHANED_VALUES_LEN.load(Ordering::Relaxed) == 0 {
        return;
    }

    let current = thread::current().id();
    let owned: Vec<OwnedValue> = {
        let mut orphaned = ORPHANED_VALUES.lock();
        let (owned, others) = std::mem::take(&mut *orphaned)
            .into_iter()
            .partition(|value| value.thread == current);
        *orphaned = others;
        ORPHANED_VALUES_LEN.store(orphaned.len(), Ordering::Relaxed);
        owned
    };

    // Values are freed outside the lock, because dropping them can take a
    // while (for example, tearing down a JS context).
    for value in owned {
        // SAFETY: `value` belongs to the current thread and was removed from
        // the list, so it can't be freed twice.
        unsafe { value.free() };
    }
}

/// Thread-local storage.
/// It uses [`Fiber Local Storage`](https://learn.microsoft.com/en-us/windows/win32/procthread/fibers#fiber-local-storage) on Windows,
/// [`pthread_setspecific(3)`](https://linux.die.net/man/3/pthread_setspecific) on Unix,
/// or a plain [`Cell`](std::cell::Cell) on single-threaded targets such as WASM.
/// Note that the inner value is not dropped on thread exit to avoid double-free after another
/// [`std::thread_local`] is dropped.
///
/// A value must be dropped by the thread that created it, because values such
/// as JS contexts rely on state that belongs to that thread. When the cell is
/// dropped, the value of the current thread is dropped immediately, and the
/// values of other threads are dropped the next time those threads access any
/// cell. The values of threads that never access a cell again are leaked.
pub(crate) struct ThreadLocalCell<T> {
    key: platform::Key<RefCell<T>>,
    values: Mutex<Vec<OwnedValue>>,
}

impl<T> ThreadLocalCell<T>
where
    T: 'static,
{
    pub(crate) fn new() -> Self {
        Self {
            key: unsafe { platform::Key::new() },
            values: Mutex::new(Vec::new()),
        }
    }

    pub(crate) fn get_mut_or_try_init<F, E>(&self, default: F) -> Result<RefMut<'_, T>, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        free_orphaned_values_of_current_thread();

        match self.get_mut() {
            Some(r) => Ok(r),
            _ => match default() {
                Ok(value) => {
                    self.set(value);
                    Ok(self.get_mut().unwrap())
                }
                Err(err) => Err(err),
            },
        }
    }

    fn get_mut(&self) -> Option<RefMut<'_, T>> {
        unsafe {
            let ptr = self.key.get();
            if ptr.is_null() {
                None
            } else {
                Some((&*ptr).borrow_mut())
            }
        }
    }

    fn set(&self, value: T) {
        let cell = Box::into_raw(Box::new(RefCell::new(value)));
        self.values.lock().push(OwnedValue {
            thread: thread::current().id(),
            value: cell.cast(),
            free: free_value::<T>,
        });
        unsafe {
            self.key.set(cell);
        }
    }
}

impl<T> Drop for ThreadLocalCell<T> {
    fn drop(&mut self) {
        let current = thread::current().id();
        let values = std::mem::take(self.values.get_mut());
        let (owned, orphaned): (Vec<_>, Vec<_>) = values
            .into_iter()
            .partition(|value| value.thread == current);

        if !orphaned.is_empty() {
            let mut orphaned_values = ORPHANED_VALUES.lock();
            orphaned_values.extend(orphaned);
            ORPHANED_VALUES_LEN.store(orphaned_values.len(), Ordering::Relaxed);
        }

        for value in owned {
            // SAFETY: `value` belongs to the current thread, and the cell
            // can't be accessed anymore.
            unsafe { value.free() };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::mpsc;

    struct DropCounter(Arc<AtomicUsize>);

    impl Drop for DropCounter {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn init(cell: &ThreadLocalCell<DropCounter>, drops: &Arc<AtomicUsize>) {
        let _ = cell
            .get_mut_or_try_init(|| Ok::<_, ()>(DropCounter(drops.clone())))
            .unwrap();
    }

    #[test]
    fn drops_value_of_current_thread_with_cell() {
        let drops = Arc::new(AtomicUsize::new(0));
        let cell = ThreadLocalCell::new();
        init(&cell, &drops);
        init(&cell, &drops);
        assert_eq!(drops.load(Ordering::SeqCst), 0);

        drop(cell);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn drops_value_of_other_thread_on_its_next_access() {
        let drops = Arc::new(AtomicUsize::new(0));
        let cell = Arc::new(ThreadLocalCell::new());
        let (initialized_tx, initialized_rx) = mpsc::channel();
        let (dropped_tx, dropped_rx) = mpsc::channel();

        let worker = thread::spawn({
            let cell = cell.clone();
            let drops = drops.clone();
            move || {
                init(&cell, &drops);
                drop(cell);
                initialized_tx.send(()).unwrap();

                dropped_rx.recv().unwrap();
                let dropped_before_access = drops.load(Ordering::SeqCst);
                let other = ThreadLocalCell::<()>::new();
                let _ = other.get_mut_or_try_init(|| Ok::<_, ()>(())).unwrap();
                (dropped_before_access, drops.load(Ordering::SeqCst))
            }
        });

        initialized_rx.recv().unwrap();
        drop(Arc::into_inner(cell).expect("the worker released the cell"));
        assert_eq!(
            drops.load(Ordering::SeqCst),
            0,
            "the value must not be dropped outside its thread"
        );
        dropped_tx.send(()).unwrap();

        let (dropped_before_access, dropped_after_access) = worker.join().unwrap();
        assert_eq!(dropped_before_access, 0);
        assert_eq!(dropped_after_access, 1);
    }
}
