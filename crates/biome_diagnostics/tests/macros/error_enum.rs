use biome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
enum ErrorEnum {
    Int(u32),
    Float(f32),
}

fn main() {}
