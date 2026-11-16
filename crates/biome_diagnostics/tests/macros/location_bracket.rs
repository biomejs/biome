use biome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
struct TestDiagnostic {
    #[location[span]]
    location: (),
}

fn main() {}
