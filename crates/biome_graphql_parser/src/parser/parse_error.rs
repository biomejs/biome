use crate::parser::GraphqlParser;
use biome_parser::{
    diagnostic::{expected_any, expected_node, ParseDiagnostic},
    Parser,
};
use biome_rowan::TextRange;

pub(crate) fn expected_any_definition(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("definition", range, p)
}

pub(crate) fn expected_any_selection(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["field", "fragment spread", "inline fragment"], range, p)
}

pub(crate) fn fragment_name_must_not_be_on(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Fragment name must not be 'on'", range)
}

pub(crate) fn expected_name(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("name", range, p)
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

pub(crate) fn expected_field_definition(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("field definition", range, p)
}

pub(crate) fn expected_argument(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("argument", range, p)
}

pub(crate) fn expected_variable_definition(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("variable definition", range, p)
}

pub(crate) fn expected_variable(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("variable", range, p)
}

pub(crate) fn expected_schema_extension(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "Expected at least one directive or root operation type definition",
        range,
    )
}

pub(crate) fn expected_root_operation_type_definition(
    p: &GraphqlParser,
    range: TextRange,
) -> ParseDiagnostic {
    expected_node("root operation type definition", range, p)
}

pub(crate) fn expected_operation_type(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["query", "mutation", "subscription"], range, p)
}

pub(crate) fn expected_directive(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("directive", range, p)
}

pub(crate) fn expected_directive_location(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a valid directive location", range)
        .with_alternatives(
            "Must be one of:",
            &[
                "QUERY",
                "MUTATION",
                "SUBSCRIPTION",
                "FIELD",
                "FRAGMENT_DEFINITION",
                "FRAGMENT_SPREAD",
                "INLINE_FRAGMENT",
                "VARIABLE_DEFINITION",
                "SCHEMA",
                "SCALAR",
                "OBJECT",
                "FIELD_DEFINITION",
                "ARGUMENT_DEFINITION",
                "INTERFACE",
                "UNION",
                "ENUM",
                "ENUM_VALUE",
                "INPUT_OBJECT",
                "INPUT_FIELD_DEFINITION",
            ],
        )
}

pub(crate) fn expected_object_extension(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "Expected at least one directive, implements interface or a set of fields definition",
        range,
    )
}

pub(crate) fn expected_union_extension(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "Expected at least one directive or a set union member types",
        range,
    )
}

pub(crate) fn expected_enum_extension(p: &GraphqlParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "Expected at least one directive or a set of enum values",
        range,
    )
}

pub(crate) fn expected_input_object_extension(
    p: &GraphqlParser,
    range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        "Expected at least one directive or a set of fields definition",
        range,
    )
}
