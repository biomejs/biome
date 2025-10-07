use crate::parser::HtmlParser;
use crate::syntax::parse_attribute_initializer;
use crate::syntax::parse_error::expected_attribute;
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind;
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::T;
use biome_parser::Parser;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

pub fn parse_vue_directive(p: &mut HtmlParser) -> ParsedSyntax {
    let m = p.start();

    p.bump_with_context(HTML_LITERAL, HtmlLexContext::InsideTag);
    if p.at(T![:]) {
        parse_vue_directive_argument(p).ok();
    }
    VueModifierList.parse_list(p);
    if p.at(T![=]) {
        parse_attribute_initializer(p).ok();
    }

    Present(m.complete(p, VUE_DIRECTIVE))
}

pub fn parse_vue_v_bind_shorthand_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }

    let m = p.start();

    parse_vue_directive_argument(p).ok();
    VueModifierList.parse_list(p);
    if p.at(T![=]) {
        parse_attribute_initializer(p).ok();
    }

    Present(m.complete(p, VUE_V_BIND_SHORTHAND_DIRECTIVE))
}

pub fn parse_vue_v_on_shorthand_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![@]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T![@], HtmlLexContext::InsideTag);
    parse_vue_dynamic_argument(p)
        .or_else(|| parse_vue_static_argument(p))
        .ok();
    VueModifierList.parse_list(p);
    if p.at(T![=]) {
        parse_attribute_initializer(p).ok();
    }

    Present(m.complete(p, VUE_V_ON_SHORTHAND_DIRECTIVE))
}

fn parse_vue_directive_argument(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T![:], HtmlLexContext::InsideTag);
    parse_vue_dynamic_argument(p)
        .or_else(|| parse_vue_static_argument(p))
        .ok();

    Present(m.complete(p, VUE_DIRECTIVE_ARGUMENT))
}

fn parse_vue_static_argument(p: &mut HtmlParser) -> ParsedSyntax {
    let m = p.start();

    p.expect_with_context(HTML_LITERAL, HtmlLexContext::InsideTag);

    Present(m.complete(p, VUE_STATIC_ARGUMENT))
}

fn parse_vue_dynamic_argument(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();

    p.expect_with_context(T!['['], HtmlLexContext::InsideTag);
    p.expect_with_context(HTML_LITERAL, HtmlLexContext::InsideTag);
    p.expect_with_context(T![']'], HtmlLexContext::InsideTag);

    Present(m.complete(p, VUE_DYNAMIC_ARGUMENT))
}

struct VueModifierList;

impl ParseNodeList for VueModifierList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = VUE_MODIFIER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_vue_modifier(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![=])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS_ATTRIBUTE, token_set![T![.], T![>]]),
            expected_attribute,
        )
    }
}

fn parse_vue_modifier(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![.]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T![.], HtmlLexContext::InsideTag);
    p.bump_with_context(HTML_LITERAL, HtmlLexContext::InsideTag);

    Present(m.complete(p, VUE_MODIFIER))
}
