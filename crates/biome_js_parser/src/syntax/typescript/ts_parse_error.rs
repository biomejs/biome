use crate::prelude::*;
use crate::JsParser;
use biome_diagnostics::location::AsSpan;
use biome_parser::diagnostic::{expected_any, expected_node};
use biome_rowan::TextRange;

pub(crate) fn expected_ts_enum_member(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "string literal", "computed name"], range, p)
}

pub(crate) fn unexpected_abstract_member_with_body(
    p: &JsParser,
    range: TextRange,
) -> ParseDiagnostic {
    p.err_builder("abstract members should not have a body", range)
}

pub(crate) fn abstract_member_cannot_be_async(p: &JsParser, range: &TextRange) -> ParseDiagnostic {
    p.err_builder("async members cannot be abstract", range)
}

pub(crate) fn ts_member_cannot_be(
    p: &JsParser,
    range: impl AsSpan,
    member_type_name: &str,
    modifier_name: &str,
) -> ParseDiagnostic {
    let msg = format!("{member_type_name} members cannot be {modifier_name}");
    p.err_builder(msg, range)
}

pub(crate) fn ts_modifier_cannot_appear_on_a_constructor_declaration(
    p: &JsParser,
    modifier_range: TextRange,
) -> ParseDiagnostic {
    let modifier = p.text(modifier_range);
    p.err_builder(
        format!("'{modifier}' cannot appear on a constructor declaration."),
        modifier_range,
    )
}

pub(crate) fn ts_modifier_cannot_appear_on_a_parameter(
    p: &JsParser,
    modifier_range: TextRange,
) -> ParseDiagnostic {
    let modifier = p.text(modifier_range);
    p.err_builder(
        format!("'{modifier}' cannot appear on a parameter."),
        modifier_range,
    )
}

pub(crate) fn ts_in_out_modifier_cannot_appear_on_a_type_parameter(
    p: &JsParser,
    modifier_range: TextRange,
) -> ParseDiagnostic {
    let modifier = p.text(modifier_range);
    p.err_builder(
        format!("'{modifier}' modifier can only appear on a type parameter of a class, interface or type alias.",),
        modifier_range,
    )
}

pub(crate) fn ts_const_modifier_cannot_appear_on_a_type_parameter(
    p: &JsParser,
    modifier_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        "'const' modifier can only appear on a type parameter of a function, method or class.",
        modifier_range,
    )
}

pub(crate) fn ts_accessibility_modifier_already_seen(
    p: &JsParser,
    second_range: TextRange,
    first_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder("Accessibility modifier already seen.", second_range)
        .with_detail(second_range, "duplicate modifier")
        .with_detail(first_range, "first modifier")
}

pub(crate) fn ts_only_syntax_error(
    p: &JsParser,
    syntax: &str,
    range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(format!("{syntax} are a TypeScript only feature. Convert your file to a TypeScript file or remove the syntax.")
        ,range).with_hint( "TypeScript only syntax")
}

pub(crate) fn ts_accessor_type_parameters_error(
    p: &JsParser,
    type_parameters: &CompletedMarker,
) -> ParseDiagnostic {
    p.err_builder(
        "An accessor cannot have type parameters.",
        type_parameters.range(p),
    )
}

pub(crate) fn ts_constructor_type_parameters_error(
    p: &JsParser,
    type_parameters: &CompletedMarker,
) -> ParseDiagnostic {
    p.err_builder(
        "constructors cannot have type parameters.",
        type_parameters.range(p),
    )
}

pub(crate) fn ts_set_accessor_return_type_error(
    p: &JsParser,
    type_annotation: &CompletedMarker,
) -> ParseDiagnostic {
    p.err_builder(
        "A 'set' accessor cannot have a return type annotation.",
        type_annotation.range(p),
    )
}

pub(crate) fn expected_ts_type(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("type", range, p)
}

pub(crate) fn expected_ts_type_parameter(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("type parameter", range, p)
}

pub(crate) fn infer_not_allowed(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "'infer' declarations are only permitted in the 'extends' clause of a conditional type.",
        range,
    )
}

pub(crate) fn expected_ts_import_type_with_arguments(
    p: &JsParser,
    range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        format!("Expected '(', but got '{}' here", p.cur_text()),
        range,
    )
}
