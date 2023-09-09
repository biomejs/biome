use biome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
struct TestDiagnostic {
    #[unknown_attr]
    field: bool,
}

fn main() {}
