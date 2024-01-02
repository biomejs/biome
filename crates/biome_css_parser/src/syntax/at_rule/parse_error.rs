use crate::parser::CssParser;
use biome_parser::diagnostic::{expect_one_of, expected_any, expected_node, ToDiagnostic};
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
    expected_any(&["declaration", "at rule", "margin at rule"], range, p)
}

pub(crate) fn expected_any_scope_range(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "(<scope-start>)",
            "to (<scope-end>)",
            "(scope-start) to (<scope-end>)",
        ],
        range,
        p,
    )
}
