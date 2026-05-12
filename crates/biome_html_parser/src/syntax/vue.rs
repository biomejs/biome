use crate::parser::HtmlParser;
use crate::syntax::AttrInitializerContext;
use crate::syntax::parse_attribute_initializer;
use crate::syntax::parse_error::expected_attribute;
use crate::syntax::parse_error::expected_vue_directive_argument;
use crate::syntax::parse_error::expected_vue_v_for_binding;
use crate::syntax::parse_error::expected_vue_v_for_binding_separator;
use crate::syntax::parse_error::expected_vue_v_for_expression;
use crate::syntax::parse_error::expected_vue_v_for_operator;
use crate::syntax::parse_error::expected_vue_v_for_tuple_binding_end;
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind;
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::T;
use biome_parser::Parser;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

pub const VUE_KEYWORDS: TokenSet<HtmlSyntaxKind> = token_set!(T![of], T![in]);

pub(crate) fn parse_vue_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(HTML_LITERAL) {
        return Absent;
    }

    let m = p.start();

    let pos = p.source().position();
    // Check if this is v-for BEFORE bumping the token
    let is_v_for = p.cur_text() == "v-for";
    // FIXME: Ideally, the lexer would just lex IDENT directly
    p.bump_remap_with_context(
        IDENT,
        HtmlLexContext::InsideTagWithDirectives { svelte: false },
    );
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
    p.bump_with_context(
        T![@],
        HtmlLexContext::InsideTagWithDirectives { svelte: false },
    );
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
    p.bump_with_context(
        T![#],
        HtmlLexContext::InsideTagWithDirectives { svelte: false },
    );
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
    p.bump_with_context(
        T![:],
        HtmlLexContext::InsideTagWithDirectives { svelte: false },
    );
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

    p.expect_with_context(
        HTML_LITERAL,
        HtmlLexContext::InsideTagWithDirectives { svelte: false },
    );

    Present(m.complete(p, VUE_STATIC_ARGUMENT))
}

fn parse_vue_dynamic_argument(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T!['['], HtmlLexContext::VueDirectiveArgument);
    p.expect_with_context(
        HTML_LITERAL,
        HtmlLexContext::InsideTagWithDirectives { svelte: false },
    );
    p.expect_with_context(
        T![']'],
        HtmlLexContext::InsideTagWithDirectives { svelte: false },
    );

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

    p.bump_with_context(
        T![.],
        HtmlLexContext::InsideTagWithDirectives { svelte: false },
    );
    if p.at(T![:]) {
        // `:` is actually a valid modifier, for example `@keydown.:`
        p.bump_remap_with_context(
            HTML_LITERAL,
            HtmlLexContext::InsideTagWithDirectives { svelte: false },
        );
    } else {
        p.expect_with_context(
            HTML_LITERAL,
            HtmlLexContext::InsideTagWithDirectives { svelte: false },
        );
    }

    Present(m.complete(p, VUE_MODIFIER))
}

/// Recovery token set for v-for parsing - stop at operators, closing quote, or tag end.
const RECOVER_V_FOR: TokenSet<HtmlSyntaxKind> =
    token_set![T![in], T![of], T!['"'], T!["'"], T![>], T![/]];

/// Parses a Vue v-for value.
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
    p.bump_v_for(start_quote_kind);

    // binding: AnyVueVForBinding
    parse_vue_v_for_binding(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS, RECOVER_V_FOR),
            expected_vue_v_for_binding,
        )
        .ok();

    parse_vue_v_for_operator(p, start_quote_kind).or_add_diagnostic(p, expected_vue_v_for_operator);

    parse_vue_v_for_expression(p, start_quote_kind)
        .or_add_diagnostic(p, expected_vue_v_for_expression);

    // r_quote: ending quote must match starting quote
    p.expect_with_context(
        start_quote_kind,
        HtmlLexContext::InsideTagWithDirectives { svelte: false },
    );

    Present(m.complete(p, VUE_V_FOR_VALUE))
}

fn parse_vue_v_for_binding(p: &mut HtmlParser) -> ParsedSyntax {
    match p.cur() {
        T!['('] => parse_vue_v_for_tuple_binding(p),
        T!['{'] => parse_vue_v_for_object_binding(p),
        T!['['] => parse_vue_v_for_array_binding(p),
        HTML_LITERAL => parse_vue_v_for_identifier_binding(p),
        _ => Absent,
    }
}

fn parse_vue_v_for_binding_list_element(p: &mut HtmlParser) -> ParsedSyntax {
    if p.at(T![...]) {
        parse_vue_v_for_rest_binding(p)
    } else {
        match p.cur() {
            T!['{'] => parse_vue_v_for_object_binding(p),
            T!['['] => parse_vue_v_for_array_binding(p),
            HTML_LITERAL => parse_vue_v_for_object_property_or_identifier_binding(p),
            _ => Absent,
        }
    }
}

fn parse_vue_v_for_object_property_or_identifier_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(HTML_LITERAL) {
        return Absent;
    }

    let Some(property) = parse_vue_v_for_identifier_binding(p).ok() else {
        return Absent;
    };

    if !p.at(T![:]) {
        return Present(property);
    }

    let m = property.precede(p);
    p.bump_v_for(T![:]);
    parse_vue_v_for_binding(p).or_add_diagnostic(p, expected_vue_v_for_binding);
    Present(m.complete(p, VUE_V_FOR_OBJECT_PROPERTY_BINDING))
}

fn parse_vue_v_for_identifier_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(HTML_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(IDENT, HtmlLexContext::VueVForValue);
    Present(m.complete(p, VUE_V_FOR_IDENTIFIER_BINDING))
}

fn parse_vue_v_for_rest_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }

    let m = p.start();
    p.bump_v_for(T![...]);
    parse_vue_v_for_identifier_binding(p).or_add_diagnostic(p, expected_vue_v_for_binding);
    Present(m.complete(p, VUE_V_FOR_REST_BINDING))
}

fn parse_vue_v_for_object_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump_v_for(T!['{']);
    VueVForBindingList::new(T!['}']).parse_list(p);
    p.expect_v_for(T!['}']);
    Present(m.complete(p, VUE_V_FOR_OBJECT_BINDING))
}

fn parse_vue_v_for_array_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();
    p.bump_v_for(T!['[']);
    VueVForBindingList::new(T![']']).parse_list(p);
    p.expect_v_for(T![']']);
    Present(m.complete(p, VUE_V_FOR_ARRAY_BINDING))
}

/// Parses a tuple v-for binding (with parentheses).
///
/// Examples: `(item)`, `(item, index)`, `(value, key, index)`, `({ msg }, index)`
fn parse_vue_v_for_tuple_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();

    p.bump_v_for(T!['(']);

    parse_vue_v_for_binding(p).or_add_diagnostic(p, expected_vue_v_for_binding);

    // second: VueVForTupleElement? (optional ", identifier")
    if p.at(T![,]) {
        parse_vue_v_for_tuple_element(p).ok();

        // third: VueVForTupleElement? (optional ", identifier")
        if p.at(T![,]) {
            parse_vue_v_for_tuple_element(p).ok();
        }
    }

    if !p.eat(T![')']) {
        p.error(expected_vue_v_for_tuple_binding_end(p, p.cur_range()));
    }

    Present(m.complete(p, VUE_V_FOR_TUPLE_BINDING))
}

/// Parses a tuple element (comma followed by binding).
fn parse_vue_v_for_tuple_element(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![,]) {
        return Absent;
    }

    let m = p.start();

    p.bump_v_for(T![,]);

    parse_vue_v_for_binding(p).or_add_diagnostic(p, expected_vue_v_for_binding);

    Present(m.complete(p, VUE_V_FOR_TUPLE_ELEMENT))
}

fn parse_vue_v_for_operator(p: &mut HtmlParser, quote: HtmlSyntaxKind) -> ParsedSyntax {
    if p.at(T![in]) {
        let m = p.start();
        p.bump_with_context(T![in], HtmlLexContext::VueVForExpression(quote));
        Present(m.complete(p, VUE_V_FOR_IN_OPERATOR))
    } else if p.at(T![of]) {
        let m = p.start();
        p.bump_with_context(T![of], HtmlLexContext::VueVForExpression(quote));
        Present(m.complete(p, VUE_V_FOR_OF_OPERATOR))
    } else {
        Absent
    }
}

fn parse_vue_v_for_expression(p: &mut HtmlParser, quote: HtmlSyntaxKind) -> ParsedSyntax {
    if p.at_ts(token_set![T!["'"], T!['"'], T![>], EOF]) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(HTML_LITERAL, HtmlLexContext::VueVForExpression(quote));
    Present(m.complete(p, HTML_TEXT_EXPRESSION))
}

struct VueVForBindingList {
    end: HtmlSyntaxKind,
}

impl VueVForBindingList {
    fn new(end: HtmlSyntaxKind) -> Self {
        Self { end }
    }
}

const VUE_V_FOR_BINDING_LIST_RECOVERY_SET: TokenSet<HtmlSyntaxKind> = token_set![
    T![,],
    T!['}'],
    T![']'],
    T![in],
    T![of],
    T!["'"],
    T!['"'],
    T![>],
];

impl ParseSeparatedList for VueVForBindingList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = VUE_V_FOR_BINDING_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_vue_v_for_binding_list_element(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(self.end) || p.at_ts(RECOVER_V_FOR)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS, VUE_V_FOR_BINDING_LIST_RECOVERY_SET),
            expected_vue_v_for_binding,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        if (self.end == T![']'] && p.at(T!['}'])) || (self.end == T!['}'] && p.at(T![']'])) {
            let end = self.end.to_string().unwrap_or("closing delimiter");
            p.error(expected_vue_v_for_binding_separator(p, p.cur_range(), end));
            false
        } else {
            p.expect_v_for(self.separating_element_kind())
        }
    }
}
