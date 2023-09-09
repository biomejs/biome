//! Provides factory function to create common diagnostics for the JavaScript syntax

use crate::prelude::*;
use crate::span::Span;
use crate::JsParser;
use crate::JsSyntaxFeature::TypeScript;
use biome_parser::diagnostic::{expected_any, expected_node};
use rome_js_syntax::TextRange;

pub(crate) fn expected_function_body(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("function body", range).into_diagnostic(p)
}

pub(crate) fn expected_class_member_name(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "identifier",
            "string literal",
            "number literal",
            "private field name",
            "computed name",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_arrow_body(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["function body", "expression"], range).into_diagnostic(p)
}

pub(crate) fn expected_object_member(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "property",
            "shorthand property",
            "getter",
            "setter",
            "method",
        ],
        range,
    )
    .into_diagnostic(p)
}
pub(crate) fn expected_array_element(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "expression", "method"], range).into_diagnostic(p)
}

pub(crate) fn expected_object_member_name(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "identifier",
            "string literal",
            "number literal",
            "computed property",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_block_statement(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("block statement", range).into_diagnostic(p)
}

pub(crate) fn expected_catch_clause(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("catch clause", range).into_diagnostic(p)
}

pub(crate) fn expected_parameter(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("parameter", range).into_diagnostic(p)
}

pub(crate) fn expected_parameters(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("parenthesis '('", range).into_diagnostic(p)
}

pub(crate) fn expected_case_or_default(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["default", "case"], range).into_diagnostic(p)
}

pub(crate) fn expected_case(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("case", range).into_diagnostic(p)
}

pub(crate) fn expected_assignment_target(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "assignment target"], range).into_diagnostic(p)
}

pub(crate) fn expected_simple_assignment_target(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "member expression"], range).into_diagnostic(p)
}

pub(crate) fn expected_identifier(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range).into_diagnostic(p)
}

pub(crate) fn expected_statement(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("statement", range).into_diagnostic(p)
}

pub(crate) fn expected_binding(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "array pattern", "object pattern"], range).into_diagnostic(p)
}

pub(crate) fn expected_class_member(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property ", "method", "getter", "setter"], range).into_diagnostic(p)
}

pub(crate) fn expected_class_parameters(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("class parameters", range).into_diagnostic(p)
}

pub(crate) fn expected_constructor_parameters(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("constructor parameters", range).into_diagnostic(p)
}

pub(crate) fn expected_class_method_body(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("class method body", range).into_diagnostic(p)
}

pub(crate) fn expected_module_source(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("string literal", range).into_diagnostic(p)
}

pub(crate) fn expected_named_import(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["namespace import", "named imports"], range).into_diagnostic(p)
}

pub(crate) fn expected_literal_export_name(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["string literal", "identifier"], range).into_diagnostic(p)
}

pub(crate) fn expected_export_clause(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["class", "function", "variable declaration"], range).into_diagnostic(p)
}

pub(crate) fn expected_export_name_specifier(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("export name", range).into_diagnostic(p)
}

pub(crate) fn expected_named_import_specifier(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range).into_diagnostic(p)
}

pub(crate) fn duplicate_assertion_keys_error(
    p: &JsParser,
    key: &str,
    first_use: TextRange,
    duplicate_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder("Duplicate assertion keys are not allowed", first_use)
        .detail(first_use, format!("First use of the key `{}`", key))
        .detail(duplicate_range, "second use here")
}

pub(crate) fn expected_expression(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("expression", range).into_diagnostic(p)
}

pub(crate) fn expected_expression_assignment(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["expression", "assignment"], range).into_diagnostic(p)
}

pub(crate) fn expected_unary_expression(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("unary expression", range).into_diagnostic(p)
}

pub(crate) fn expected_property_or_signature(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "signature"], range).into_diagnostic(p)
}

pub(crate) fn expected_declaration(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "function",
            "class",
            "variable declaration",
            "interface",
            "enum",
            "type alias",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_export_default_declaration(
    p: &JsParser,
    range: TextRange,
) -> ParseDiagnostic {
    let expected = if TypeScript.is_supported(p) {
        expected_any(
            &[
                "class declaration",
                "function declaration",
                "interface declaration",
            ],
            range,
        )
    } else {
        expected_any(&["class declaration", "function declaration"], range)
    };

    expected.into_diagnostic(p)
}

pub(crate) fn unexpected_body_inside_ambient_context(
    p: &JsParser,
    range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        "members inside ambient contexts should not have a body",
        range,
    )
}

pub(crate) fn private_names_only_allowed_on_left_side_of_in_expression(
    p: &JsParser,
    private_name_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        "Private names are only allowed on the left side of a 'in' expression",
        private_name_range,
    )
}

pub(crate) fn invalid_assignment_error(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        format!("Invalid assignment to `{}`", p.text(range.as_range()),),
        range,
    )
    .hint("This expression cannot be assigned to")
}

pub(crate) fn modifier_already_seen(
    p: &JsParser,
    second_range: TextRange,
    first_range: TextRange,
) -> ParseDiagnostic {
    let modifier = p.text(second_range);
    p.err_builder(format!("'{modifier}' already seen"), second_range)
        .detail(second_range, "duplicate modifier")
        .detail(first_range, "first seen here")
}

pub(crate) fn modifier_cannot_be_used_with_modifier(
    p: &JsParser,
    range: TextRange,
    other_modifier_range: TextRange,
) -> ParseDiagnostic {
    let modifier = p.text(range);
    let other_modifier = p.text(other_modifier_range);

    p.err_builder(
        format!("'{modifier}' cannot be used with '{other_modifier}' modifier."),
        range,
    )
    .detail(range, format!("'{modifier}' modifier"))
    .detail(other_modifier_range, format!("'{other_modifier}' modifier"))
}

pub(crate) fn modifier_must_precede_modifier(
    p: &JsParser,
    range: TextRange,
    to_precede_modifier_range: TextRange,
) -> ParseDiagnostic {
    let modifier_name = p.text(range);
    let to_precede_name = p.text(to_precede_modifier_range);

    p.err_builder(
        format!("'{modifier_name}' must precede '{to_precede_name}'",),
        range,
    )
    .detail(range, "move this modifier")
    .detail(to_precede_modifier_range, "before this modifier")
}

pub(crate) fn invalid_decorator_error(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        format!("Invalid decorator `{}`", p.text(range.as_range()),),
        range,
    )
}

pub(crate) fn parameter_decorators_not_allowed(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    decorators_not_allowed(p, range).hint("You can enable parameter decorators by setting the `unsafeParameterDecoratorsEnabled` option to `true` in your configuration file.")
}

pub(crate) fn decorators_not_allowed(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Decorators are not valid here.", range).hint(
        "Decorators are only valid on class declarations, class expressions, and class methods.",
    )
}

pub(crate) fn decorator_must_precede_modifier(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "Decorators must precede the name and all keywords of property declarations.",
        range,
    )
}
