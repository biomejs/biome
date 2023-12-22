use crate::parser::CssParser;
use biome_parser::diagnostic::{expect_one_of, expected_node, ToDiagnostic};
use biome_parser::prelude::ParseDiagnostic;
use biome_rowan::TextRange;

pub(crate) fn expected_media_query(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("media query", range, p)
}

pub(crate) fn expected_keyframes_item(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("keyframes item", range, p)
}

pub(crate) fn expected_keyframes_item_selector(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["from", "to", "number"], range).into_diagnostic(p)
}
