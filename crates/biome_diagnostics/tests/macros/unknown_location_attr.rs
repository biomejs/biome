use biome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
struct TestDiagnostic {
    #[location(unknown)]
    location: (),
}

fn main() {}
