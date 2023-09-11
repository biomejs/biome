mod reporter;

use crate::reporter::reporter::ReportWriter;
use crate::CliDiagnostic;
use biome_console::{markup, Console, ConsoleExt};
use biome_diagnostics::{Error, PrintDiagnostic};
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
pub(crate) use reporter::{ConsoleReporter, ReportDiff, TraverseSummary};

pub struct Reporter {
    pub(crate) diagnostics: DashMap<String, Vec<Error>>,
    pub(crate) should_report_to_terminal: bool,
    pub(crate) verbose: bool,
}

impl Reporter {
    pub(crate) fn report_diagnostic(&self, path: String, diagnostic: Error) {
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

    pub(crate) fn report_diagnostics(&self, path: String, diagnostics: Vec<Error>) {
        match self.diagnostics.entry(path) {
            Entry::Occupied(mut entry) => {
                let mut error_list = entry.get_mut();
                error_list.extend(diagnostics);
            }
            Entry::Vacant(entry) => {
                entry.insert(diagnostics);
            }
        }
    }

    pub(crate) fn report_diff(&self, path: String, report: ReportDiff) {
        todo!()
    }

    pub(crate) fn report_summary(&self, summary: TraverseSummary) {
        todo!()
    }

    pub(crate) fn dump(mut self, console: &mut impl Console) -> Result<(), CliDiagnostic> {
        for diagnostic in self.diagnostics {
            console.error(markup! {
            {if self.verbose { PrintDiagnostic::verbose(&diagnostic) } else { PrintDiagnostic::simple(&diagnostic) }}
        });
        }

        if mode.is_check() && total_skipped_suggested_fixes > 0 {
            console.log(markup! {
            <Warn>"Skipped "{total_skipped_suggested_fixes}" suggested fixes.\n"</Warn>
            <Info>"If you wish to apply the suggested (unsafe) fixes, use the command "<Emphasis>"biome check --apply-unsafe\n"</Emphasis></Info>
        })
        }

        if !mode.is_ci() && not_printed_diagnostics > 0 {
            console.log(markup! {
            <Warn>"The number of diagnostics exceeds the number allowed by Biome.\n"</Warn>
            <Info>"Diagnostics not shown: "</Info><Emphasis>{not_printed_diagnostics}</Emphasis><Info>"."</Info>
        })
        }

        Ok(())
    }
}
