use crate::parser::TailwindParser;
use crate::syntax::css_value::{
    is_at_dashed_identifier, is_at_identifier, parse_css_generic_component_value_list,
    parse_css_parameter_list,
};
use crate::syntax::parse_error::expected_value;
use crate::token_source::TailwindLexContext;
use biome_parser::Parser;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::*;

/// Parses a candidate or modifier value. Tailwind accepts a type hint
/// (`bg-(color:--a)`) only in a candidate's CSS variable, so modifiers pass
/// `allow_type_hint: false`.
pub(crate) fn parse_value(p: &mut TailwindParser, allow_type_hint: bool) -> ParsedSyntax {
    if p.at(T!['[']) {
        return parse_arbitrary_value(p);
    }
    if p.at(T!['(']) {
        return parse_css_variable_value(p, allow_type_hint);
    }
    if p.at(TW_NUMBER) {
        return parse_numeric_value(p);
    }
    parse_named_value(p)
}

fn parse_named_value(p: &mut TailwindParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();
    if !p.expect(TW_VALUE) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    Present(m.complete(p, TW_NAMED_VALUE))
}

/// Parses a numeric value which can be either a number or a percentage (a number followed by a % sign).
fn parse_numeric_value(p: &mut TailwindParser) -> ParsedSyntax {
    if !p.at(TW_NUMBER) {
        return Absent;
    }
    let m = p.start();
    p.bump(TW_NUMBER);
    if p.eat(T![%]) {
        return Present(m.complete(p, TW_PERCENTAGE_VALUE));
    }

    Present(m.complete(p, TW_NUMBER_VALUE))
}

fn parse_arbitrary_value(p: &mut TailwindParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();
    if !p.expect_with_context(T!['['], TailwindLexContext::CssValue) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    if !parse_css_generic_component_value_list(p) {
        p.error(expected_value(p, p.cur_range()));
    }
    if !p.expect(T![']']) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    Present(m.complete(p, TW_ARBITRARY_VALUE))
}

fn parse_css_variable_value(p: &mut TailwindParser, allow_type_hint: bool) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();
    if !p.expect_with_context(T!['('], TailwindLexContext::CssValue) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    if allow_type_hint {
        parse_type_hint(p).ok();
    }
    let parameters = parse_css_parameter_list(p);
    if parameters.range(p).is_empty() {
        p.error(expected_value(p, p.cur_range()));
    }
    if !p.expect(T![')']) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    Present(m.complete(p, TW_CSS_VARIABLE_VALUE))
}

/// Parses the `color:` in `bg-(color:--brand)`.
fn parse_type_hint(p: &mut TailwindParser) -> ParsedSyntax {
    // A dashed identifier before a `:` is the variable itself (`bg-(--a:b)`),
    // which Tailwind rejects; leaving it to the parameter list reports the
    // whole value as invalid.
    if !is_at_identifier(p) || is_at_dashed_identifier(p) || !p.nth_at(1, T![:]) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(TW_VALUE, TailwindLexContext::CssValue);
    p.bump_with_context(T![:], TailwindLexContext::CssValue);
    Present(m.complete(p, TW_TYPE_HINT))
}
