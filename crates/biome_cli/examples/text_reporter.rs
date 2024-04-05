use biome_cli::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};

/// This will be the visitor, which where we **write** the data
struct BufferVisitor(String);

/// This is the reporter, which will be a type that will hold the information needed to the reporter
struct TextReport {
    summary: TraversalSummary,
}

impl Reporter for TextReport {
    fn write(&mut self, visitor: &mut dyn ReporterVisitor) -> std::io::Result<()> {
        let execution = Execution::new_format();
        visitor.report_summary(&execution, &self.summary)?;
        Ok(())
    }
}

impl ReporterVisitor for BufferVisitor {
    fn report_summary(
        &mut self,
        _execution: &Execution,
        summary: &TraversalSummary,
    ) -> std::io::Result<()> {
        self.0
            .push_str(&format!("Total is {}", summary.changed + summary.unchanged));
        Ok(())
    }

    fn report_diagnostics(&mut self, _payload: &DiagnosticsPayload) -> std::io::Result<()> {
        todo!()
    }
}

pub fn main() {
    let mut summary = TraversalSummary::default();
    summary.changed = 32;
    summary.unchanged = 28;
    let mut visitor = BufferVisitor(String::new());
    let mut reporter = TextReport { summary };
    reporter.write(&mut visitor).unwrap();

    assert_eq!(visitor.0.as_str(), "Total is 64")
}
