use crate::parser::CssParser;
use biome_parser::diagnostic::{expected_any, expected_node};
use biome_parser::prelude::{ParseDiagnostic, ToDiagnostic};
use biome_rowan::TextRange;

pub(crate) fn expected_identifier(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range).into_diagnostic(p)
}

pub(crate) fn expect_any_selector(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("selector", range).into_diagnostic(p)
}

pub(crate) fn expect_any_sub_selector(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "id selector",
            "class selector",
            "attribute selector",
            "pseudo class selector",
            "pseudo element selector",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expect_block(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("body", range).into_diagnostic(p)
}
