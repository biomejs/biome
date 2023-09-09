use biome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message(description("description")))]
struct TestDiagnostic {}

fn main() {}
