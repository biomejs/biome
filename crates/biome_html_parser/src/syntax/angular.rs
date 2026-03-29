use crate::parser::HtmlParser;
use crate::syntax::parse_attribute_initializer;
use crate::syntax::parse_error::expected_angular_name;
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

    let m = p.start();

    p.bump_with_context(T!['('], HtmlLexContext::InsideTagAngular);
    parse_angular_binding_name(p).ok();
    p.expect_with_context(T![')'], HtmlLexContext::InsideTagAngular);
    if p.at(T![=]) {
        parse_attribute_initializer(p).ok();
    }

    Present(m.complete(p, ANGULAR_EVENT_BINDING))
}

fn parse_angular_binding_name(p: &mut HtmlParser) -> ParsedSyntax {
    let m = p.start();

    if p.at(HTML_LITERAL) {
        p.bump_with_context(HTML_LITERAL, HtmlLexContext::InsideTagAngular);
    } else {
        p.error(expected_angular_name(p, p.cur_range()));
    }

    Present(m.complete(p, ANGULAR_BINDING_NAME))
}

pub(crate) fn parse_angular_property_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T!['['], HtmlLexContext::InsideTagAngular);
    parse_angular_binding_name(p).ok();
    p.expect_with_context(T![']'], HtmlLexContext::InsideTagAngular);
    if p.at(T![=]) {
        parse_attribute_initializer(p).ok();
    }

    Present(m.complete(p, ANGULAR_PROPERTY_BINDING))
}

pub(crate) fn parse_angular_two_way_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["[("]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T!["[("], HtmlLexContext::InsideTagAngular);
    parse_angular_binding_name(p).ok();
    p.expect_with_context(T![")]"], HtmlLexContext::InsideTagAngular);
    if p.at(T![=]) {
        parse_attribute_initializer(p).ok();
    }

    Present(m.complete(p, ANGULAR_TWO_WAY_BINDING))
}

pub(crate) fn parse_angular_structural_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![*]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T![*], HtmlLexContext::InsideTagAngular);
    parse_angular_binding_name(p).ok();
    if p.at(T![=]) {
        parse_attribute_initializer(p).ok();
    }

    Present(m.complete(p, ANGULAR_STRUCTURAL_DIRECTIVE))
}

pub(crate) fn parse_angular_template_ref(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![#]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T![#], HtmlLexContext::InsideTagAngular);
    parse_angular_binding_name(p).ok();
    if p.at(T![=]) {
        parse_attribute_initializer(p).ok();
    }

    Present(m.complete(p, ANGULAR_TEMPLATE_REF_VARIABLE))
}
