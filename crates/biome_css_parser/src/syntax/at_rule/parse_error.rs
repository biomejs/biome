use crate::parser::CssParser;
use biome_parser::diagnostic::{expect_one_of, expected_node, ToDiagnostic};
use biome_parser::prelude::ParseDiagnostic;
use biome_rowan::TextRange;

pub(crate) fn expected_media_query(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("media query", range, p)
}

pub(crate) fn expected_keyframes_item(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("keyframes item", range, p)
}

pub(crate) fn expected_keyframes_item_selector(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["from", "to", "a percentage"], range).into_diagnostic(p)
}

pub(crate) fn expected_page_selector(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("<ident-token>? <pseudo-page>*", range, p)
}

pub(crate) fn expected_page_selector_pseudo(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&[":left", ":right", ":first", ":blank"], range).into_diagnostic(p)
}

pub(crate) fn expected_any_page_at_rule_item(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["declaration", "at rule", "margin at rule"], range).into_diagnostic(p)
}

pub(crate) fn expected_any_scope_range(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(
        &[
            "(<scope-start>)",
            "to (<scope-end>)",
            "(scope-start) to (<scope-end>)",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_any_namespace_url(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["url()", "<string-token>"], range).into_diagnostic(p)
}

pub(crate) fn expected_any_document_matcher(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(
        &[
            "url()",
            "url-prefix(<string>)",
            "domain(<string>)",
            "media-document(<string>)",
            "regexp(<string>)",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_font_feature_values_item(
    p: &CssParser,
    range: TextRange,
) -> ParseDiagnostic {
    expected_node("font-feature-values item", range, p)
}

pub(crate) fn expected_any_font_feature_value_item(
    p: &CssParser,
    range: TextRange,
) -> ParseDiagnostic {
    expect_one_of(
        &[
            "stylistic",
            "historical_forms",
            "styleset",
            "character_variant",
            "swash",
            "ornaments",
            "annotation",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_any_font_family_name(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["<family-name>", "<string>"], range).into_diagnostic(p)
}
