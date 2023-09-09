use biome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(category = Identifier)]
struct TestDiagnostic {}

fn main() {}
