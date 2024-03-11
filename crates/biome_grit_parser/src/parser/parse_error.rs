use crate::parser::GritParser;
use biome_parser::diagnostic::expected_node;
use biome_parser::prelude::ParseDiagnostic;
use biome_parser::Parser;
use biome_rowan::TextRange;

pub(crate) fn expected_int_literal(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    expected_node("integer", range, p)
}

pub(crate) fn expected_language_name(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "Expected a supported language; must be one of `js`, `json`, `css`, `html`, or `grit`",
        range,
    )
}

pub(crate) fn expected_language_flavor(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "Expected a supported language flavor; must be one of `typescript` or `jsx`",
        range,
    )
}

pub(crate) fn expected_pattern(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    expected_node("pattern", range, p)
}

pub(crate) fn expected_predicate(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    expected_node("predicate", range, p)
}

pub(crate) fn expected_predicate_infix_operator(
    p: &GritParser,
    range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        "Expected an operator valid inside predicates; must be one of `+=`, `=`, `==`, `>`, `>=`, `<`, `<=`, `<:`, `!=`, or `=>`.",
        range,
    )
}

pub(crate) fn expected_variable(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    expected_node("variable", range, p)
}
