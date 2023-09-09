#[cfg(feature = "dhat-heap")]
use crate::features::print_stats;
use crate::language::Parse;
use crate::test_case::TestCase;
use crate::BenchmarkSummary;
use biome_parser::diagnostic::ParseDiagnostic;
use itertools::Itertools;
use rome_diagnostics::console::fmt::Termcolor;
use rome_diagnostics::console::markup;
use rome_diagnostics::termcolor::Buffer;
use rome_diagnostics::DiagnosticExt;
use rome_diagnostics::PrintDiagnostic;
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ParseMeasurement {
    id: String,
    code: String,
    duration: Duration,
    diagnostics: Vec<ParseDiagnostic>,
}

pub fn benchmark_parse_lib(case: &TestCase, parse: &Parse) -> BenchmarkSummary {
    #[cfg(feature = "dhat-heap")]
    println!("Start");
    #[cfg(feature = "dhat-heap")]
    let stats = print_stats(dhat::HeapStats::get(), None);

    let parser_timer = timing::start();
    let parsed = parse.parse();
    let parse_duration = parser_timer.stop();

    #[cfg(feature = "dhat-heap")]
    println!("Parsed");
    #[cfg(feature = "dhat-heap")]
    print_stats(dhat::HeapStats::get(), Some(stats));

    BenchmarkSummary::Parser(ParseMeasurement {
        id: case.filename().to_string(),
        code: case.code().to_string(),
        duration: parse_duration,
        diagnostics: parsed.into_diagnostics(),
    })
}

impl ParseMeasurement {
    pub(crate) fn summary(&self) -> String {
        format!("{}, Total Time: {:?}", self.id, self.duration,)
    }
}

impl Display for ParseMeasurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "\tDuration:        {:>10?}", self.duration);

        let _ = writeln!(f, "\tDiagnostics");

        let diagnostics = &self
            .diagnostics
            .iter()
            .map(|diagnostic| rome_diagnostics::Error::from(diagnostic.clone()))
            .group_by(|x| x.severity());
        for (severity, items) in diagnostics {
            let _ = writeln!(f, "\t\t{:?}: {}", severity, items.count());
        }

        let mut buffer = Buffer::no_color();

        for diagnostic in self.diagnostics.iter().filter(|diag| diag.is_error()) {
            let error = diagnostic
                .clone()
                .with_file_path(self.id.to_string())
                .with_file_source_code(self.code.clone());
            rome_diagnostics::console::fmt::Formatter::new(&mut Termcolor(&mut buffer))
                .write_markup(markup! {
                    {PrintDiagnostic::verbose(&error)}
                })
                .unwrap();
        }

        Ok(())
    }
}
