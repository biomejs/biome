use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::CssSyntaxFeatures;
use crate::syntax::selector::selector_lex_context;
use biome_css_syntax::CssSyntaxKind::SCSS_INTERPOLATION;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{SyntaxFeature, TokenSet, token_set};

use super::list::parse_scss_inner_expression_until;
use crate::syntax::scss::expected_scss_expression;

const SCSS_INTERPOLATION_END_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set![T!['}'], T![;]];

/// Parses one standalone SCSS interpolation value such as `#{$value}`.
#[inline]
pub(crate) fn parse_scss_regular_interpolation(p: &mut CssParser) -> ParsedSyntax {
    let Some(m) = parse_scss_interpolation_prefix(p) else {
        return Absent;
    };

    parse_scss_interpolation_inner_expression(p);
    p.expect_with_context(T!['}'], CssLexContext::Regular);

    Present(m.complete(p, SCSS_INTERPOLATION))
}

/// Parses one standalone interpolation inside a selector-aware context.
///
/// This keeps the closing `}` in selector lexing mode so whitespace after
/// `#{$...}` becomes a selector combinator instead of trivia.
#[inline]
pub(crate) fn parse_scss_selector_interpolation(p: &mut CssParser) -> ParsedSyntax {
    let Some(m) = parse_scss_interpolation_prefix(p) else {
        return Absent;
    };

    parse_scss_interpolation_inner_expression(p);
    let closing_context = selector_lex_context(p);
    p.expect_with_context(T!['}'], closing_context);

    Present(m.complete(p, SCSS_INTERPOLATION))
}

#[inline]
pub(crate) fn parse_scss_interpolation_inner_expression(p: &mut CssParser) {
    parse_scss_inner_expression_until(p, SCSS_INTERPOLATION_END_TOKEN_SET)
        .or_add_diagnostic(p, expected_scss_expression);
}

/// Parses the shared `#{` prefix of an SCSS interpolation and returns the open
/// marker to be completed by the caller after choosing the recovery and closing
/// policy.
#[inline]
pub(crate) fn parse_scss_interpolation_prefix(p: &mut CssParser) -> Option<Marker> {
    if !is_at_scss_interpolation(p) {
        return None;
    }

    let m = p.start();
    p.bump(T![#]);
    p.bump(T!['{']);
    Some(m)
}

#[inline]
pub(crate) fn is_at_scss_interpolation(p: &mut CssParser) -> bool {
    is_nth_at_scss_interpolation(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_scss_interpolation(p: &mut CssParser, n: usize) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p) && p.nth_at(n, T![#]) && p.nth_at(n + 1, T!['{'])
}
