use crate::parser::CssParser;
use biome_css_syntax::TextRange;
use biome_parser::diagnostic::{expect_one_of, expected_any, ParseDiagnostic};
use biome_parser::prelude::ToDiagnostic;

pub(crate) fn expected_url_modifier(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["function", "identifier"], range).into_diagnostic(p)
}

pub(crate) fn expected_expression(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "component value",
            "binary expression",
            "parenthesized expression",
            "any function expression",
        ],
        range,
        p,
    )
}
