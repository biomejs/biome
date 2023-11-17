use crate::parser::CssParser;
use crate::syntax::parse_error::{
    expect_any_attribute_matcher_name, expect_any_attribute_modifier, expected_identifier,
};
use crate::syntax::selector::selector_lex_context;
use crate::syntax::{is_at_identifier, parse_css_string, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::diagnostic::expected_token;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

const ATTRIBUTE_SELECTOR_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T![')'], T!['{']];
#[inline]
pub(crate) fn parse_attribute_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }
    let m = p.start();

    p.bump(T!['[']);
    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);

    // `parse_attribute_matcher` method is invoked with `ok()`,
    // which turns an `Err` into an `Ok` variant, because attribute matcher in a CSS attribute selector
    // is optional. If it isn't present or fails to parse correctly, parsing can continue.
    parse_attribute_matcher(p).ok();

    let context = selector_lex_context(p);
    if !p.eat_with_context(T![']'], context)
        && ParseRecovery::new(CSS_BOGUS, ATTRIBUTE_SELECTOR_RECOVERY_SET)
            .recover(p)
            .is_err()
    {
        p.error(expected_token(T![']']));
    }

    Present(m.complete(p, CSS_ATTRIBUTE_SELECTOR))
}

const ATTRIBUTE_MATCHER_SET: TokenSet<CssSyntaxKind> =
    token_set![T![~=], T![|=], T![^=], T!["$="], T![*=], T![=]];
#[inline]
fn is_at_attribute_matcher(p: &mut CssParser) -> bool {
    p.at_ts(ATTRIBUTE_MATCHER_SET)
}

#[inline]
fn parse_attribute_matcher(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attribute_matcher(p) {
        return Absent;
    }

    let m = p.start();

    // bump attribute matcher type
    p.bump_any();
    parse_attribute_matcher_value(p).or_add_diagnostic(p, expect_any_attribute_matcher_name);

    let modifier = p.cur();
    if modifier.is_attribute_modifier_keyword() {
        p.bump(modifier);
    } else if modifier != T![']'] {
        // if we have an invalid modifier, we should add a diagnostic and bump it
        let diagnostic = expect_any_attribute_modifier(p, p.cur_range());
        p.error(diagnostic);
        p.bump_any();
    }

    Present(m.complete(p, CSS_ATTRIBUTE_MATCHER))
}

#[inline]
fn is_at_attribute_matcher_value(p: &mut CssParser) -> bool {
    is_at_identifier(p) || p.at(CSS_STRING_LITERAL)
}
#[inline]
fn parse_attribute_matcher_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attribute_matcher_value(p) {
        return Absent;
    }

    let m = p.start();

    if p.at(CSS_STRING_LITERAL) {
        parse_css_string(p).ok();
    } else {
        parse_regular_identifier(p).ok();
    }

    Present(m.complete(p, CSS_ATTRIBUTE_MATCHER_VALUE))
}
