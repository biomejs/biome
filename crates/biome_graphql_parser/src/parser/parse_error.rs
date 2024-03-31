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
    // TODO: any selection
    expected_any(&["field", "fragment spread"], range, p)
}

pub(crate) fn expected_name(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("name", range, p)
}

pub(crate) fn expected_directive(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("directive", range, p)
}
