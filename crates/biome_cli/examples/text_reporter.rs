use biome_cli::{
    DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary, VcsTargeted,
};
use camino::Utf8Path;

/// This will be the visitor, which where we **write** the data
struct BufferVisitor(String);

/// This is the reporter, which will be a type that will hold the information needed to the reporter
struct TextReport {
    summary: TraversalSummary,
}

impl Reporter for TextReport {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> std::io::Result<()> {
        let execution = Execution::new_format(VcsTargeted {
            staged: false,
            changed: false,
        });
        visitor.report_summary(&execution, self.summary, false)?;
        Ok(())
    }
}

impl ReporterVisitor for BufferVisitor {
    fn report_summary(
        &mut self,
        _execution: &Execution,
        summary: TraversalSummary,
        _verbose: bool,
    ) -> std::io::Result<()> {
        self.0
            .push_str(&format!("Total is {}", summary.changed + summary.unchanged));
        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        _execution: &Execution,
        _payload: DiagnosticsPayload,
        _verbose: bool,
        _working_directory: Option<&Utf8Path>,
    ) -> std::io::Result<()> {
        todo!()
    }
}

pub fn main() {
    // In a real scenario, the project key is obtained from the
    let summary = TraversalSummary {
        changed: 32,
        unchanged: 28,
        ..TraversalSummary::default()
    };
    let mut visitor = BufferVisitor(String::new());
    let reporter = TextReport { summary };
    reporter.write(&mut visitor).unwrap();

    assert_eq!(visitor.0.as_str(), "Total is 64")
}
