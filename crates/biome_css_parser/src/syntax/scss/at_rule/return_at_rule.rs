use crate::parser::CssParser;
use crate::syntax::scss::at_rule::parse_scss_expression_at_rule;
use biome_css_syntax::CssSyntaxKind::SCSS_RETURN_AT_RULE;
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

/// Parses the SCSS `@return` at-rule.
///
/// # Example
///
/// ```scss
/// @return $width * 2;
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/function/
#[inline]
pub(crate) fn parse_scss_return_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_return_at_rule(p) {
        return Absent;
    }

    parse_scss_expression_at_rule(p, T![return], SCSS_RETURN_AT_RULE)
}

#[inline]
fn is_at_scss_return_at_rule(p: &mut CssParser) -> bool {
    p.at(T![return])
}
