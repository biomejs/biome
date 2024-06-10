use crate::parser::CssParser;
use crate::syntax::block::parse_conditional_block;
use crate::syntax::selector::parse_selector;
use crate::syntax::value::function::{is_at_function, parse_function};
use crate::syntax::{is_nth_at_identifier, parse_any_value, parse_declaration};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

#[inline]
pub(crate) fn is_at_supports_at_rule(p: &mut CssParser) -> bool {
    p.at(T![supports])
}

#[inline]
pub(crate) fn parse_supports_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_supports_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![supports]);

    parse_any_supports_condition(p).ok(); // TODO handle error
    parse_conditional_block(p);

    Present(m.complete(p, CSS_SUPPORTS_AT_RULE))
}

#[inline]
pub(crate) fn parse_any_supports_condition(p: &mut CssParser) -> ParsedSyntax {
    if is_at_supports_not_condition(p) {
        parse_supports_not_condition(p)
    } else {
        let condition_in_parens = parse_any_supports_condition_in_parens(p);

        match p.cur() {
            T![and] => {
                let m = condition_in_parens.precede(p);
                p.bump(T![and]);
                parse_supports_and_condition(p).ok(); // TODO handle error
                Present(m.complete(p, CSS_SUPPORTS_AND_CONDITION))
            }
            T![or] => {
                let m = condition_in_parens.precede(p);
                p.bump(T![or]);
                parse_supports_or_condition(p).ok(); // TODO handle error
                Present(m.complete(p, CSS_SUPPORTS_OR_CONDITION))
            }
            _ => condition_in_parens,
        }
    }
}

#[inline]
fn parse_supports_and_condition(p: &mut CssParser) -> ParsedSyntax {
    let condition_in_parens = parse_any_supports_condition_in_parens(p);

    if p.at(T![and]) {
        let m = condition_in_parens.precede(p);
        p.bump(T![and]);
        parse_supports_and_condition(p).ok(); // TODO handle error
        Present(m.complete(p, CSS_SUPPORTS_AND_CONDITION))
    } else {
        condition_in_parens
    }
}

#[inline]
fn parse_supports_or_condition(p: &mut CssParser) -> ParsedSyntax {
    let condition_in_parens = parse_any_supports_condition_in_parens(p);

    if p.at(T![or]) {
        let m = condition_in_parens.precede(p);
        p.bump(T![or]);
        parse_supports_or_condition(p).ok(); // TODO handle error
        Present(m.complete(p, CSS_SUPPORTS_OR_CONDITION))
    } else {
        condition_in_parens
    }
}

#[inline]
pub(crate) fn is_at_supports_not_condition(p: &mut CssParser) -> bool {
    p.at(T![not])
}
#[inline]
fn parse_supports_not_condition(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_supports_not_condition(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![not]);
    parse_any_supports_condition_in_parens(p).ok(); // TODO handle error

    Present(m.complete(p, CSS_SUPPORTS_NOT_CONDITION))
}
#[inline]
fn parse_any_supports_condition_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if is_at_supports_feature_selector(p) {
        parse_supports_feature_selector(p)
    } else if is_at_supports_feature_declaration(p) {
        parse_supports_feature_declaration(p)
    } else if is_at_function(p) {
        parse_function(p)
    } else if is_at_supports_condition_in_parens(p) {
        parse_supports_condition_in_parens(p) // TODO handle error
    } else {
        parse_any_value(p)
    }
}

#[inline]
fn is_at_supports_condition_in_parens(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
fn parse_supports_condition_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_supports_condition_in_parens(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    parse_any_supports_condition(p).ok(); // TODO handle error
    p.bump(T![')']);

    Present(m.complete(p, CSS_SUPPORTS_CONDITION_IN_PARENS))
}

#[inline]
fn is_at_supports_feature_selector(p: &mut CssParser) -> bool {
    p.at(T![selector]) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_supports_feature_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_supports_feature_selector(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![selector]);
    p.bump(T!['(']);
    parse_selector(p).ok(); // TODO handle error
    p.expect(T![')']); // TODO handle error

    Present(m.complete(p, CSS_SUPPORTS_FEATURE_SELECTOR))
}

#[inline]
fn is_at_supports_feature_declaration(p: &mut CssParser) -> bool {
    p.at(T!['(']) && is_nth_at_identifier(p, 1) && p.nth_at(2, T![:])
}

#[inline]
fn parse_supports_feature_declaration(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_supports_feature_declaration(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    parse_declaration(p).ok(); // TODO handle error
    p.expect(T![')']); // TODO handle error

    Present(m.complete(p, CSS_SUPPORTS_FEATURE_DECLARATION))
}
