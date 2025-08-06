use crate::parser::HtmlParser;
use biome_console::markup;
use biome_html_syntax::TextRange;
use biome_parser::Parser;
use biome_parser::diagnostic::{ParseDiagnostic, expect_one_of, expected_node};
use biome_parser::prelude::ToDiagnostic;

/// Creates a diagnostic indicating that an attribute node was expected at the specified range.
///
/// # Examples
///
/// ```
/// let diag = expected_attribute(&parser, text_range);
/// assert_eq!(diag.message(), "Expected attribute");
/// ```
pub(crate) fn expected_attribute(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("attribute", range, p).into_diagnostic(p)
}

/// Creates a diagnostic error indicating that text expressions are not supported at the specified range.
///
/// The diagnostic includes a hint suggesting to remove the text expression or enable parsing via the `"html.parser.textExpression"` option.
///
/// # Examples
///
/// ```
/// let diagnostic = disabled_interpolation(&parser, text_range);
/// assert_eq!(diagnostic.message(), "Text expressions aren't supported.");
/// ```
pub(crate) fn disabled_interpolation(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Text expressions aren't supported.", range).with_hint(markup!("Remove it or enable the parsing using the "<Emphasis>"html.parser.textExpression"</Emphasis>" option."))
}

/// Creates a diagnostic for a text expression missing its closing delimiter.
///
/// The diagnostic highlights the location of the incomplete expression and provides a detail pointing to where the opening expression was found.
///
/// # Examples
///
/// ```
/// let diagnostic = expected_text_expression(parser, curr_range, opening_range);
/// assert_eq!(diagnostic.message(), "Found a text expression that doesn't have the closing expression:");
/// ```
pub(crate) fn expected_text_expression(
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

/// Creates a diagnostic indicating that an element or text node was expected at the specified range.
///
/// # Examples
///
/// ```
/// let diagnostic = expected_child(&parser, text_range);
/// assert!(diagnostic.message().contains("Expected element or text"));
/// ```
pub(crate) fn expected_child(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["element", "text"], range).into_diagnostic(p)
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
