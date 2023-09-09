//! The `error` module contains the implementation of [Error], a dynamic
//! container struct for any type implementing [Diagnostic].
//!
//! We reduce the size of `Error` by using `Box<Box<dyn Diagnostic>>` (a thin
//! pointer to a fat pointer) rather than `Box<dyn Diagnostic>` (a fat
//! pointer), in order to make returning a `Result<T, Error>` more efficient.
//!
//! When [`ThinBox`](https://doc.rust-lang.org/std/boxed/struct.ThinBox.html)
//! becomes available in stable Rust, we can switch to that.

use std::ops::Deref;
use std::{
    fmt::{Debug, Formatter},
    io,
};

use rome_console::fmt;

use crate::{
    diagnostic::internal::AsDiagnostic, Category, Diagnostic, DiagnosticTags, Location, Severity,
    Visit,
};

/// The `Error` struct wraps any type implementing [Diagnostic] into a single
/// dynamic type.
pub struct Error {
    inner: Box<Box<dyn Diagnostic + Send + Sync + 'static>>,
}

/// Implement the [Diagnostic] trait as inherent methods on the [Error] type.
impl Error {
    /// Calls [Diagnostic::category] on the [Diagnostic] wrapped by this [Error].
    pub fn category(&self) -> Option<&'static Category> {
        self.as_diagnostic().category()
    }

    /// Calls [Diagnostic::severity] on the [Diagnostic] wrapped by this [Error].
    pub fn severity(&self) -> Severity {
        self.as_diagnostic().severity()
    }

    /// Calls [Diagnostic::description] on the [Diagnostic] wrapped by this [Error].
    pub fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_diagnostic().description(fmt)
    }

    /// Calls [Diagnostic::message] on the [Diagnostic] wrapped by this [Error].
    pub fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        self.as_diagnostic().message(fmt)
    }

    /// Calls [Diagnostic::advices] on the [Diagnostic] wrapped by this [Error].
    pub fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        self.as_diagnostic().advices(visitor)
    }

    /// Calls [Diagnostic::verbose_advices] on the [Diagnostic] wrapped by this [Error].
    pub fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        self.as_diagnostic().verbose_advices(visitor)
    }

    /// Calls [Diagnostic::location] on the [Diagnostic] wrapped by this [Error].
    pub fn location(&self) -> Location<'_> {
        self.as_diagnostic().location()
    }

    /// Calls [Diagnostic::tags] on the [Diagnostic] wrapped by this [Error].
    pub fn tags(&self) -> DiagnosticTags {
        self.as_diagnostic().tags()
    }

    /// Calls [Diagnostic::source] on the [Diagnostic] wrapped by this [Error].
    pub fn source(&self) -> Option<&dyn Diagnostic> {
        self.as_diagnostic().source()
    }
}

/// Implement [From] for all types implementing [Diagnostic], [Send], [Sync]
/// and outlives the `'static` lifetime.
impl<T> From<T> for Error
where
    T: Diagnostic + Send + Sync + 'static,
{
    fn from(diag: T) -> Self {
        Self {
            inner: Box::new(Box::new(diag)),
        }
    }
}

impl AsDiagnostic for Error {
    type Diagnostic = dyn Diagnostic;

    fn as_diagnostic(&self) -> &Self::Diagnostic {
        &**self.inner
    }

    fn as_dyn(&self) -> &dyn Diagnostic {
        self.as_diagnostic()
    }
}

impl AsRef<dyn Diagnostic + 'static> for Error {
    fn as_ref(&self) -> &(dyn Diagnostic + 'static) {
        self.as_diagnostic()
    }
}

impl Deref for Error {
    type Target = dyn Diagnostic + 'static;

    fn deref(&self) -> &Self::Target {
        self.as_diagnostic()
    }
}

// Defer the implementation of `Debug` and `Drop` to the wrapped type
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_diagnostic(), f)
    }
}

/// Alias of [std::result::Result] with the `Err` type defaulting to [Error].
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use std::{
        mem::size_of,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
    };

    use crate::{Diagnostic, Error, Result};

    #[derive(Debug)]
    struct TestDiagnostic(Arc<AtomicBool>);

    impl Diagnostic for TestDiagnostic {}

    impl Drop for TestDiagnostic {
        fn drop(&mut self) {
            let was_dropped = self.0.swap(true, Ordering::Relaxed);
            assert!(!was_dropped);
        }
    }

    #[test]
    fn test_drop() {
        let drop_flag = AtomicBool::new(false);
        let drop_flag = Arc::new(drop_flag);

        let diag = TestDiagnostic(drop_flag.clone());

        let error = Error::from(diag);
        drop(error);

        assert!(drop_flag.load(Ordering::Relaxed));
    }

    #[test]
    fn test_error_size() {
        assert_eq!(size_of::<Error>(), size_of::<usize>());
    }

    #[test]
    fn test_result_size() {
        assert_eq!(size_of::<Result<()>>(), size_of::<usize>());
    }
}
