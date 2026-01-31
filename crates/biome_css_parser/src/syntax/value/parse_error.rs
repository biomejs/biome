use crate::parser::CssParser;
use biome_css_syntax::TextRange;
use biome_parser::diagnostic::{ParseDiagnostic, expect_one_of, expected_any, expected_node};
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

pub(crate) fn expected_if_branch(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["if branch", "if test boolean expression"], range, p)
}

pub(crate) fn expected_if_test_boolean_expr_group(
    p: &CssParser,
    range: TextRange,
) -> ParseDiagnostic {
    expected_any(&["parenthesized boolean expression", "if test"], range, p)
}

pub(crate) fn expected_syntax_component(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["syntax type", "identifier"], range, p)
}

pub(crate) fn expected_any_syntax(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["*", "syntax single component", "string"], range, p)
}

pub(crate) fn expected_syntax_type_name(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("syntax type name", range, p)
}

pub(crate) fn expected_if_test_boolean_not_expr(
    p: &CssParser,
    range: TextRange,
) -> ParseDiagnostic {
    expected_any(&["not boolean expression", "if test"], range, p)
}
