use crate::parser::HtmlParser;
use biome_console::markup;
use biome_html_syntax::TextRange;
use biome_parser::Parser;
use biome_parser::diagnostic::{ParseDiagnostic, expect_one_of, expected_node};
use biome_parser::prelude::ToDiagnostic;

pub(crate) fn expected_attribute(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("attribute", range, p).into_diagnostic(p)
}

pub(crate) fn disabled_interpolation(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Text expressions aren't supported.", range).with_hint(markup!("Remove it or enable the parsing using the "<Emphasis>"html.parser.interpolation"</Emphasis>" option."))
}

pub(crate) fn disabled_svelte(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("This looks like Svelte syntax, but this is not a Svelte file.", range).with_hint(markup!("Remove it or rename this file to have the "<Emphasis>".svelte"</Emphasis>" file extension."))
}

pub(crate) fn expected_closing_text_expression(
    p: &HtmlParser,
    curr_range: TextRange,
    opening_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        "Found a text expression that doesn't have the closing expression:",
        curr_range,
    )
    .with_detail(
        opening_range,
        "This is where the opening expression was found:",
    )
}

pub(crate) fn expected_text_expression(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("expression", range, p).into_diagnostic(p)
}

pub(crate) fn expected_child(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["element", "text"], range).into_diagnostic(p)
}

pub(crate) fn expected_child_or_block(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["element", "text", "closing block"], range).into_diagnostic(p)
}

pub(crate) fn expected_closed_fence(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("---", range, p).into_diagnostic(p)
}

/// The parser was expecting a value for an attribute initializer clause.
///
/// ```html
/// <div id= />
///         ^ expected initializer
/// ```
pub(crate) fn expected_initializer(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("initializer", range, p).into_diagnostic(p)
}

/// The parser encountered a tag that does not have a corresponding closing tag.
///
/// ```html
/// <div>foo
/// ```
pub(crate) fn expected_closing_tag(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("closing tag", range, p).into_diagnostic(p)
}

pub(crate) fn expected_matching_closing_tag(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("matching closing tag", range, p).into_diagnostic(p)
}

/// The parser was encountered a tag that does not have a name.
///
/// ```html
/// <>
/// ^ expected element name
/// ```
pub(crate) fn expected_element_name(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("element name", range, p).into_diagnostic(p)
}

/// Void elements should not have a closing tag.
///
/// ```html
/// <img></img>
///      ^^^^^^ should not have a closing tag
/// ```
pub(crate) fn void_element_should_not_have_closing_tag(
    _p: &HtmlParser,
    range: TextRange,
) -> ParseDiagnostic {
    ParseDiagnostic::new("Void elements should not have a closing tag.", range)
}

pub(crate) fn closing_tag_should_not_have_attributes(
    _p: &HtmlParser,
    range: TextRange,
) -> ParseDiagnostic {
    ParseDiagnostic::new("Closing tags should not have attributes.", range)
}

pub(crate) fn expected_svelte_closing_block(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a closing block, instead found none.", range)
}

pub(crate) fn expected_name(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["name", "closing block"], range).into_diagnostic(p)
}

pub(crate) fn disabled_vue(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Vue syntax isn't enabled. Is this supposed to be a .vue file?", range).with_hint(markup!("Remove it or enable the parsing using the "<Emphasis>"html.parser.vue"</Emphasis>" option."))
}

pub(crate) fn expected_vue_directive_argument(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("vue directive argument", range, p).into_diagnostic(p)
}

pub(crate) fn expected_expression(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected an expression, instead none was found.", range)
        .into_diagnostic(p)
}

pub(crate) fn expected_svelte_property(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a property, instead none was found.", range)
        .into_diagnostic(p)
}

pub(crate) fn expected_valid_directive(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "Expected a valid directive, which should be followed by ':' and a value.",
        range,
    )
    .into_diagnostic(p)
}
