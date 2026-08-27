//! Formatting of sources whose line breaks are not `\n`. Spec fixtures cannot
//! cover them because they are stored with `\n` line endings (see
//! `.gitattributes`).

use biome_formatter::LineEnding;
use biome_html_formatter::{HtmlFormatOptions, format_node};
use biome_html_parser::parse_html;
use biome_languages::html::HtmlFileSource;

/// Parses `source`, formats it with `options`, and asserts that the printed
/// output equals `expected`.
fn assert_format(options: HtmlFormatOptions, source: &str, expected: &str) {
    let parse = parse_html(source, options.file_source().into());
    assert!(!parse.has_errors(), "source failed to parse: {source:?}");

    let formatted = format_node(options, &parse.syntax(), false).unwrap();
    let output = formatted.print().unwrap().as_code().to_string();

    assert_eq!(output, expected, "for source {source:?}");
}

#[test]
fn astro_multi_line_expression_with_carriage_return_line_feed() {
    assert_format(
        HtmlFormatOptions::new(HtmlFileSource::astro()),
        "<p>{a +\r\n  b}</p>\r\n",
        "<p>\n\t{a +\n  b}\n</p>\n",
    );
}

#[test]
fn astro_attribute_expression_with_carriage_return_line_feed() {
    assert_format(
        HtmlFormatOptions::new(HtmlFileSource::astro()),
        "<C x={a +\r\n  b} />\r\n",
        "<C\n\tx={a +\n  b}\n/>\n",
    );
}

#[test]
fn svelte_multi_line_expression_with_carriage_return_line_feed() {
    assert_format(
        HtmlFormatOptions::new(HtmlFileSource::svelte()),
        "<p>{a +\r\n  b}</p>\r\n",
        "<p>\n\t{a +\n  b}\n</p>\n",
    );
}

#[test]
fn astro_expression_follows_the_line_ending_option() {
    assert_format(
        HtmlFormatOptions::new(HtmlFileSource::astro()).with_line_ending(LineEnding::Crlf),
        "<p>{a +\r\n  b}</p>\r\n",
        "<p>\r\n\t{a +\r\n  b}\r\n</p>\r\n",
    );
}
