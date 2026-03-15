use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::SCSS_INTERPOLATION;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{TokenSet, token_set};

use super::list::parse_scss_inner_expression_until;
use crate::syntax::scss::expected_scss_expression;

const SCSS_INTERPOLATION_END_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set![T!['}'], T![;]];

/// Parses a standalone SCSS interpolation expression such as `#{$value}`.
///
/// Example:
/// ```scss
/// $value: #{$name};
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(super) fn parse_scss_interpolation(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolation(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![#]);
    p.bump(T!['{']);
    parse_scss_inner_expression_until(p, SCSS_INTERPOLATION_END_TOKEN_SET)
        .or_add_diagnostic(p, expected_scss_expression);
    p.expect(T!['}']);

    Present(m.complete(p, SCSS_INTERPOLATION))
}

#[inline]
pub(super) fn is_at_scss_interpolation(p: &mut CssParser) -> bool {
    p.at(T![#]) && p.nth_at(1, T!['{'])
}
