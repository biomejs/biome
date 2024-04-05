mod parse_error;

use crate::parser::HtmlParser;
use crate::syntax::parse_error::{expected_attribute, expected_child};
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind::{
    HTML_ATTRIBUTE, HTML_ATTRIBUTE_INITIALIZER_CLAUSE, HTML_ATTRIBUTE_LIST, HTML_BOGUS_ELEMENT,
    HTML_CLOSING_ELEMENT, HTML_CONTENT, HTML_DIRECTIVE, HTML_ELEMENT, HTML_ELEMENT_LIST,
    HTML_LITERAL, HTML_NAME, HTML_OPENING_ELEMENT, HTML_ROOT, HTML_SELF_CLOSING_ELEMENT,
    HTML_STRING, HTML_STRING_LITERAL, UNICODE_BOM,
};
use biome_html_syntax::{HtmlSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;
use biome_parser::Parser;

const RECOVER_ATTRIBUTE_LIST: TokenSet<HtmlSyntaxKind> = token_set!(T![>], T![<], T![/]);

pub(crate) fn parse_root(p: &mut HtmlParser) {
    let m = p.start();

    p.eat(UNICODE_BOM);

    parse_doc_type(p).ok();
    parse_element(p).ok();

    m.complete(p, HTML_ROOT);
}

fn parse_doc_type(p: &mut HtmlParser) -> ParsedSyntax {
    if !(p.at(T![<]) && p.nth_at(1, T![!])) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![<]);
    p.bump(T![!]);

    if p.at(T![doctype]) {
        p.eat(T![doctype]);
    }

    p.eat(T![>]);

    Present(m.complete(p, HTML_DIRECTIVE))
}

fn parse_element(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![<]) {
        return Absent;
    }
    let m = p.start();

    p.bump(T![<]);
    // TODO: handle error
    parse_literal(p).ok();

    AttributeList.parse_list(p);

    if p.at(T![/]) {
        p.bump(T![/]);
        p.bump(T![>]);
        Present(m.complete(p, HTML_SELF_CLOSING_ELEMENT))
    } else {
        p.bump_with_context(T![>], HtmlLexContext::ElementList);
        let opening = m.complete(p, HTML_OPENING_ELEMENT);
        ElementList.parse_list(p);
        // TODO: handle error
        parse_closing_element(p).ok();
        let previous = opening.precede(p);

        Present(previous.complete(p, HTML_ELEMENT))
    }
}

fn parse_closing_element(p: &mut HtmlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![<]);
    p.bump(T![/]);
    let _name = parse_literal(p);
    p.bump(T![>]);
    Present(m.complete(p, HTML_CLOSING_ELEMENT))
}

#[derive(Default)]
struct ElementList;

impl ParseNodeList for ElementList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = HTML_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        match p.cur() {
            T![<] => parse_element(p),
            HTML_LITERAL => {
                let m = p.start();
                p.bump(HTML_LITERAL);
                Present(m.complete(p, HTML_CONTENT))
            }
            _ => Absent,
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        let at_l_angle0 = p.at(T![<]);
        let at_slash1 = p.nth_at(1, T![/]);
        at_l_angle0 && at_slash1
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS_ELEMENT, token_set![T![<], T![>]]),
            expected_child,
        )
    }
}

#[derive(Default)]
struct AttributeList;

impl ParseNodeList for AttributeList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = HTML_ATTRIBUTE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        // Absent
        parse_attribute(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![>]) || p.at(T![/])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS_ELEMENT, RECOVER_ATTRIBUTE_LIST),
            expected_attribute,
        )
    }
}

fn parse_attribute(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(HTML_LITERAL) {
        return Absent;
    }
    let m = p.start();
    // TODO: handle error
    parse_literal(p).ok();
    if p.at(T![=]) {
        parse_attribute_initializer(p).ok();
        Present(m.complete(p, HTML_ATTRIBUTE))
    } else {
        Present(m.complete(p, HTML_ATTRIBUTE))
    }
}

fn parse_literal(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(HTML_LITERAL) {
        return Absent;
    }
    let m = p.start();

    p.bump(HTML_LITERAL);

    Present(m.complete(p, HTML_NAME))
}

fn parse_string_literal(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(HTML_STRING_LITERAL) {
        return Absent;
    }
    let m = p.start();

    p.bump(HTML_STRING_LITERAL);

    Present(m.complete(p, HTML_STRING))
}

fn parse_attribute_initializer(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![=]);
    // TODO: handle error
    parse_string_literal(p).ok();
    Present(m.complete(p, HTML_ATTRIBUTE_INITIALIZER_CLAUSE))
}
