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
