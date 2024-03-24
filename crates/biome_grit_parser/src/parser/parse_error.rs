use super::{SUPPORTED_LANGUAGE_FLAVOR_SET_STR, SUPPORTED_LANGUAGE_SET_STR};
use crate::parser::GritParser;
use biome_parser::diagnostic::expected_node;
use biome_parser::prelude::ParseDiagnostic;
use biome_parser::Parser;
use biome_rowan::TextRange;

pub(crate) fn expected_definition(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a definition.", range)
        .with_hint("Definitions can be functions, patterns, or predicates.")
}

pub(crate) fn expected_engine_version(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected an engine version.", range)
        .with_hint("Add a version between parentheses. For example: '(1.0)'")
}

pub(crate) fn expected_int_literal(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    expected_node("integer", range, p)
}

pub(crate) fn expected_language_name(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Unexpected language.", range)
        .with_alternatives("Expected one of:", SUPPORTED_LANGUAGE_SET_STR)
}

pub(crate) fn expected_language_flavor(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Unexpected language flavor.", range)
        .with_alternatives("Expected one of:", SUPPORTED_LANGUAGE_FLAVOR_SET_STR)
}

pub(crate) fn expected_list_pattern(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    expected_node("pattern or '...'", range, p)
}

pub(crate) fn expected_map_element(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a map element.", range)
        .with_hint("Map elements consist of a name, followed by a colon and a pattern.")
}

pub(crate) fn expected_node_arg(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Unexpected node argument.", range).with_hint(
        "Node arguments must be patterns, optionally preceeded by a name and an equal sign.",
    )
}

pub(crate) fn expected_pattern(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    expected_node("pattern", range, p)
}

pub(crate) fn expected_predicate(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    expected_node("predicate", range, p)
}

pub(crate) fn expected_predicate_call_arg(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Unexpected predicate call argument.", range).with_hint(
        "Predicate call arguments must be patterns, optionally preceeded by a name and an equal sign.",
    )
}

pub(crate) fn expected_predicate_infix_operator(
    p: &GritParser,
    range: TextRange,
) -> ParseDiagnostic {
    p.err_builder("Expected an operator valid inside predicates.", range)
        .with_alternatives(
            "Must be one of:",
            &["+=", "=", "==", ">", ">=", "<", "<=", "<:", "!=", "=>"],
        )
}

pub(crate) fn expected_variable(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    expected_node("variable", range, p)
}

pub(crate) fn too_many_patterns(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Too many patterns.", range).with_hint(
        "Grit files may only contain a single pattern. Use `sequential` if you would like to match multiple patterns.",
    )
}
