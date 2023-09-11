use crate::execute::TraversalMode;
use crate::CliDiagnostic;
use biome_console::{fmt, markup, Console, ConsoleExt, Markup, MarkupBuf};
use biome_diagnostics::{Error, PrintDiagnostic, Severity};
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use std::io;
use std::time::Duration;

/// Information computed from a diff result
#[derive(Debug)]
pub struct ReportDiff {
    /// The severity fo the diff
    pub severity: Severity,
    /// How was the code before the command
    pub before: String,
    /// How is the code after the command
    pub after: String,
}

pub struct TraverseSummary<'a> {
    pub(crate) count: usize,
    pub(crate) duration: Duration,
    pub(crate) errors: usize,
    pub(crate) traverse: &'a TraversalMode,
}

impl<'a> fmt::Display for TraverseSummary<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> io::Result<()> {
        let verb = match self.traverse {
            TraversalMode::Check { fix_file_mode, .. } => {
                if fix_file_mode.is_some() {
                    "Fixed"
                } else {
                    "Linted"
                }
            }
            TraversalMode::Lint { fix_file_mode, .. } => {
                if fix_file_mode.is_some() {
                    "Fixed"
                } else {
                    "Linted"
                }
            }
            TraversalMode::CI => "Checked",
            TraversalMode::Format { write, .. } => {
                if *write {
                    "Formatted"
                } else {
                    "Compared"
                }
            }
            TraversalMode::Migrate { write, .. } => {
                if *write {
                    "Migrated"
                } else {
                    "Compared"
                }
            }
        };
        markup!(<Info>{verb}" "{self.count}" file(s) in "{self.duration}</Info>).fmt(fmt)?;

        if self.errors > 0 {
            markup!("\n"<Error>"Found "{self.errors}" error(s)"</Error>).fmt(fmt)?
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ReportErrorKind {
    Diagnostic(Error),
    Diff(ReportDiff),
}

trait Reporter<W>: Send + Sync
where
    W: ReportWriter,
{
    // fn report_message(&mut self, message: impl Display);
    fn report_diagnostic(&self, path: String, diagnostic: Error);

    fn report_diff(&self, path: String, report: ReportDiff);

    fn report_summary(&self, summary: TraverseSummary);

    fn dump(self, writer: W) -> Result<(), CliDiagnostic>;
}

pub trait ReportWriter {
    type Formatter;

    fn error(&mut self, markup: Markup) -> Result<(), CliDiagnostic>;

    fn out(&mut self) -> Result<(), CliDiagnostic>;
}

pub struct ConsoleReporter {
    pub(crate) diagnostics: DashMap<String, Vec<Error>>,
    pub(crate) should_report_to_terminal: bool,
    pub(crate) verbose: bool,
}

impl ConsoleReporter {
    pub fn new(verbose: bool, should_report_to_terminal: bool) -> Self {
        Self {
            should_report_to_terminal,
            verbose,
            diagnostics: DashMap::default(),
        }
    }
}

impl<W> Reporter<W> for ConsoleReporter
where
    W: ReportWriter,
{
    fn report_diagnostic(&self, path: String, diagnostic: Error) {
        match self.diagnostics.entry(path) {
            Entry::Occupied(mut entry) => {
                let mut error_list = entry.get_mut();
                error_list.push(diagnostic);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![diagnostic]);
            }
        }
    }

    fn report_diff(&self, path: String, report: ReportDiff) {
        todo!()
    }

    fn report_summary(&self, summary: TraverseSummary) {
        todo!()
    }

    fn dump(mut self, mut writer: W) -> Result<(), CliDiagnostic> {
        for (file, diagnostic) in self.diagnostics {
            writer.error(markup! {
                {if self.verbose { PrintDiagnostic::verbose(diagnostic) } else { PrintDiagnostic::simple(diagnostic) }}
            })?;
        }

        Ok(())
    }
}
