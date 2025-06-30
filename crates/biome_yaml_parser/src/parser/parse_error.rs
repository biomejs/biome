use crate::parser::YamlParser;
use biome_parser::diagnostic::{ParseDiagnostic, expected_node};
use biome_rowan::TextRange;

pub(crate) fn expected_block_mapping(p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("mapping", range, p)
}

// This shouldn't happen that often
pub(crate) fn malformed_document(_p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    ParseDiagnostic::new("Malformed document", range)
}

pub(crate) fn expected_directive(p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("directive", range, p)
}
