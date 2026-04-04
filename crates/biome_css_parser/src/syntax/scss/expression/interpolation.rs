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

/// Controls how the closing `}` of an SCSS interpolation is lexed.
///
/// Most interpolation sites can finish in regular lexing mode, but selector
/// contexts must lex the closing brace with selector rules so following
/// whitespace becomes `CSS_SPACE_LITERAL` instead of being skipped as trivia.
#[derive(Clone, Copy)]
pub(crate) enum ScssInterpolationMode {
    /// Lexes the closing `}` with the regular CSS context.
    Regular,
    /// Lexes the closing `}` with selector rules so following whitespace stays
    /// visible as a selector combinator.
    Selector,
}

/// Parses an SCSS interpolation, lexing the closing `}` with the caller-provided
/// mode.
#[inline]
pub(crate) fn parse_scss_interpolation_with_mode(
    p: &mut CssParser,
    mode: ScssInterpolationMode,
) -> ParsedSyntax {
    if !is_at_scss_interpolation(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![#]);
    p.bump(T!['{']);
    parse_scss_inner_expression_until(p, SCSS_INTERPOLATION_END_TOKEN_SET)
        .or_add_diagnostic(p, expected_scss_expression);

    let closing_context = match mode {
        ScssInterpolationMode::Regular => CssLexContext::Regular,
        ScssInterpolationMode::Selector => selector_lex_context(p),
    };

    p.expect_with_context(T!['}'], closing_context);

    Present(m.complete(p, SCSS_INTERPOLATION))
}

#[inline]
pub(crate) fn is_at_scss_interpolation(p: &mut CssParser) -> bool {
    is_nth_at_scss_interpolation(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_scss_interpolation(p: &mut CssParser, n: usize) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p) && p.nth_at(n, T![#]) && p.nth_at(n + 1, T!['{'])
}
