use crate::parser::HtmlParser;
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::T;
use biome_parser::Parser;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

pub(crate) fn parse_angular_event_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    println!("🟢1");
    let m = p.start();

    // TODO

    Present(m.complete(p, ANGULAR_EVENT_BINDING))
}

pub(crate) fn parse_angular_property_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    println!("🟢2");
    let m = p.start();

    // TODO

    Present(m.complete(p, ANGULAR_PROPERTY_BINDING))
}

pub(crate) fn parse_angular_two_way_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["[("]) {
        return Absent;
    }

    println!("🟢3");
    let m = p.start();

    // TODO

    Present(m.complete(p, ANGULAR_TWO_WAY_BINDING))
}

pub(crate) fn parse_angular_structural_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![*]) {
        return Absent;
    }

    println!("🟢4");
    let m = p.start();

    // TODO

    Present(m.complete(p, ANGULAR_TWO_WAY_BINDING))
}

pub(crate) fn parse_angular_template_ref(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![#]) {
        return Absent;
    }

    println!("🟢5");
    let m = p.start();

    // TODO

    Present(m.complete(p, ANGULAR_TWO_WAY_BINDING))
}
