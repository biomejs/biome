use crate::parser::TailwindParser;
use crate::syntax::atoms::{ValueEnd, css_token, value};
use crate::syntax::css_value::{
    is_at_dashed_identifier, is_identifier, parse_css_generic_component_value_list,
    parse_css_parameter_list,
};
use crate::syntax::parse_error::expected_value;
use biome_parser::Parser;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::*;

/// Parses a candidate value, after its `-`, or a modifier value, after its `/`. Tailwind accepts a
/// type hint (`bg-(color:--a)`) only in a candidate's CSS variable.
pub(crate) fn parse_value(p: &mut TailwindParser, end: ValueEnd) -> ParsedSyntax {
    if p.at(T!['[']) {
        return parse_arbitrary_value(p);
    }
    if p.at(T!['(']) {
        return parse_css_variable_value(p, end == ValueEnd::Candidate);
    }
    if matches!(p.cur(), WHITESPACE | T![')'] | T![']'] | EOF) {
        return Absent;
    }
    let (kind, atoms) = value(p.source(), end);
    // A candidate's value can be empty (`bg-%` is the percentage of an empty number), but a
    // modifier can't.
    if atoms == 0 && end == ValueEnd::Modifier {
        return Absent;
    }
    let m = p.start();
    p.bump_atoms(kind, atoms);
    if kind == TW_VALUE {
        return Present(m.complete(p, TW_NAMED_VALUE));
    }
    if p.eat(T![%]) {
        return Present(m.complete(p, TW_PERCENTAGE_VALUE));
    }
    Present(m.complete(p, TW_NUMBER_VALUE))
}

fn parse_arbitrary_value(p: &mut TailwindParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();
    if !p.expect_css(T!['[']) {
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
    if !p.expect_css(T!['(']) {
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
    // which Tailwind rejects; leaving it to the parameter list reports what
    // follows the variable as invalid.
    let (kind, atoms) = css_token(p.source(), 0);
    if !is_identifier(kind) || is_at_dashed_identifier(p) || p.source().nth_kind(atoms) != T![:] {
        return Absent;
    }

    let m = p.start();
    p.bump_css_atoms(TW_VALUE, atoms);
    p.bump_css(T![:]);
    Present(m.complete(p, TW_TYPE_HINT))
}
