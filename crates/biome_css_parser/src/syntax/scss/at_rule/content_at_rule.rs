use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::SCSS_CONTENT_AT_RULE;
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

/// Parses the SCSS `@content` at-rule.
///
/// # Example
///
/// ```scss
/// @content;
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/mixin/
#[inline]
pub(crate) fn parse_scss_content_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_content_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![content]);
    p.expect(T![;]);

    Present(m.complete(p, SCSS_CONTENT_AT_RULE))
}

#[inline]
fn is_at_scss_content_at_rule(p: &mut CssParser) -> bool {
    p.at(T![content])
}
