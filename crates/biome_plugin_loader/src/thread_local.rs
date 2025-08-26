use std::cell::{RefCell, RefMut};

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
            let inner = unsafe { win32::FlsAlloc(Some(dtor::<T>)) };

            Self {
                inner,
                _phantom: PhantomData,
            }
        }

        pub(super) unsafe fn get(&self) -> *mut T {
            unsafe { win32::FlsGetValue(self.inner) as *mut T }
        }

        pub(super) unsafe fn set(&self, value: *mut T) {
            let result = unsafe { win32::FlsSetValue(self.inner, value as *const c_void) };

            assert!(result.is_ok());
        }
    }

    impl<T> Drop for Key<T> {
        fn drop(&mut self) {
            unsafe { win32::FlsFree(self.inner) };
        }
    }

    unsafe extern "system" fn dtor<T>(ptr: *mut c_void) {
        unsafe {
            if !ptr.is_null() {
                std::ptr::drop_in_place(ptr as *mut T);
            }
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
                let result = libc::pthread_key_create(inner.as_mut_ptr(), Some(dtor::<T>));

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

    unsafe extern "C" fn dtor<T>(ptr: *mut c_void) {
        if !ptr.is_null() {
            // FIXME: Dropping the value here will break Boa's GC
            // std::ptr::drop_in_place(ptr as *mut T);
            let _ = ptr as *mut T;
        }
    }
}

/// Thread-local storage.
/// It uses [`Fiber Local Storage`](https://learn.microsoft.com/en-us/windows/win32/procthread/fibers#fiber-local-storage) on Windows,
/// or [`pthread_setspecific(3)`](https://linux.die.net/man/3/pthread_setspecific) on Unix.
pub(crate) struct ThreadLocalCell<T> {
    key: platform::Key<RefCell<T>>,
}

impl<T> ThreadLocalCell<T> {
    pub(crate) fn new() -> Self {
        Self {
            key: unsafe { platform::Key::new() },
        }
    }

    pub(crate) fn get_mut_or_try_init<F, E>(&self, default: F) -> Result<RefMut<'_, T>, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
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
        unsafe {
            self.key.set(cell);
        }
    }
}
