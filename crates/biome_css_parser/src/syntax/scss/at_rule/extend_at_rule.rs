use crate::parser::CssParser;
use crate::syntax::selector::SelectorList;
use biome_css_syntax::CssSyntaxKind::{self, SCSS_EXTEND_AT_RULE, SCSS_EXTEND_OPTIONAL_MODIFIER};
use biome_css_syntax::T;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{TokenSet, token_set};

const SCSS_EXTEND_SELECTOR_LIST_END_SET: TokenSet<CssSyntaxKind> =
    token_set![T![!], T![;], T!['{'], T!['}']];

/// Parses the SCSS `@extend` at-rule.
///
/// # Example
///
/// ```scss
/// @extend %toolbelt !optional;
/// @extend .message;
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/extend/
#[inline]
pub(crate) fn parse_scss_extend_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_extend_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![extend]);

    parse_scss_extend_selector_list(p).ok();

    // The `!optional` modifier is optional in the grammar, so `Absent` is valid here.
    parse_scss_extend_optional_modifier(p).ok();
    p.expect(T![;]);

    Present(m.complete(p, SCSS_EXTEND_AT_RULE))
}

#[inline]
fn is_at_scss_extend_at_rule(p: &mut CssParser) -> bool {
    p.at(T![extend])
}

#[inline]
fn parse_scss_extend_selector_list(p: &mut CssParser) -> ParsedSyntax {
    Present(
        SelectorList::default()
            .with_end_kind_ts(SCSS_EXTEND_SELECTOR_LIST_END_SET)
            .with_recovery_ts(SCSS_EXTEND_SELECTOR_LIST_END_SET)
            .parse_list(p),
    )
}

#[inline]
fn parse_scss_extend_optional_modifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_extend_optional_modifier(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![!]);
    p.expect(T![optional]);

    Present(m.complete(p, SCSS_EXTEND_OPTIONAL_MODIFIER))
}

#[inline]
fn is_at_scss_extend_optional_modifier(p: &mut CssParser) -> bool {
    p.at(T![!])
}
