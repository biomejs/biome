use crate::parser::YamlParser;
use biome_diagnostics::location::AsSpan;
use biome_parser::{
    Parser,
    diagnostic::{ParseDiagnostic, expected_node},
    prelude::TokenSource,
};
use biome_rowan::{TextLen, TextRange};

pub(crate) fn expected_block_mapping_entry(p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("block mapping entry", range, p)
}

pub(crate) fn expected_block_sequence_entry(p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("block sequence entry", range, p)
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

pub(crate) fn unexpected_token(p: &YamlParser, range: TextRange) -> ParseDiagnostic {
    let msg = if p.source().text().text_len() <= range.start() {
        "Unexpected end of file."
    } else {
        "Unexpected token."
    };
    ParseDiagnostic::new(msg, range.as_span())
}
