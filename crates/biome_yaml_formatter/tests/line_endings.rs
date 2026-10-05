use biome_yaml_formatter::{YamlFormatLanguage, YamlFormatOptions};
use biome_yaml_parser::parse_yaml;

/// Spec fixtures are stored with `\n` line endings (see `.gitattributes`),
/// so sources using other line breaks are tested here instead.
fn assert_format(source: &str, expected: &str) {
    let parse = parse_yaml(source);
    assert!(!parse.has_errors(), "source failed to parse: {source:?}");

    let options = YamlFormatOptions::default();
    let formatted = biome_formatter::format_node(
        &parse.syntax(),
        YamlFormatLanguage::new(options.clone()),
        false,
    )
    .unwrap();
    let output = formatted.print().unwrap().as_code().to_string();
    assert_eq!(output, expected, "for source {source:?}");

    // Idempotency
    let reparse = parse_yaml(&output);
    let reformatted =
        biome_formatter::format_node(&reparse.syntax(), YamlFormatLanguage::new(options), false)
            .unwrap();
    assert_eq!(
        reformatted.print().unwrap().as_code(),
        output,
        "reformatting {output:?} changed it"
    );
}

#[test]
fn block_scalar_with_carriage_return_line_breaks() {
    assert_format("a: |\r  foo\r  bar\r", "a: |\n  foo\n  bar\n");
}

#[test]
fn flow_scalar_with_carriage_return_line_breaks() {
    assert_format("a: \"foo\r\r bar\"\r", "a: \"foo\n\n  bar\"\n");
}

#[test]
fn block_scalar_with_carriage_return_line_feed_line_breaks() {
    assert_format("a: |\r\n  foo\r\n  bar\r\n", "a: |\n  foo\n  bar\n");
}

#[test]
fn block_scalar_reindented_with_carriage_return_line_feed_line_breaks() {
    assert_format(
        "a: >\r\n    foo\r\n\r\n     bar\r\n",
        "a: >\n  foo\n\n   bar\n",
    );
}

#[test]
fn keep_chomped_block_scalar_with_carriage_return_line_feed_line_breaks() {
    assert_format("a: |+\r\n  foo\r\n\r\n\r\n", "a: |+\n  foo\n\n\n");
}
