use crate::parser::CssParser;
use biome_parser::diagnostic::{expect_one_of, expected_any, expected_node};
use biome_parser::prelude::{ParseDiagnostic, ToDiagnostic};
use biome_rowan::TextRange;

pub(crate) fn expected_identifier(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range, p)
}

pub(crate) fn expected_express(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("express", range, p)
}

pub(crate) fn expected_number(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("number", range, p)
}

pub(crate) fn expected_string(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("string", range, p)
}

pub(crate) fn expected_any_pseudo_class_nth(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["even", "odd", "n", "<An+B>", "number"], range, p)
}

pub(crate) fn expected_selector(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("selector", range, p)
}

pub(crate) fn expected_relative_selector(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("relative selector", range, p)
}

pub(crate) fn expected_compound_selector(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("compound selector", range, p)
}

pub(crate) fn expected_any_sub_selector(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "id selector",
            "class selector",
            "attribute selector",
            "pseudo class selector",
            "pseudo element selector",
        ],
        range,
        p,
    )
}

pub(crate) fn expected_any_attribute_matcher_name(
    p: &CssParser,
    range: TextRange,
) -> ParseDiagnostic {
    expected_any(&["identifier", "string literal"], range, p)
}

pub(crate) fn expected_any_attribute_modifier(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["s", "S", "i", "I"], range, p)
}

pub(crate) fn expected_any_pseudo_element(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(
        &[
            "after",
            "backdrop",
            "before",
            "cue",
            "cue-region",
            "first-letter",
            "first-line",
            "file-selector-button",
            "grammar-error",
            "part",
            "placeholder",
            "selection",
            "slotted",
            "spelling-error",
            "target-text",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_any_pseudo_class(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(
        &[
            "hover",
            "focus",
            "active",
            "first-child",
            "last-child",
            "nth-child",
            "nth-last-child",
            "first-of-type",
            "last-of-type",
            "nth-of-type",
            "nth-last-of-type",
            "only-child",
            "only-of-type",
            "checked",
            "disabled",
            "enabled",
            "required",
            "optional",
            "valid",
            "invalid",
            "in-range",
            "out-of-range",
            "read-only",
            "read-write",
            "placeholder-shown",
            "default",
            "checked",
            "indeterminate",
            "blank",
            "empty",
            "root",
            "target",
            "lang",
            "not",
            "is",
            "where",
            "fullscreen",
            "link",
            "visited",
            "any-link",
            "local-link",
            "scope",
            "current",
            "past",
            "future",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_any_at_rule(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(
        &[
            "charset",
            "color-profile",
            "container",
            "counter-style",
            "document",
            "font-face",
            "font-feature-values",
            "font-palette-values",
            "import",
            "keyframes",
            "layer",
            "media",
            "namespace",
            "page",
            "property",
            "supports",
            "viewport",
            "scope",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_block(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("body", range, p)
}

pub(crate) fn expected_declaration_item(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("declaration item", range, p)
}
pub(crate) fn expected_unit(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("unit", range, p)
}

pub(crate) fn expected_component_value(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(
        &[
            "identifier",
            "string",
            "number",
            "dimension",
            "ratio",
            "custom property",
            "function",
        ],
        range,
    )
    .into_diagnostic(p)
}
