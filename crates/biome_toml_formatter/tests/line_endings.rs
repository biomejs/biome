use biome_formatter::LineEnding;
use biome_toml_formatter::{TomlFormatOptions, format_node};
use biome_toml_parser::parse_toml;

fn format(source: &str, line_ending: LineEnding) -> String {
    let options = TomlFormatOptions::default().with_line_ending(line_ending);
    format_node(options, &parse_toml(source).syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code()
}

#[test]
fn crlf_line_endings() {
    let output = format("a=1\nb=\"\"\"\nx\ny\"\"\"\n", LineEnding::Crlf);
    assert_eq!(output, "a = 1\r\nb = \"\"\"\r\nx\r\ny\"\"\"\r\n");
    assert!(!parse_toml(&output).has_errors());
}

/// TOML only allows LF and CRLF line endings.
#[test]
fn carriage_return_line_endings_print_line_feeds() {
    let output = format("a=1\nb=2\n", LineEnding::Cr);
    assert_eq!(output, "a = 1\nb = 2\n");
    assert!(!parse_toml(&output).has_errors());
}
