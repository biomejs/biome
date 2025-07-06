mod astro;
mod parse_error;

use crate::parser::HtmlParser;
use crate::syntax::astro::{
    is_at_fence, parse_astro_fence, is_at_astro_expression, parse_astro_expression,
    is_at_astro_spread_attribute, parse_astro_spread_attribute, parse_astro_shorthand_attribute,
    parse_astro_expression_attribute, parse_astro_template_literal_attribute
};
use crate::syntax::parse_error::*;
use crate::token_source::{HtmlEmbeddedLanguage, HtmlLexContext};
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::{HtmlSyntaxKind, T};
use biome_parser::Parser;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

const RECOVER_ATTRIBUTE_LIST: TokenSet<HtmlSyntaxKind> = token_set!(T![>], T![<], T![/]);

/// These elements are effectively always self-closing. They should not have a closing tag (if they do, it should be a parsing error). They might not contain a `/` like in `<img />`.
static VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

/// For these elements, the content is treated as raw text and no parsing is done inside them. This is so that the contents of these tags can be parsed by a different parser.
pub(crate) static EMBEDDED_LANGUAGE_ELEMENTS: &[&str] = &["script", "style", "pre"];

/// Check if a tag name represents an Astro component (starts with uppercase) or regular HTML element
fn is_astro_component(tag_name: &str) -> bool {
    tag_name.chars().next().map_or(false, |c| c.is_ascii_uppercase())
}

pub(crate) fn parse_root(p: &mut HtmlParser) {
    let m = p.start();

    p.eat(UNICODE_BOM);

    if is_at_fence(p) {
        parse_astro_fence(p).ok();
    }
    parse_doc_type(p).ok();
    ElementList.parse_list(p);

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
        p.eat_with_context(T![doctype], HtmlLexContext::Doctype);
    }

    if p.at(T![html]) {
        p.eat_with_context(T![html], HtmlLexContext::Doctype);
    }

    if p.at(HTML_LITERAL) {
        p.eat_with_context(HTML_LITERAL, HtmlLexContext::Doctype);
    }

    if p.at(HTML_STRING_LITERAL) {
        p.eat_with_context(HTML_STRING_LITERAL, HtmlLexContext::Doctype);
    }

    if p.at(HTML_STRING_LITERAL) {
        p.eat_with_context(HTML_STRING_LITERAL, HtmlLexContext::Doctype);
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
    let opening_tag_name = p.cur_text().to_string();
    let is_component = is_astro_component(&opening_tag_name);
    let should_be_self_closing = !is_component && VOID_ELEMENTS
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case(opening_tag_name.as_str()));
    let is_embedded_language_tag = !is_component && EMBEDDED_LANGUAGE_ELEMENTS
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case(opening_tag_name.as_str()));
    parse_literal(p, HTML_TAG_NAME).or_add_diagnostic(p, expected_element_name);

    AttributeList.parse_list(p);

    if p.at(T![/]) {
        p.bump(T![/]);
        p.expect_with_context(T![>], HtmlLexContext::OutsideTag);
        Present(m.complete(p, HTML_SELF_CLOSING_ELEMENT))
    } else {
        if should_be_self_closing {
            if p.at(T![/]) {
                p.bump(T![/]);
            }
            p.expect_with_context(T![>], HtmlLexContext::OutsideTag);
            return Present(m.complete(p, HTML_SELF_CLOSING_ELEMENT));
        }
        p.expect_with_context(
            T![>],
            if is_embedded_language_tag {
                HtmlLexContext::EmbeddedLanguage(match opening_tag_name.as_str() {
                    tag if tag.eq_ignore_ascii_case("script") => HtmlEmbeddedLanguage::Script,
                    tag if tag.eq_ignore_ascii_case("style") => HtmlEmbeddedLanguage::Style,
                    tag if tag.eq_ignore_ascii_case("pre") => HtmlEmbeddedLanguage::Preformatted,
                    _ => unreachable!(),
                })
            } else {
                HtmlLexContext::OutsideTag
            },
        );
        let opening = m.complete(p, HTML_OPENING_ELEMENT);
        loop {
            ElementList.parse_list(p);
            if let Some(mut closing) =
                parse_closing_tag(p).or_add_diagnostic(p, expected_closing_tag)
            {
                if !closing.text(p).contains(opening_tag_name.as_str()) {
                    p.error(expected_matching_closing_tag(p, closing.range(p)).into_diagnostic(p));
                    closing.change_to_bogus(p);
                    continue;
                }
            }
            break;
        }
        let previous = opening.precede(p);

        Present(previous.complete(p, HTML_ELEMENT))
    }
}

fn parse_closing_tag(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![<]) || !p.nth_at(1, T![/]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![<]);
    p.bump(T![/]);
    let should_be_self_closing = VOID_ELEMENTS
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case(p.cur_text()));
    if should_be_self_closing {
        p.error(void_element_should_not_have_closing_tag(p, p.cur_range()).into_diagnostic(p));
    }
    let _name = parse_literal(p, HTML_TAG_NAME);

    // There shouldn't be any attributes in a closing tag.
    while p.at(HTML_LITERAL) {
        p.error(closing_tag_should_not_have_attributes(p, p.cur_range()));
        p.bump_remap(HTML_BOGUS);
    }
    p.bump_with_context(T![>], HtmlLexContext::OutsideTag);
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
            T![<!--] => parse_comment(p),
            T!["<![CDATA["] => parse_cdata_section(p),
            T![<] => parse_element(p),
            T!['{'] if is_at_astro_expression(p) => parse_astro_expression(p),
            HTML_LITERAL => {
                let m = p.start();
                p.bump_with_context(HTML_LITERAL, HtmlLexContext::OutsideTag);
                Present(m.complete(p, HTML_CONTENT))
            }
            _ => Absent,
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        let at_l_angle0 = p.at(T![<]);
        let at_slash1 = p.nth_at(1, T![/]);
        let at_eof = p.at(EOF);
        at_l_angle0 && at_slash1 || at_eof
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
        if is_at_astro_spread_attribute(p) {
            parse_astro_spread_attribute(p)
        } else if is_at_astro_expression(p) {
            parse_astro_shorthand_attribute(p)
        } else {
            parse_attribute(p)
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![>]) || p.at(T![/]) || p.at(EOF)
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
    parse_literal(p, HTML_ATTRIBUTE_NAME).or_add_diagnostic(p, expected_attribute);
    if p.at(T![=]) {
        parse_attribute_initializer(p).ok();
        Present(m.complete(p, HTML_ATTRIBUTE))
    } else {
        Present(m.complete(p, HTML_ATTRIBUTE))
    }
}

fn parse_literal(p: &mut HtmlParser, kind: HtmlSyntaxKind) -> ParsedSyntax {
    if !p.at(HTML_LITERAL) {
        return Absent;
    }
    let m = p.start();

    p.bump(HTML_LITERAL);

    Present(m.complete(p, kind))
}

fn parse_attribute_string_literal(p: &mut HtmlParser) -> ParsedSyntax {
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
    p.bump_with_context(T![=], HtmlLexContext::AttributeValue);
    
    // Try parsing Astro expression attributes first, then fall back to regular strings
    let value_parsed = if is_at_astro_expression(p) {
        if p.nth_at(1, T!['`']) {
            parse_astro_template_literal_attribute(p)
        } else {
            parse_astro_expression_attribute(p)
        }
    } else {
        parse_attribute_string_literal(p)
    };
    
    value_parsed.or_add_diagnostic(p, expected_initializer);
    Present(m.complete(p, HTML_ATTRIBUTE_INITIALIZER_CLAUSE))
}

fn parse_comment(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![<!--]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T![<!--], HtmlLexContext::Comment);
    while !p.at(T![-->]) && !p.at(EOF) {
        p.bump_with_context(HTML_LITERAL, HtmlLexContext::Comment);
    }
    p.expect(T![-->]);
    Present(m.complete(p, HTML_COMMENT))
}

fn parse_cdata_section(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["<![CDATA["]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T!["<![CDATA["], HtmlLexContext::CdataSection);
    while !p.at(T!["]]>"]) && !p.at(EOF) {
        p.bump_with_context(HTML_LITERAL, HtmlLexContext::CdataSection);
    }
    p.expect(T!["]]>"]);
    Present(m.complete(p, HTML_CDATA_SECTION))
}
