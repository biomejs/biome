//! This modules exposes a number of "adapter diagnostics" that wrap error types
//! such as [std::error::Error] or [std::io::Error] in newtypes implementing the
//! [Diagnostic] trait

#[cfg(feature = "std")]
use std::io;

use biome_console::{
    fmt::{self},
    markup,
};

use crate::{category, Category, Diagnostic};

/// Implements [Diagnostic] over types implementing [std::error::Error].
#[cfg(feature = "std")]
#[derive(Debug)]
pub struct StdError {
    error: Box<dyn std::error::Error + Send + Sync>,
}

#[cfg(feature = "std")]
impl<E: std::error::Error + Send + Sync + 'static> From<E> for StdError {
    fn from(error: E) -> Self {
        Self {
            error: Box::new(error),
        }
    }
}

#[cfg(feature = "std")]
impl Diagnostic for StdError {
    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.error)
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        fmt.write_markup(markup!({ AsConsoleDisplay(&self.error) }))
    }
}

struct AsConsoleDisplay<'a, T>(&'a T);

impl<T: std::fmt::Display> fmt::Display for AsConsoleDisplay<'_, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_fmt(format_args!("{}", self.0))
    }
}

/// Implements [Diagnostic] over for [io::Error].
#[cfg(feature = "std")]
#[derive(Debug)]
pub struct IoError {
    error: io::Error,
}

#[cfg(feature = "std")]
impl From<io::Error> for IoError {
    fn from(error: io::Error) -> Self {
        Self { error }
    }
}

#[cfg(feature = "std")]
impl Diagnostic for IoError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/io"))
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.error)
    }

    fn tags(&self) -> crate::DiagnosticTags {
        crate::DiagnosticTags::INTERNAL
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        fmt.write_markup(markup!({ AsConsoleDisplay(&self.error) }))
    }
}

/// Implements [Diagnostic] over for [bpaf::ParseFailure].
#[cfg(feature = "bpaf")]
#[derive(Debug)]
pub struct BpafError {
    error: bpaf::ParseFailure,
}

#[cfg(feature = "bpaf")]
impl From<bpaf::ParseFailure> for BpafError {
    fn from(error: bpaf::ParseFailure) -> Self {
        Self { error }
    }
}

#[cfg(feature = "bpaf")]
impl Diagnostic for BpafError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("flags/invalid"))
    }

    fn tags(&self) -> crate::DiagnosticTags {
        crate::DiagnosticTags::FIXABLE
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let bpaf::ParseFailure::Stderr(reason) = &self.error {
            write!(fmt, "{reason}")?;
        }
        Ok(())
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        if let bpaf::ParseFailure::Stderr(reason) = &self.error {
            let error = reason.to_string();
            fmt.write_str(&error)?;
        }
        Ok(())
    }
}

#[cfg(feature = "oxc_resolver")]
#[derive(Debug)]
pub struct ResolveError {
    error: oxc_resolver::ResolveError,
}

#[cfg(feature = "oxc_resolver")]
impl From<oxc_resolver::ResolveError> for ResolveError {
    fn from(error: oxc_resolver::ResolveError) -> Self {
        Self { error }
    }
}

#[cfg(feature = "oxc_resolver")]
impl Diagnostic for ResolveError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/io"))
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.error)
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(markup!({ AsConsoleDisplay(&self.error) }))
    }
}

#[derive(Debug)]
pub struct SerdeJsonError {
    error: serde_json::Error,
}

impl From<serde_json::Error> for SerdeJsonError {
    fn from(error: serde_json::Error) -> Self {
        Self { error }
    }
}

impl Diagnostic for SerdeJsonError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/io"))
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.error)
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(markup!({ AsConsoleDisplay(&self.error) }))
    }
}

#[cfg(feature = "serde_ini")]
#[derive(Debug, Clone)]
pub struct IniError {
    error: serde_ini::de::Error,
}

#[cfg(feature = "serde_ini")]
impl Diagnostic for IniError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("configuration"))
    }

    fn severity(&self) -> crate::Severity {
        crate::Severity::Error
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.error)
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(markup!({ AsConsoleDisplay(&self.error) }))
    }
}

#[cfg(feature = "serde_ini")]
impl biome_console::fmt::Display for IniError {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        write!(fmt, "{:?}", self.error)
    }
}

#[cfg(feature = "serde_ini")]
impl From<serde_ini::de::Error> for IniError {
    fn from(error: serde_ini::de::Error) -> Self {
        Self { error }
    }
}

#[cfg(feature = "camino")]
#[derive(Debug, Clone)]
pub struct CaminoError {
    error: camino::FromPathBufError,
}

#[cfg(feature = "camino")]
impl Diagnostic for CaminoError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/fs"))
    }

    fn severity(&self) -> crate::Severity {
        crate::Severity::Error
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.error)
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(markup!({ AsConsoleDisplay(&self.error) }))
    }
}

#[cfg(feature = "camino")]
impl biome_console::fmt::Display for CaminoError {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        write!(fmt, "{:?}", self.error)
    }
}

#[cfg(feature = "camino")]
impl From<camino::FromPathBufError> for CaminoError {
    fn from(error: camino::FromPathBufError) -> Self {
        Self { error }
    }
}
