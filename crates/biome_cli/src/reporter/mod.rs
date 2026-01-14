pub(crate) mod checkstyle;
pub(crate) mod github;
pub(crate) mod gitlab;
pub(crate) mod json;
pub(crate) mod junit;
pub(crate) mod rdjson;
pub(crate) mod sarif;
pub(crate) mod summary;
pub(crate) mod terminal;

use crate::cli_options::MaxDiagnostics;
use crate::runner::execution::Execution;
use biome_console::{Console, ConsoleExt, FileBufferConsole, Markup};
use biome_diagnostics::advice::ListAdvice;
use biome_diagnostics::{Diagnostic, Error, Severity};
use biome_fs::BiomePath;
use camino::Utf8Path;
use serde::Serialize;
use std::collections::BTreeSet;
use std::io;
use std::time::Duration;

#[derive(Debug)]
pub struct DiagnosticsPayload {
    pub diagnostics: Vec<Error>,
    pub diagnostic_level: Severity,
    pub max_diagnostics: MaxDiagnostics,
}

/// A type that holds the result of the traversal
#[derive(Debug, Default, Serialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TraversalSummary {
    pub changed: usize,
    pub unchanged: usize,
    pub matches: usize,
    // We skip it during testing because the time isn't predictable
    #[cfg_attr(debug_assertions, serde(skip))]
    pub duration: Duration,
    // We skip it during testing because the time isn't predictable
    #[cfg_attr(debug_assertions, serde(skip))]
    pub scanner_duration: Option<Duration>,
    pub errors: u32,
    pub warnings: u32,
    pub infos: u32,
    pub skipped: usize,
    pub suggested_fixes_skipped: u32,
    pub diagnostics_not_printed: u32,
}

/// When using this trait, the type that implements this trait is the one that holds the read-only information to pass around
pub(crate) trait Reporter: Sized {
    /// Writes the summary using the underling visitor
    fn write(
        self,
        writer: &mut dyn ReporterWriter,
        visitor: &mut dyn ReporterVisitor,
    ) -> io::Result<()>;
}

/// When using this trait, the type that implements this trait is the one that will **write** the data, ideally inside a buffer
pub(crate) trait ReporterVisitor {
    /// Writes the summary in the underling writer
    fn report_summary(
        &mut self,
        _writer: &mut dyn ReporterWriter,
        _execution: &dyn Execution,
        _summary: TraversalSummary,
        _verbose: bool,
    ) -> io::Result<()>;

    /// Writes the paths handled during a run.
    fn report_handled_paths(
        &mut self,
        _writer: &mut dyn ReporterWriter,
        _evaluated_paths: BTreeSet<BiomePath>,
        _working_directory: Option<&Utf8Path>,
    ) -> io::Result<()> {
        Ok(())
    }

    /// Writes a diagnostics
    fn report_diagnostics(
        &mut self,
        _writer: &mut dyn ReporterWriter,
        _execution: &dyn Execution,
        _payload: &DiagnosticsPayload,
        _verbose: bool,
        _working_directory: Option<&Utf8Path>,
    ) -> io::Result<()>;
}

pub trait ReporterWriter {
    fn log(&mut self, message: Markup);
    fn error(&mut self, message: Markup);
    fn dump(&mut self) -> Option<String>;
    fn clear(&mut self);
}

pub(crate) struct ConsoleReporterWriter<'a, C>(pub(crate) &'a mut C)
where
    C: Console + ?Sized;

impl<'a, C> ReporterWriter for ConsoleReporterWriter<'a, C>
where
    C: Console + ?Sized,
{
    fn log(&mut self, message: Markup) {
        self.0.log(message);
    }

    fn error(&mut self, message: Markup) {
        self.0.error(message);
    }

    fn dump(&mut self) -> Option<String> {
        None
    }

    fn clear(&mut self) {}
}

pub(crate) struct FileReporterWriter(FileBufferConsole);

impl Default for FileReporterWriter {
    fn default() -> Self {
        Self(FileBufferConsole::default())
    }
}

impl ReporterWriter for FileReporterWriter {
    fn log(&mut self, message: Markup) {
        self.0.log(message);
    }

    fn error(&mut self, message: Markup) {
        self.0.error(message);
    }

    fn dump(&mut self) -> Option<String> {
        self.0.dump()
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    tags(VERBOSE),
    severity = Information,
    message = "Files fixed:"
)]
pub(crate) struct FixedPathsDiagnostic {
    #[advice]
    advice: ListAdvice<String>,
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    tags(VERBOSE),
    severity = Information,
    message = "Files processed:"
)]
pub(crate) struct EvaluatedPathsDiagnostic {
    #[advice]
    advice: ListAdvice<String>,
}
