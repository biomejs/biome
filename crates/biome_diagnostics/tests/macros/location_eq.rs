use biome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
struct TestDiagnostic {
    #[location = Identifier]
    location: (),
}

fn main() {}
