use crate::reporter::{Reporter, ReporterVisitor, ReporterWriter};
use crate::runner::execution::Execution;
use crate::{DiagnosticsPayload, TraversalSummary};
use biome_console::markup;
use biome_diagnostics::display::SourceFile;
use biome_diagnostics::{Error, Resource};
use camino::{Utf8Path, Utf8PathBuf};
use quick_junit::{NonSuccessKind, Report, TestCase, TestCaseStatus, TestSuite};
use std::fmt::{Display, Formatter};
use std::io;

pub(crate) struct JunitReporter<'a> {
    pub(crate) diagnostics_payload: &'a DiagnosticsPayload,
    pub(crate) execution: &'a dyn Execution,
    pub(crate) summary: TraversalSummary,
    pub(crate) verbose: bool,
    pub(crate) working_directory: Option<Utf8PathBuf>,
}

impl Reporter for JunitReporter<'_> {
    fn write(
        self,
        writer: &mut dyn ReporterWriter,

        visitor: &mut dyn ReporterVisitor,
    ) -> io::Result<()> {
        visitor.report_summary(writer, self.execution, self.summary, self.verbose)?;
        visitor.report_diagnostics(
            writer,
            self.execution,
            self.diagnostics_payload,
            self.verbose,
            self.working_directory.as_deref(),
        )?;
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

pub(crate) struct JunitReporterVisitor(pub(crate) Report);

impl JunitReporterVisitor {
    pub(crate) fn new() -> Self {
        let report = Report::new("Biome");
        Self(report)
    }
}

impl ReporterVisitor for JunitReporterVisitor {
    fn report_summary(
        &mut self,
        _writer: &mut dyn ReporterWriter,
        _execution: &dyn Execution,
        summary: TraversalSummary,
        _verbose: bool,
    ) -> io::Result<()> {
        self.0.time = Some(summary.duration);
        self.0.errors = summary.errors as usize;

        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        writer: &mut dyn ReporterWriter,
        _execution: &dyn Execution,
        payload: &DiagnosticsPayload,
        verbose: bool,
        _working_directory: Option<&Utf8Path>,
    ) -> io::Result<()> {
        let diagnostics = payload.diagnostics.iter().filter(|diagnostic| {
            if diagnostic.tags().is_verbose() {
                verbose
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
                    row = start.line_number.get(),
                    col = start.column_number.get(),
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

        writer.log(markup! {
            {self.0.to_string().expect("To serialize report to string")}
        });

        Ok(())
    }
}
