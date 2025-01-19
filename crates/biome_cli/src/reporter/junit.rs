use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::{markup, Console, ConsoleExt};
use biome_diagnostics::display::SourceFile;
use biome_diagnostics::{Error, Resource};
use quick_junit::{NonSuccessKind, Report, TestCase, TestCaseStatus, TestSuite};
use std::fmt::{Display, Formatter};
use std::io;

pub(crate) struct JunitReporter {
    pub(crate) diagnostics_payload: DiagnosticsPayload,
    pub(crate) execution: Execution,
    pub(crate) summary: TraversalSummary,
}

impl Reporter for JunitReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> io::Result<()> {
        visitor.report_summary(&self.execution, self.summary)?;
        visitor.report_diagnostics(&self.execution, self.diagnostics_payload)?;
        Ok(())
    }
}

struct JunitDiagnostic<'a> {
    diagnostic: &'a Error,
}

impl Display for JunitDiagnostic<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.diagnostic.description(f)
    }
}

pub(crate) struct JunitReporterVisitor<'a>(pub(crate) Report, pub(crate) &'a mut dyn Console);

impl<'a> JunitReporterVisitor<'a> {
    pub(crate) fn new(console: &'a mut dyn Console) -> Self {
        let report = Report::new("Biome");
        Self(report, console)
    }
}

impl ReporterVisitor for JunitReporterVisitor<'_> {
    fn report_summary(
        &mut self,
        _execution: &Execution,
        summary: TraversalSummary,
    ) -> io::Result<()> {
        self.0.time = Some(summary.duration);
        self.0.errors = summary.errors as usize;

        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        _execution: &Execution,
        payload: DiagnosticsPayload,
    ) -> io::Result<()> {
        let diagnostics = payload.diagnostics.iter().filter(|diagnostic| {
            if diagnostic.tags().is_verbose() {
                payload.verbose
            } else {
                true
            }
        });

        for diagnostic in diagnostics {
            let mut status = TestCaseStatus::non_success(NonSuccessKind::Failure);
            let message = format!("{}", JunitDiagnostic { diagnostic });
            status.set_message(message.clone());

            let location = diagnostic.location();

            if let (Some(span), Some(source_code), Some(resource)) =
                (location.span, location.source_code, location.resource)
            {
                let source = SourceFile::new(source_code);
                let start = source.location(span.start())?;

                status.set_description(format!(
                    "line {row:?}, col {col:?}, {body}",
                    row = start.line_number.to_zero_indexed(),
                    col = start.column_number.to_zero_indexed(),
                    body = message
                ));
                let mut case = TestCase::new(
                    format!(
                        "org.biome.{}",
                        diagnostic
                            .category()
                            .map(|c| c.name())
                            .unwrap_or_default()
                            .replace('/', ".")
                    ),
                    status,
                );

                if let Resource::File(path) = resource {
                    let mut test_suite = TestSuite::new(path);
                    case.extra
                        .insert("line".into(), start.line_number.get().to_string().into());
                    case.extra.insert(
                        "column".into(),
                        start.column_number.get().to_string().into(),
                    );
                    test_suite
                        .extra
                        .insert("package".into(), "org.biome".into());
                    test_suite.add_test_case(case);
                    self.0.add_test_suite(test_suite);
                }
            }
        }

        self.1.log(markup! {
            {self.0.to_string().unwrap()}
        });

        Ok(())
    }
}
