use crate::parser::YamlParser;
use biome_parser::diagnostic::{ParseDiagnostic, expected_node};
use biome_rowan::TextRange;

pub(crate) fn expected_block_mapping_entry(p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("block mapping entry", range, p)
}

pub(crate) fn expected_block_sequence_entry(p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("block sequence entry", range, p)
}

// This shouldn't happen that often
pub(crate) fn malformed_document(_p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    ParseDiagnostic::new("Malformed document", range)
}

pub(crate) fn expected_directive(p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("directive", range, p)
}

pub(crate) fn expected_flow_sequence_entry(p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("flow sequence entry", range, p)
}

pub(crate) fn expected_flow_mapping_entry(p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("flow mapping entry", range, p)
}

pub(crate) fn expected_flow_mapping_closing_quote(range: TextRange) -> ParseDiagnostic {
    ParseDiagnostic::new("Expected `}`", range)
}

pub(crate) fn expected_flow_sequence_closing_bracket(range: TextRange) -> ParseDiagnostic {
    ParseDiagnostic::new("Expected `]`", range)
}

pub(crate) fn expected_header(p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("block header", range, p)
}
