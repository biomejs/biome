use crate::parser::CssParser;
use biome_css_syntax::TextRange;
use biome_parser::diagnostic::{expect_one_of, ParseDiagnostic};
use biome_parser::prelude::ToDiagnostic;

pub(crate) fn expected_url_modifier(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["function", "identifier"], range).into_diagnostic(p)
}
