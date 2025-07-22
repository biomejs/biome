use biome_parser::diagnostic::expected_node;
use biome_parser::prelude::*;
use biome_rowan::TextRange;

use crate::parser::TailwindParser;

pub(crate) fn expected_candidate(p: &TailwindParser, range: TextRange) -> ParseDiagnostic {
    expected_node("candidate", range, p).into_diagnostic(p)
}

pub(crate) fn expected_variant(p: &TailwindParser, range: TextRange) -> ParseDiagnostic {
    expected_node("variant", range, p).into_diagnostic(p)
}

pub(crate) fn expected_value(p: &TailwindParser, range: TextRange) -> ParseDiagnostic {
    expected_node("value", range, p).into_diagnostic(p)
}

pub(crate) fn expected_modifier(p: &TailwindParser, range: TextRange) -> ParseDiagnostic {
    expected_node("modifier", range, p).into_diagnostic(p)
}
