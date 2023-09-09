use biome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message(description = "text {unclosed"))]
struct TestDiagnostic {}

fn main() {}
