use crate::parser::HtmlParser;
use crate::syntax::AttrInitializerContext;
use crate::syntax::parse_attribute_initializer;
use crate::syntax::parse_error::expected_attribute;
use crate::syntax::parse_error::expected_vue_directive_argument;
use crate::syntax::parse_error::expected_vue_v_for_binding;
use crate::syntax::parse_error::invalid_vue_vfor;
use crate::token_source::{HtmlLexContext, RestrictedExpressionStopAt};
use biome_html_syntax::HtmlSyntaxKind;
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::T;
use biome_parser::Parser;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

pub(crate) fn parse_vue_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(HTML_LITERAL) {
        return Absent;
    }

    let m = p.start();

    let pos = p.source().position();
    // Check if this is v-for BEFORE bumping the token
    let is_v_for = p.cur_text() == "v-for";
    // FIXME: Ideally, the lexer would just lex IDENT directly
    p.bump_remap_with_context(IDENT, HtmlLexContext::InsideTagVue);
    if p.at(T![:]) {
        // is there any trivia after the directive name and before the colon?
        if let Some(last_trivia) = p.source().trivia_list.last()
            && pos < last_trivia.text_range().start()
        {
            // `v-else :foo="5"` is 2 directives, not `v-else:foo="5"`
            p.start().complete(p, VUE_MODIFIER_LIST);
            return Present(m.complete(p, VUE_DIRECTIVE));
        }
        parse_vue_directive_argument(p).ok();
    }
    VueModifierList.parse_list(p);
    if p.at(T![=]) {
        parse_attribute_initializer(
            p,
            if is_v_for {
                AttrInitializerContext::VueVFor
            } else {
                AttrInitializerContext::Regular
            },
        )
        .ok();
    }

    Present(m.complete(p, VUE_DIRECTIVE))
}

pub(crate) fn parse_vue_v_bind_shorthand_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }

    let m = p.start();

    parse_vue_directive_argument(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(VUE_BOGUS_DIRECTIVE_ARGUMENT, token_set![T![.], T![=]]),
            expected_vue_directive_argument,
        )
        .ok();
    VueModifierList.parse_list(p);
    if p.at(T![=]) {
        parse_attribute_initializer(p, AttrInitializerContext::Regular).ok();
    }

    Present(m.complete(p, VUE_V_BIND_SHORTHAND_DIRECTIVE))
}

pub(crate) fn parse_vue_v_on_shorthand_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![@]) {
        return Absent;
    }

    let m = p.start();

    let pos = p.source().position();
    p.bump_with_context(T![@], HtmlLexContext::InsideTagVue);
    // is there any trivia after the @ and before argument?
    if let Some(last_trivia) = p.source().trivia_list.last()
        && pos < last_trivia.text_range().start()
    {
        // `@ click="foo"` is not valid syntax
        // but we want to recover gracefully
        p.error(expected_vue_directive_argument(p, last_trivia.text_range()));
        return Present(m.complete(p, VUE_BOGUS_DIRECTIVE));
    }
    parse_vue_dynamic_argument(p)
        .or_else(|| parse_vue_static_argument(p))
        .ok();
    VueModifierList.parse_list(p);
    if p.at(T![=]) {
        parse_attribute_initializer(p, AttrInitializerContext::Regular).ok();
    }

    Present(m.complete(p, VUE_V_ON_SHORTHAND_DIRECTIVE))
}

pub(crate) fn parse_vue_v_slot_shorthand_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![#]) {
        return Absent;
    }

    let m = p.start();

    let pos = p.source().position();
    p.bump_with_context(T![#], HtmlLexContext::InsideTagVue);
    // is there any trivia after the hash and before argument?
    if let Some(last_trivia) = p.source().trivia_list.last()
        && pos < last_trivia.text_range().start()
    {
        // `# slot="5"` is not valid syntax
        // but we want to recover gracefully
        p.error(expected_vue_directive_argument(p, last_trivia.text_range()));
        return Present(m.complete(p, VUE_BOGUS_DIRECTIVE));
    }
    parse_vue_dynamic_argument(p)
        .or_else(|| parse_vue_static_argument(p))
        .ok();
    VueModifierList.parse_list(p);
    if p.at(T![=]) {
        parse_attribute_initializer(p, AttrInitializerContext::Regular).ok();
    }

    Present(m.complete(p, VUE_V_SLOT_SHORTHAND_DIRECTIVE))
}

fn parse_vue_directive_argument(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }

    let m = p.start();

    let pos = p.source().position();
    p.bump_with_context(T![:], HtmlLexContext::InsideTagVue);
    // is there any trivia after the colon and before argument?
    if let Some(last_trivia) = p.source().trivia_list.last()
        && pos < last_trivia.text_range().start()
    {
        // `: foo="5"` is not valid syntax
        // but we want to recover gracefully
        p.error(expected_vue_directive_argument(p, last_trivia.text_range()));
        return Present(m.complete(p, VUE_BOGUS_DIRECTIVE));
    }
    parse_vue_dynamic_argument(p)
        .or_else(|| parse_vue_static_argument(p))
        .ok();

    Present(m.complete(p, VUE_DIRECTIVE_ARGUMENT))
}

fn parse_vue_static_argument(p: &mut HtmlParser) -> ParsedSyntax {
    let m = p.start();

    p.expect_with_context(HTML_LITERAL, HtmlLexContext::InsideTagVue);

    Present(m.complete(p, VUE_STATIC_ARGUMENT))
}

fn parse_vue_dynamic_argument(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T!['['], HtmlLexContext::InsideTagVue);
    p.expect_with_context(HTML_LITERAL, HtmlLexContext::InsideTagVue);
    p.expect_with_context(T![']'], HtmlLexContext::InsideTagVue);

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
        !p.at(T![.])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                VUE_BOGUS_DIRECTIVE,
                token_set![T![.], T![>], T![/], T!['}']],
            ),
            expected_attribute,
        )
    }
}

fn parse_vue_modifier(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![.]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T![.], HtmlLexContext::InsideTagVue);
    p.expect_with_context(HTML_LITERAL, HtmlLexContext::InsideTagVue);

    Present(m.complete(p, VUE_MODIFIER))
}

/// Recovery token set for v-for parsing - stop at closing quote or tag end
const RECOVER_V_FOR: TokenSet<HtmlSyntaxKind> = token_set![T!['"'], T!["'"], T![>], T![/]];

/// Parses a Vue v-for value.
///
/// Grammar:
/// ```text
/// VueVForValue =
///     l_quote: ('"' | '\'')
///     binding: AnyVueVForBinding
///     'in' | 'of'
///     expr: HtmlTextExpression
///     r_quote: ('"' | '\'')
/// ```
///
/// Examples:
/// - `"item in items"`
/// - `"(item, index) in items"`
/// - `"(value, key, index) in obj"`
/// - `"{ message } in items"`
pub fn parse_vue_v_for_value(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at_ts(token_set![T!["'"], T!['"']]) {
        return Absent;
    }
    let start_quote_kind = if p.at(T!["'"]) { T!["'"] } else { T!['"'] };

    let m = p.start();

    // l_quote: consume either starting single or double quote
    p.bump_with_context(start_quote_kind, HtmlLexContext::VueVForValue);

    // binding: AnyVueVForBinding
    parse_vue_v_for_binding(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS, RECOVER_V_FOR),
            expected_vue_v_for_binding,
        )
        .ok();

    // 'in' | 'of' - one must be present
    // The factory has separate slots for 'in' (slot 2) and 'of' (slot 3)
    // and will correctly handle whichever one is present
    if p.at(T![in]) {
        p.bump_with_context(T![in], HtmlLexContext::VueVForValue);
    } else if p.at(T![of]) {
        p.bump_with_context(T![of], HtmlLexContext::VueVForValue);
    } else {
        p.error(invalid_vue_vfor(p, p.cur_range()));
    }

    // expr: HtmlTextExpression - parse until closing quote
    parse_vue_v_for_expression(p, start_quote_kind);

    // r_quote: ending quote must match starting quote
    // Use InsideTagVue for next token so '>' is recognized as R_ANGLE
    p.expect_with_context(start_quote_kind, HtmlLexContext::InsideTagVue);

    Present(m.complete(p, VUE_V_FOR_VALUE))
}

/// Parses the binding part of v-for.
///
/// Grammar:
/// ```text
/// AnyVueVForBinding =
///     VueVForSimpleBinding     // item, {msg}, [a,b]
///     | VueVForTupleBinding    // (item), (item, index), (value, key, index)
/// ```
fn parse_vue_v_for_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if p.at(T!['(']) {
        parse_vue_v_for_tuple_binding(p)
    } else {
        parse_vue_v_for_simple_binding(p)
    }
}

/// Parses a simple v-for binding (no parentheses).
///
/// Grammar:
/// ```text
/// VueVForSimpleBinding =
///     name: HtmlTextExpression
/// ```
///
/// Examples: `item`, `{ message }`, `[first, second]`
fn parse_vue_v_for_simple_binding(p: &mut HtmlParser) -> ParsedSyntax {
    let m = p.start();

    // Parse the binding name/pattern - stop at 'in' or 'of' keyword
    parse_vue_v_for_text_expression(p, RestrictedExpressionStopAt::InOrOf);

    Present(m.complete(p, VUE_V_FOR_SIMPLE_BINDING))
}

/// Parses a tuple v-for binding (with parentheses).
///
/// Grammar:
/// ```text
/// VueVForTupleBinding =
///     '('
///     value: HtmlTextExpression
///     second: VueVForTupleElement?
///     third: VueVForTupleElement?
///     ')'
/// ```
///
/// Examples: `(item)`, `(item, index)`, `(value, key, index)`, `({ msg }, index)`
fn parse_vue_v_for_tuple_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();

    // '('
    p.bump_with_context(T!['('], HtmlLexContext::VueVForValue);

    // value: HtmlTextExpression - stop at comma or close paren
    parse_vue_v_for_text_expression(p, RestrictedExpressionStopAt::CommaOrCloseParen);

    // second: VueVForTupleElement? (optional ", identifier")
    if p.at(T![,]) {
        parse_vue_v_for_tuple_element(p).ok();

        // third: VueVForTupleElement? (optional ", identifier")
        if p.at(T![,]) {
            parse_vue_v_for_tuple_element(p).ok();
        }
    }

    // ')'
    p.expect_with_context(T![')'], HtmlLexContext::VueVForValue);

    Present(m.complete(p, VUE_V_FOR_TUPLE_BINDING))
}

/// Parses a tuple element (comma followed by identifier).
///
/// Grammar:
/// ```text
/// VueVForTupleElement =
///     ','
///     name: 'ident'
/// ```
fn parse_vue_v_for_tuple_element(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![,]) {
        return Absent;
    }

    let m = p.start();

    // ','
    p.bump_with_context(T![,], HtmlLexContext::VueVForValue);

    // name: identifier - VueVForValue lexes identifiers as HTML_LITERAL
    // Remap to IDENT to satisfy the grammar
    p.bump_remap_with_context(IDENT, HtmlLexContext::VueVForValue);

    Present(m.complete(p, VUE_V_FOR_TUPLE_ELEMENT))
}

/// Parses a text expression inside v-for, stopping at the given delimiter.
/// This handles nested brackets (for destructuring patterns).
fn parse_vue_v_for_text_expression(p: &mut HtmlParser, _stop_at: RestrictedExpressionStopAt) {
    let m = p.start();

    // The binding was already lexed by VueVForValue context as HTML_LITERAL.
    // Just consume it and continue with VueVForValue for proper whitespace handling.
    p.bump_with_context(HTML_LITERAL, HtmlLexContext::VueVForValue);

    m.complete(p, HTML_TEXT_EXPRESSION);
}

/// Parses the expression part of v-for (the iterable).
/// Consumes everything until the closing quote.
fn parse_vue_v_for_expression(p: &mut HtmlParser, closing_quote: HtmlSyntaxKind) {
    let m = p.start();

    // Consume all tokens until closing quote
    while !p.at(closing_quote) && !p.at(EOF) && !p.at(T![>]) {
        p.bump_with_context(p.cur(), HtmlLexContext::VueVForValue);
    }

    m.complete(p, HTML_TEXT_EXPRESSION);
}
