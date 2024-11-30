//! Provides factory function to create common diagnostics for the JavaScript syntax

use crate::prelude::*;
use crate::span::Span;
use crate::JsParser;
use crate::JsSyntaxFeature::TypeScript;
use biome_js_syntax::TextRange;
use biome_parser::diagnostic::{expected_any, expected_node};

pub(crate) fn expected_function_body(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("function body", range, p)
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
        p,
    )
}

pub(crate) fn expected_arrow_body(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["function body", "expression"], range, p)
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
        p,
    )
}
pub(crate) fn expected_array_element(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "expression", "method"], range, p)
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
        p,
    )
}

pub(crate) fn expected_block_statement(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("block statement", range, p)
}

pub(crate) fn expected_catch_clause(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("catch clause", range, p)
}

pub(crate) fn expected_parameter(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("parameter", range, p)
}

pub(crate) fn expected_parameters(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("parenthesis '('", range, p)
}

pub(crate) fn expected_case_or_default(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["default", "case"], range, p)
}

pub(crate) fn expected_case(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("case", range, p)
}

pub(crate) fn expected_assignment_target(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "assignment target"], range, p)
}

pub(crate) fn expected_simple_assignment_target(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "member expression"], range, p)
}

pub(crate) fn expected_identifier(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range, p)
}

pub(crate) fn expected_statement(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("statement", range, p)
}

pub(crate) fn expected_binding(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "array pattern", "object pattern"], range, p)
}

pub(crate) fn expected_class_member(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property ", "method", "getter", "setter"], range, p)
}

pub(crate) fn expected_class_parameters(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("class parameters", range, p)
}

pub(crate) fn expected_constructor_parameters(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("constructor parameters", range, p)
}

pub(crate) fn expected_class_method_body(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("class method body", range, p)
}

pub(crate) fn expected_module_source(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("string literal", range, p)
}

pub(crate) fn expected_named_import(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["named imports"], range, p)
}

pub(crate) fn expected_namespace_import(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["namespace imports"], range, p)
}

pub(crate) fn expected_declare_statement(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("declare statement", range, p)
}

pub(crate) fn expected_namespace_or_named_import(
    p: &JsParser,
    range: TextRange,
) -> ParseDiagnostic {
    expected_any(&["namespace import", "named imports"], range, p)
}

pub(crate) fn expected_literal_export_name(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["string literal", "identifier"], range, p)
}

pub(crate) fn expected_export_clause(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["class", "function", "variable declaration"], range, p)
}

pub(crate) fn expected_export_name_specifier(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("export name", range, p)
}

pub(crate) fn expected_named_import_specifier(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range, p)
}

pub(crate) fn duplicate_assertion_keys_error(
    p: &JsParser,
    key: &str,
    first_use: TextRange,
    duplicate_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder("Duplicate assertion keys are not allowed", first_use)
        .with_detail(first_use, format!("First use of the key `{key}`"))
        .with_detail(duplicate_range, "second use here")
}

pub(crate) fn expected_expression(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("expression", range, p)
}

pub(crate) fn expected_expression_assignment(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["expression", "assignment"], range, p)
}

pub(crate) fn expected_unary_expression(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("unary expression", range, p)
}

pub(crate) fn expected_property_or_signature(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "signature"], range, p)
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
        p,
    )
}

pub(crate) fn expected_export_default_declaration(
    p: &JsParser,
    range: TextRange,
) -> ParseDiagnostic {
    if TypeScript.is_supported(p) {
        expected_any(
            &[
                "class declaration",
                "function declaration",
                "interface declaration",
            ],
            range,
            p,
        )
    } else {
        expected_any(&["class declaration", "function declaration"], range, p)
    }
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
    .with_hint("This expression cannot be assigned to")
}

pub(crate) fn modifier_already_seen(
    p: &JsParser,
    second_range: TextRange,
    first_range: TextRange,
) -> ParseDiagnostic {
    let modifier = p.text(second_range);
    p.err_builder(format!("'{modifier}' already seen"), second_range)
        .with_detail(second_range, "duplicate modifier")
        .with_detail(first_range, "first seen here")
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
    .with_detail(range, format!("'{modifier}' modifier"))
    .with_detail(other_modifier_range, format!("'{other_modifier}' modifier"))
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
    .with_detail(range, "move this modifier")
    .with_detail(to_precede_modifier_range, "before this modifier")
}

pub(crate) fn invalid_decorator_error(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        format!("Invalid decorator `{}`", p.text(range.as_range()),),
        range,
    )
}

pub(crate) fn parameter_decorators_not_allowed(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    decorators_not_allowed(p, range).with_hint("You can enable parameter decorators by setting the `unsafeParameterDecoratorsEnabled` option to `true` in your configuration file.")
}

pub(crate) fn decorators_not_allowed(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Decorators are not valid here.", range)
        .with_hint(
        "Decorators are only valid on class declarations, class expressions, and class methods.",
    )
}

pub(crate) fn decorator_must_precede_modifier(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "Decorators must precede the name and all keywords of property declarations.",
        range,
    )
}
