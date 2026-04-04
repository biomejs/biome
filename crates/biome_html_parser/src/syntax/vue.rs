use crate::parser::HtmlParser;
use crate::syntax::parse_attribute_initializer;
use crate::syntax::parse_error::expected_attribute;
use crate::syntax::parse_error::expected_vue_directive_argument;
use crate::token_source::HtmlLexContext;
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
        parse_attribute_initializer(p).ok();
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
        parse_attribute_initializer(p).ok();
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
        parse_attribute_initializer(p).ok();
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
        parse_attribute_initializer(p).ok();
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
