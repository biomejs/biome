use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind;
use biome_parser::diagnostic::{expect_one_of, expected_any, expected_node};
use biome_parser::prelude::{ParseDiagnostic, ToDiagnostic};
use biome_parser::Parser;
use biome_rowan::TextRange;

pub(crate) fn expected_identifier(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range, p)
}

pub(crate) fn expected_dashed_identifier(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("dashed identifier", range, p)
}

/// If we know the token that wasn't parsed is a CSS-wide keyword that isn't
/// allowed here, we can give a more helpful error message since they _are_
/// valid identifiers, just not allowed in specific contexts.
pub(crate) fn expected_non_css_wide_keyword_identifier(
    p: &CssParser,
    range: TextRange,
) -> ParseDiagnostic {
    let text = p.text(range);

    // It's possible that the parser recovered over more than one token and
    // isn't just on the keyword anymore, so we want to try to cast _all_ of
    // the skipped text to a keyword to see if it matches. For example:
    //     @container revert-layer-and
    if CssSyntaxKind::from_keyword(text).is_some_and(|keyword| keyword.is_css_wide_keyword()) {
        // Trying to use `expected_node` here with the additional hint results in
        // two details being added, but since we're adding the hint as well, we
        // only want to show one code frame.
        ParseDiagnostic::new(
            format!("Expected an identifier but instead found '{text}'"),
            range,
        )
        .with_hint(format!(
            "'{text}' is a CSS-wide keyword that cannot be used here"
        ))
    } else {
        expected_identifier(p, range)
    }
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

pub(crate) fn expected_any_rule(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["qualified rule", "at rule"], range, p)
}

pub(crate) fn expected_any_declaration_or_at_rule(
    p: &CssParser,
    range: TextRange,
) -> ParseDiagnostic {
    expected_any(&["declaration", "at rule"], range, p)
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

pub(crate) fn expected_declaration_item(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("declaration item", range, p)
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
