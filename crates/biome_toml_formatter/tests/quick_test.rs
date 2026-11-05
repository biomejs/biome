use biome_toml_formatter::{TomlFormatOptions, format_node};
use biome_toml_parser::parse_toml;

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
[a]
b = [1, 2]
"#;
    let parse = parse_toml(src);
    let formatted = format_node(TomlFormatOptions::default(), &parse.syntax()).unwrap();
    println!("{}", formatted.print().unwrap().as_code());
}
