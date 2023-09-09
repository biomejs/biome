use biome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message(unknown))]
struct TestDiagnostic {}

fn main() {}
