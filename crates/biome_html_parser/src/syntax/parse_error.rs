use crate::parser::HtmlParser;
use biome_html_syntax::TextRange;
use biome_parser::diagnostic::{expect_one_of, expected_node, ParseDiagnostic};
use biome_parser::prelude::ToDiagnostic;

pub(crate) fn expected_attribute(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("attribute", range, p).into_diagnostic(p)
}

pub(crate) fn expected_child(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["element", "text"], range).into_diagnostic(p)
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
