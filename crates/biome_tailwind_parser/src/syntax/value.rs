use crate::parser::TailwindParser;
use crate::syntax::css_value::parse_css_generic_component_value_list;
use crate::syntax::variant::parse_data_attribute;
use crate::token_source::TailwindLexContext;
use biome_parser::Parser;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::*;

pub(crate) fn parse_value(p: &mut TailwindParser) -> ParsedSyntax {
    if p.at(T!['[']) {
        return parse_arbitrary_value(p);
    }
    if p.at(T!['(']) {
        return parse_css_variable_value(p);
    }
    if p.at(T![data]) {
        return parse_data_attribute(p);
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

/// Parses a numeric value which can be either a number, a ratio of two numbers, or a percentage (a number followed by a % sign).
fn parse_numeric_value(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    if !p.expect(TW_NUMBER) {
        m.abandon(p);
        return Absent;
    }

    if p.at(T![%]) {
        p.bump(T![%]);
        return Present(m.complete(p, TW_PERCENTAGE_VALUE));
    }

    let number = m.complete(p, TW_NUMBER_VALUE);

    if p.at(T![/]) {
        let ratio = number.precede(p);
        p.bump(T![/]);
        parse_number_only(p).or_add_diagnostic(p, crate::syntax::parse_error::expected_value);
        return Present(ratio.complete(p, TW_RATIO_VALUE));
    }

    Present(number)
}

fn parse_number_only(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    if !p.expect(TW_NUMBER) {
        m.abandon(p);
        return Absent;
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
        p.error(crate::syntax::parse_error::expected_value(p, p.cur_range()));
    }
    if !p.expect(T![']']) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    Present(m.complete(p, TW_ARBITRARY_VALUE))
}

fn parse_css_variable_value(p: &mut TailwindParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();
    if !p.expect(T!['(']) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    if !p.expect(TW_VALUE) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    if !p.expect(T![')']) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    Present(m.complete(p, TW_CSS_VARIABLE_VALUE))
}
