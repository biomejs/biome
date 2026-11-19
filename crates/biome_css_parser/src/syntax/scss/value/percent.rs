use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::SCSS_PERCENT_VALUE;
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

#[inline]
pub(crate) fn is_at_scss_percent_value(p: &mut CssParser) -> bool {
    p.at(T![%])
}

/// Parses `%` as an unquoted Sass string in operand position.
///
/// Numeric percentages have a distinct lexer token, while infix `%` is handled
/// by the expression parser. For example, `$unit: %` stores a string.
///
/// Docs: https://sass-lang.com/documentation/values/strings/#other-unquoted-strings
#[inline]
pub(crate) fn parse_scss_percent_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_percent_value(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![%]);
    Present(m.complete(p, SCSS_PERCENT_VALUE))
}
