use crate::parser::HtmlParser;
use crate::syntax::parse_error::{expected_child, expected_svelte_closing_block};
use crate::syntax::{TextExpression, parse_html_element};
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind::{
    EOF, HTML_BOGUS_ELEMENT, HTML_ELEMENT_LIST, SVELTE_BINDING_LIST, SVELTE_BOGUS_BLOCK,
    SVELTE_DEBUG_BLOCK, SVELTE_IDENT, SVELTE_KEY_BLOCK, SVELTE_KEY_CLOSING_BLOCK,
    SVELTE_KEY_OPENING_BLOCK, SVELTE_NAME,
};
use biome_html_syntax::{HtmlSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Marker, Parser, TokenSet, token_set};

pub(crate) fn parse_svelte_hash_block(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["{#"]) {
        return Absent;
    }
    // NOTE: use or_else chain here to parse
    // other possible hash blocks
    parse_key_block(p)
}

pub(crate) fn parse_key_block(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["{#"]) {
        return Absent;
    }

    let m = p.start();

    let completed = parse_opening_block(p, T![key], SVELTE_KEY_OPENING_BLOCK).ok();

    SvelteElementList.parse_list(p);

    parse_closing_block(p, T![key], SVELTE_KEY_CLOSING_BLOCK).or_add_diagnostic(p, |p, range| {
        let diagnostic = expected_svelte_closing_block(p, range);
        if let Some(completed) = completed {
            diagnostic.with_detail(completed.range(p), "This is where the block started.")
        } else {
            diagnostic
        }
    });

    Present(m.complete(p, SVELTE_KEY_BLOCK))
}

/// Parses a `{#<keyword> expression }` block.
///
/// `node` is the name of the node to emit
pub(crate) fn parse_opening_block(
    p: &mut HtmlParser,
    keyword: HtmlSyntaxKind,
    node: HtmlSyntaxKind,
) -> ParsedSyntax {
    if !p.at(T!["{#"]) {
        return Absent;
    }
    let m = p.start();
    let checkpoint = p.checkpoint();
    p.bump_with_context(T!["{#"], HtmlLexContext::Svelte);

    if !p.at(keyword) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    p.bump_with_context(keyword, HtmlLexContext::Svelte);
    TextExpression::new_single()
        .parse_element(p)
        .or_add_diagnostic(p, |p, range| {
            p.err_builder(
                "Expected an expression, instead none was found.",
                range.sub_start(m.start()),
            )
        });

    p.expect_with_context(T!['}'], HtmlLexContext::InsideTag);

    Present(m.complete(p, node))
}

/// Parses a `{/<keyword> }` block.
///
/// `node` is the name of the node to emit
pub(crate) fn parse_closing_block(
    p: &mut HtmlParser,
    keyword: HtmlSyntaxKind,
    node: HtmlSyntaxKind,
) -> ParsedSyntax {
    if !p.at(T!["{/"]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T!["{/"], HtmlLexContext::Svelte);

    p.expect_with_context(keyword, HtmlLexContext::Svelte);

    p.expect_with_context(T!['}'], HtmlLexContext::InsideTag);

    Present(m.complete(p, node))
}

pub(crate) fn parse_svelte_at_block(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["{@"]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T!["{@"], HtmlLexContext::Svelte);

    match p.cur() {
        T![debug] => parse_debug_block(p, m),
        _ => {
            m.abandon(p);
            Absent
        }
    }
}
pub(crate) fn parse_debug_block(p: &mut HtmlParser, marker: Marker) -> ParsedSyntax {
    if !p.at(T![debug]) {
        return Absent;
    }
    p.bump_with_context(T![debug], HtmlLexContext::Svelte);

    BindingList.parse_list(p);

    p.expect_with_context(T!['}'], HtmlLexContext::InsideTag);

    Present(marker.complete(p, SVELTE_DEBUG_BLOCK))
}

const BLOCK_RECOVER: TokenSet<HtmlSyntaxKind> = token_set!(
    T!['{'],
    T![<],
    T!["{@"],
    T!["{/"],
    T!["{:"],
    T!["{#"],
    T!['}']
);

#[derive(Debug)]
struct BindingList;

impl ParseSeparatedList for BindingList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = SVELTE_BINDING_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_name(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T!['}']) || p.at(T!['{'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SVELTE_BOGUS_BLOCK, BLOCK_RECOVER),
            expected_svelte_closing_block,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        p.expect_with_context(self.separating_element_kind(), HtmlLexContext::Svelte)
    }
}

fn parse_name(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(SVELTE_IDENT) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(SVELTE_IDENT, HtmlLexContext::Svelte);

    Present(m.complete(p, SVELTE_NAME))
}

#[derive(Default)]
struct SvelteElementList;

impl ParseNodeList for SvelteElementList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = HTML_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_html_element(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        let at_l_angle0 = p.at(T![<]);
        let at_slash1 = p.nth_at(1, T![/]);
        let at_eof = p.at(EOF);
        at_l_angle0 && at_slash1 || at_eof || p.at(T!["{/"])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS_ELEMENT, token_set![T![<], T![>], T!["{/"]]),
            expected_child,
        )
    }
}
