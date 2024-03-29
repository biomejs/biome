use crate::parser::HtmlParser;
use biome_html_syntax::TextRange;
use biome_parser::diagnostic::{expected_node, ParseDiagnostic};

pub(crate) fn expected_attribute(p: &HtmlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("attribute", range, p)
}
