use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::{CSS_COLOR, CSS_COLOR_LITERAL};
use biome_css_syntax::{TextRange, T};
use biome_parser::diagnostic::{expected_node, ParseDiagnostic};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::Parser;

#[inline]
pub(crate) fn is_at_color(p: &mut CssParser) -> bool {
    p.at(T![#])
}
#[inline]
pub(crate) fn parse_color(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_color(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T![#], CssLexContext::Color);

    if !p.eat(CSS_COLOR_LITERAL) {
        p.error(expected_color(p, p.cur_range()));
    }

    Present(m.complete(p, CSS_COLOR))
}

/// Generates a parse diagnostic for an expected "color" error message at the given range.
pub(crate) fn expected_color(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("color", range, p)
        .with_hint("Ensure the color is specified in a valid hexadecimal format (#RRGGBB or #RGB). Examples: #000, #000f, #ffffff, #ffffffff")
}
