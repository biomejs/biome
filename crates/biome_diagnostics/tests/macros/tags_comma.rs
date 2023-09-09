use biome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(tags(Identifier, Identifier))]
struct TestDiagnostic {}

fn main() {}
