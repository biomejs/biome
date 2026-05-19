use crate::parser::CssParser;
use crate::syntax::parse_error::{
    expected_any_attribute_matcher_name, expected_any_attribute_modifier, expected_identifier,
};
use crate::syntax::scss::{
    is_at_scss_interpolated_attribute_identifier, is_at_scss_interpolated_string,
    parse_scss_interpolated_attribute_modifier, parse_scss_interpolated_identifier,
    parse_scss_interpolated_string,
};
use crate::syntax::selector::{is_nth_at_namespace, parse_namespace, selector_lex_context};
use crate::syntax::{is_at_identifier, is_at_string, parse_regular_identifier, parse_string};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::diagnostic::expected_token;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, TokenSet, token_set};

const ATTRIBUTE_SELECTOR_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T![')'], T!['{']];
#[inline]
pub(crate) fn parse_attribute_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }
    let m = p.start();

    p.bump(T!['[']);
    // we have diagnostic inside `parse_attribute_name` method
    parse_attribute_name(p).ok();

    // `parse_attribute_matcher` method is invoked with `ok()`,
    // which turns an `Err` into an `Ok` variant, because attribute matcher in a CSS attribute selector
    // is optional. If it isn't present or fails to parse correctly, parsing can continue.
    parse_attribute_matcher(p).ok();

    let context = selector_lex_context(p);
    if !p.eat_with_context(T![']'], context)
        && ParseRecoveryTokenSet::new(CSS_BOGUS, ATTRIBUTE_SELECTOR_RECOVERY_SET)
            .recover(p)
            .is_err()
    {
        p.error(expected_token(T![']']));
    }

    Present(m.complete(p, CSS_ATTRIBUTE_SELECTOR))
}

fn is_at_attribute_name(p: &mut CssParser) -> bool {
    is_at_attribute_name_identifier(p) || is_nth_at_namespace(p, 0)
}

/// Parses an attribute name with an optional namespace.
///
/// Examples:
/// ```scss
/// [data-#{$name}]
/// [ns|data-#{$name}]
/// ```
#[inline]
pub(crate) fn parse_attribute_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attribute_name(p) {
        return Absent;
    }

    let m = p.start();
    // we don't need diagnostic here, because namespace is optional
    parse_namespace(p).ok();
    parse_attribute_name_identifier(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, CSS_ATTRIBUTE_NAME))
}

#[inline]
fn is_at_attribute_name_identifier(p: &mut CssParser) -> bool {
    is_at_identifier(p) || is_at_scss_interpolated_attribute_identifier(p)
}

#[inline]
fn parse_attribute_name_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attribute_name_identifier(p) {
        return Absent;
    }

    if is_at_scss_interpolated_attribute_identifier(p) {
        parse_scss_interpolated_identifier(p)
    } else {
        parse_regular_identifier(p)
    }
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
    parse_attribute_matcher_value(p).or_add_diagnostic(p, expected_any_attribute_matcher_name);

    let modifier = parse_attribute_modifier(p);

    // `[title="x" s {}` already has a modifier; let the selector recovery
    // report the missing `]` instead of treating `{` as another modifier.
    if modifier.is_absent() && !p.at(T![']']) && !p.at(EOF) {
        // if we have an invalid modifier, we should add a diagnostic and bump it
        let diagnostic = expected_any_attribute_modifier(p, p.cur_range());
        p.error(diagnostic);
        p.bump_any();
    }

    Present(m.complete(p, CSS_ATTRIBUTE_MATCHER))
}

#[inline]
fn is_at_attribute_matcher_value(p: &mut CssParser) -> bool {
    is_at_scss_interpolated_attribute_identifier(p)
        || is_at_identifier(p)
        || is_at_string(p)
        || is_at_scss_interpolated_string(p)
}

#[inline]
fn is_at_attribute_modifier(p: &mut CssParser) -> bool {
    is_at_attribute_modifier_keyword(p) || is_at_scss_interpolated_attribute_identifier(p)
}

/// Parses an attribute selector modifier.
///
/// Examples:
/// ```scss
/// [title="x" i]
/// [title="x" #{$mod}]
/// ```
#[inline]
fn parse_attribute_modifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attribute_modifier(p) {
        return Absent;
    }

    if is_at_scss_interpolated_attribute_identifier(p) {
        parse_scss_interpolated_attribute_modifier(p)
    } else {
        parse_attribute_modifier_keyword(p)
    }
}

#[inline]
fn is_at_attribute_modifier_keyword(p: &mut CssParser) -> bool {
    p.cur().is_attribute_modifier_keyword()
}

/// Parses the case-sensitivity modifier in `[title="x" i]`.
#[inline]
fn parse_attribute_modifier_keyword(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attribute_modifier_keyword(p) {
        return Absent;
    }

    let m = p.start();
    // `parse_attribute_modifier` guards this branch with
    // `is_at_attribute_modifier_keyword`.
    p.bump_any();
    Present(m.complete(p, CSS_ATTRIBUTE_MODIFIER))
}

#[inline]
fn parse_attribute_matcher_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_attribute_matcher_value(p) {
        return Absent;
    }

    let m = p.start();

    if is_at_scss_interpolated_attribute_identifier(p) {
        parse_scss_interpolated_identifier(p).ok();
    } else if is_at_scss_interpolated_string(p) {
        parse_scss_interpolated_string(p).ok();
    } else if is_at_string(p) {
        parse_string(p).ok();
    } else {
        parse_regular_identifier(p).ok();
    }

    Present(m.complete(p, CSS_ATTRIBUTE_MATCHER_VALUE))
}
