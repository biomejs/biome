use crate::parser::GraphqlParser;
use biome_parser::diagnostic::{expected_any, expected_node, ParseDiagnostic};
use biome_rowan::TextRange;

pub(crate) fn expected_any_definition(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("definition", range, p)
}

pub(crate) fn expected_selection_set(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("selection set", range, p)
}

pub(crate) fn expected_any_selection(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["field", "fragment spread", "inline fragment"], range, p)
}

pub(crate) fn expected_name(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("name", range, p)
}

pub(crate) fn expected_directive(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("directive", range, p)
}

pub(crate) fn expected_named_type(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("named type", range, p)
}

pub(crate) fn expected_named_or_list_type(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["named type", "list type"], range, p)
}

pub(crate) fn expected_type(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("type", range, p)
}

pub(crate) fn expected_value(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("value", range, p)
}

pub(crate) fn expected_object_field(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("object field", range, p)
}

pub(crate) fn expected_argument(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("argument", range, p)
}

pub(crate) fn expected_variable_definition(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("variable definition", range, p)
}
