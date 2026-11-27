use biome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic[category = "internalError/io"]]
struct TestDiagnostic {}

fn main() {}
