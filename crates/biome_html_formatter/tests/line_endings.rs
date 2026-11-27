//! Formatting of sources whose line breaks are not `\n`. Spec fixtures cannot
//! cover them because they are stored with `\n` line endings (see
//! `.gitattributes`).

use biome_formatter::{LineEnding, SourceMapGeneration};
use biome_html_formatter::{HtmlFormatOptions, format_node, format_node_with_verbatim_ranges};
use biome_html_parser::parse_html;
use biome_html_syntax::{
    AstroEmbeddedContent, HtmlEmbeddedContent, HtmlString, HtmlTextExpression,
};
use biome_languages::html::HtmlFileSource;
use biome_rowan::{AstNode, WalkEvent};

/// Parses `source`, formats it with `options`, and asserts that the printed
/// output equals `expected`.
fn assert_format(options: HtmlFormatOptions, source: &str, expected: &str) {
    let parse = parse_html(source, options.file_source().into());
    assert!(!parse.has_errors(), "source failed to parse: {source:?}");

    let formatted = format_node(options, &parse.syntax(), Vec::new()).unwrap();
    let output = formatted.print().unwrap().as_code().to_string();

    assert_eq!(output, expected, "for source {source:?}");
}

fn assert_format_with_verbatim_embeds(options: HtmlFormatOptions, source: &str, expected: &str) {
    let parse = parse_html(source, options.file_source().into());
    let kinds = HtmlEmbeddedContent::KIND_SET
        .union(AstroEmbeddedContent::KIND_SET)
        .union(HtmlTextExpression::KIND_SET)
        .union(HtmlString::KIND_SET);
    let ranges = parse
        .syntax()
        .preorder_of_kinds(kinds)
        .filter_map(|event| match event {
            WalkEvent::Enter(node) => HtmlEmbeddedContent::cast(node.clone())
                .map(|content| content.range())
                .or_else(|| AstroEmbeddedContent::cast(node.clone()).map(|content| content.range()))
                .or_else(|| HtmlTextExpression::cast(node.clone()).map(|content| content.range()))
                .or_else(|| {
                    HtmlString::cast(node).and_then(|content| content.inner_string_range().ok())
                }),
            WalkEvent::Leave(_) => None,
        })
        .collect();
    let formatted =
        format_node_with_verbatim_ranges(options, &parse.syntax(), Vec::new(), ranges).unwrap();
    let output = formatted
        .print_with_indent(0, SourceMapGeneration::Enabled)
        .unwrap()
        .as_code()
        .to_string();

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

#[test]
fn cjk_segment_break_with_carriage_return() {
    assert_format(
        HtmlFormatOptions::new(HtmlFileSource::html()),
        "<div>漢\r字</div>",
        "<div>\n\t漢\n\t字\n</div>\n",
    );
}

#[test]
fn cjk_segment_break_with_carriage_return_line_feed() {
    assert_format(
        HtmlFormatOptions::new(HtmlFileSource::html()),
        "<div>漢\r\n字</div>",
        "<div>\n\t漢\n\t字\n</div>\n",
    );
}

#[test]
fn opaque_script_preserves_guest_line_endings() {
    assert_format_with_verbatim_embeds(
        HtmlFormatOptions::new(HtmlFileSource::astro()),
        "<div><span>before</span></div>\r\n<script lang=\"coffee\">\r\n  square = (value) -> value * value\r</script>\r\n<div><span>after</span></div>\r\n",
        "<div><span>before</span></div>\n<script lang=\"coffee\">\r\n  square = (value) -> value * value\r</script>\n<div><span>after</span></div>\n",
    );
}

#[test]
fn malformed_frontmatter_preserves_guest_line_endings() {
    assert_format_with_verbatim_embeds(
        HtmlFormatOptions::new(HtmlFileSource::astro()),
        "---\r\nconst value =\r---\r\n<div>after</div>\r\n",
        "---\nconst value =\r---\n\n<div>after</div>\n",
    );
}

#[test]
fn malformed_expression_preserves_guest_line_endings() {
    assert_format_with_verbatim_embeds(
        HtmlFormatOptions::new(HtmlFileSource::astro()),
        "<p>{a +\r\n}</p>\r\n",
        "<p>\n\t{a +\r\n}\n</p>\n",
    );
}

#[test]
fn opaque_script_ignores_the_line_ending_option() {
    assert_format_with_verbatim_embeds(
        HtmlFormatOptions::new(HtmlFileSource::astro()).with_line_ending(LineEnding::Crlf),
        "<script lang=\"coffee\">\nvalue = 1\n</script>\n",
        "<script lang=\"coffee\">\nvalue = 1\n</script>\r\n",
    );
}
