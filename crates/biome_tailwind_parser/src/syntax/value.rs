use crate::parser::TailwindParser;
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

    if p.at(T![-]) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    Present(m.complete(p, TW_NAMED_VALUE))
}

fn parse_arbitrary_value(p: &mut TailwindParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();
    if !p.expect_with_context(T!['['], TailwindLexContext::Arbitrary) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    if !p.expect_with_context(TW_VALUE, TailwindLexContext::Arbitrary) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
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
