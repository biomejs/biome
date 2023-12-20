use crate::parser::CssParser;
use biome_parser::diagnostic::expected_node;
use biome_parser::prelude::ParseDiagnostic;
use biome_rowan::TextRange;

pub(crate) fn expected_media_query(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("media query", range, p)
}
