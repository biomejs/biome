use crate::parser::CssParser;
use biome_css_syntax::TextRange;
use biome_parser::diagnostic::{ParseDiagnostic, ToDiagnostic, expect_one_of};

pub(crate) fn expected_any_container_query(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(
        &[
            "not <query-in-parens>",
            "<query-in-parens>",
            "<query-in-parens> and <query-in-parens>",
            "<query-in-parens> or <query-in-parens>",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_any_container_query_in_parens(
    p: &CssParser,
    range: TextRange,
) -> ParseDiagnostic {
    expect_one_of(
        &[
            "( <container-query> )",
            "( <size-feature> )",
            "style( <style-query> )",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_any_container_style_query(
    p: &CssParser,
    range: TextRange,
) -> ParseDiagnostic {
    expect_one_of(
        &[
            "not <style-in-parens>",
            "<style-in-parens>",
            "<style-in-parens> and <style-in-parens>",
            "<style-in-parens> or <style-in-parens>",
            "<style-feature>",
        ],
        range,
    )
    .into_diagnostic(p)
}

pub(crate) fn expected_any_container_style_in_parens(
    p: &CssParser,
    range: TextRange,
) -> ParseDiagnostic {
    expect_one_of(&["( <style-query> )", "( <style-feature> )"], range).into_diagnostic(p)
}
