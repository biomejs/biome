use biome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(unknown_attr)]
struct TestDiagnostic {}

fn main() {}
