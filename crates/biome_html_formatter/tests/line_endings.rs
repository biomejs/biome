use biome_html_formatter::{HtmlFormatOptions, format_node};
use biome_html_parser::parse_html;
use biome_languages::html::HtmlFileSource;

/// Spec fixtures are stored with `\n` line endings (see `.gitattributes`),
/// so sources using other line breaks are tested here instead.
fn assert_format(file_source: HtmlFileSource, source: &str, expected: &str) {
    let parse = parse_html(source, (&file_source).into());
    assert!(!parse.has_errors(), "source failed to parse: {source:?}");

    let options = HtmlFormatOptions::new(file_source);
    let formatted = format_node(options, &parse.syntax(), false).unwrap();
    let output = formatted.print().unwrap().as_code().to_string();

    assert_eq!(output, expected, "for source {source:?}");
}

#[test]
fn astro_multi_line_expression_with_carriage_return_line_feed() {
    assert_format(
        HtmlFileSource::astro(),
        "<p>{a +\r\n  b}</p>\r\n",
        "<p>\n\t{a +\n  b}\n</p>\n",
    );
}

#[test]
fn astro_attribute_expression_with_carriage_return_line_feed() {
    assert_format(
        HtmlFileSource::astro(),
        "<C x={a +\r\n  b} />\r\n",
        "<C\n\tx={a +\n  b}\n/>\n",
    );
}

#[test]
fn svelte_multi_line_expression_with_carriage_return_line_feed() {
    assert_format(
        HtmlFileSource::svelte(),
        "<p>{a +\r\n  b}</p>\r\n",
        "<p>\n\t{a +\n  b}\n</p>\n",
    );
}
