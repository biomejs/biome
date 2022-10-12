use std::fs;

fn main() {
    const IDL_FILE: &str = "src/bindings/rowan.idl";

    println!("cargo:rerun-if-changed={IDL_FILE}");

    let source = fs::read_to_string(IDL_FILE).expect("failed to read IDL file");
    let content = rome_idl::generate(&source);

    fs::write("src/bindings/rowan.idl.rs", content).expect("failed to write generated code");
}
