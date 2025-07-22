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
    let m = p.start();
    p.expect(TW_VALUE);

    if p.at(T![-]) {
        return Absent;
    }

    Present(m.complete(p, TW_NAMED_VALUE))
}

fn parse_arbitrary_value(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    p.expect_with_context(T!['['], TailwindLexContext::Arbitrary);
    p.expect_with_context(TW_VALUE, TailwindLexContext::Arbitrary);
    p.expect(T![']']);

    Present(m.complete(p, TW_ARBITRARY_VALUE))
}

fn parse_css_variable_value(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    p.expect(T!['(']);
    p.expect(TW_VALUE);
    p.expect(T![')']);

    Present(m.complete(p, TW_CSS_VARIABLE_VALUE))
}
