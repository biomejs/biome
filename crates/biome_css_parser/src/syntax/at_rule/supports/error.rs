use crate::parser::CssParser;
use biome_css_syntax::TextRange;
use biome_parser::diagnostic::{ParseDiagnostic, ToDiagnostic, expect_one_of};

pub(crate) fn expected_any_supports_condition(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(
        &[
            "not <supports-in-parens>",
            "<supports-in-parens>",
            "<supports-in-parens> and <supports-in-parens>",
            "<supports-in-parens> or <supports-in-parens>",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_any_supports_condition_in_parens(
    p: &CssParser,
    range: TextRange,
) -> ParseDiagnostic {
    expect_one_of(&["( <supports-condition> ) ", "<supports-feature>"], range).into_diagnostic(p)
}
