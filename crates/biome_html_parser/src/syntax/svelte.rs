use crate::parser::HtmlParser;
use crate::syntax::parse_error::{
    expected_child_or_block, expected_svelte_closing_block, expected_text_expression,
};
use crate::syntax::{parse_html_element, parse_single_text_expression_content};
use crate::token_source::{HtmlLexContext, HtmlReLexContext};
use biome_html_syntax::HtmlSyntaxKind::{
    EOF, HTML_BOGUS_ELEMENT, HTML_ELEMENT_LIST, SVELTE_ATTACH_ATTRIBUTE, SVELTE_BINDING_LIST,
    SVELTE_BOGUS_BLOCK, SVELTE_CONST_BLOCK, SVELTE_DEBUG_BLOCK, SVELTE_ELSE_CLAUSE,
    SVELTE_ELSE_IF_CLAUSE, SVELTE_ELSE_IF_CLAUSE_LIST, SVELTE_HTML_BLOCK, SVELTE_IDENT,
    SVELTE_IF_BLOCK, SVELTE_IF_CLOSING_BLOCK, SVELTE_IF_OPENING_BLOCK, SVELTE_KEY_BLOCK,
    SVELTE_KEY_CLOSING_BLOCK, SVELTE_KEY_OPENING_BLOCK, SVELTE_NAME, SVELTE_RENDER_BLOCK,
};
use biome_html_syntax::{HtmlSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Marker, Parser, TokenSet, token_set};
use std::ops::Sub;

pub(crate) fn parse_svelte_hash_block(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["{#"]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T!["{#"], HtmlLexContext::Svelte);
    match p.cur() {
        T![key] => parse_key_block(p, m),
        T![if] => parse_if_block(p, m),
        _ => {
            m.abandon(p);
            Absent
        }
    }
    // NOTE: use or_else chain here to parse
    // other possible hash blocks
}

pub(crate) fn parse_key_block(p: &mut HtmlParser, parent_marker: Marker) -> ParsedSyntax {
    let result = parse_opening_block(p, T![key], SVELTE_KEY_OPENING_BLOCK, parent_marker);
    if result.is_absent() {
        return Absent;
    }
    let m = result.precede(p);

    SvelteElementList::new().parse_list(p);

    parse_closing_block(p, T![key], SVELTE_KEY_CLOSING_BLOCK).or_add_diagnostic(p, |p, range| {
        expected_svelte_closing_block(p, range)
            .with_detail(range.sub(m.start()), "This is where the block started.")
    });

    Present(m.complete(p, SVELTE_KEY_BLOCK))
}

pub(crate) fn parse_if_block(p: &mut HtmlParser, parent_marker: Marker) -> ParsedSyntax {
    if !p.at(T![if]) {
        parent_marker.abandon(p);
        return Absent;
    }

    let result = parse_if_opening_block(p, parent_marker);
    let m = result.precede(p);

    SvelteElseIfClauseLit.parse_list(p);

    parse_else_clause(p).ok();

    parse_closing_block(p, T![if], SVELTE_IF_CLOSING_BLOCK).or_add_diagnostic(p, |p, range| {
        expected_svelte_closing_block(p, range)
            .with_detail(range.sub(m.start()), "This is where the block started.")
    });

    Present(m.complete(p, SVELTE_IF_BLOCK))
}

fn parse_if_opening_block(p: &mut HtmlParser, parent_marker: Marker) -> ParsedSyntax {
    if !p.at(T![if]) {
        parent_marker.abandon(p);
        return Absent;
    }

    p.bump_with_context(T![if], HtmlLexContext::single_expression());

    parse_single_text_expression_content(p).or_add_diagnostic(p, |p, range| {
        p.err_builder(
            "Expected an expression, instead none was found.",
            range.sub_start(parent_marker.start()),
        )
    });

    p.expect(T!['}']);

    SvelteElementList::new()
        .with_stop_at_curly_colon()
        .parse_list(p);

    Present(parent_marker.complete(p, SVELTE_IF_OPENING_BLOCK))
}

/// Parses `{:else if expression} ...`
pub(crate) fn parse_else_if_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["{:"]) {
        return Absent;
    }
    let m = p.start();
    let checkpoint = p.checkpoint();

    p.bump_with_context(T!["{:"], HtmlLexContext::Svelte);

    p.expect_with_context(T![else], HtmlLexContext::Svelte);

    if p.at(T!['}']) {
        // It's an `{:else}` block, we rewind the parsing and exit early
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    p.expect_with_context(T![if], HtmlLexContext::single_expression());

    parse_single_text_expression_content(p).or_add_diagnostic(p, |p, range| {
        p.err_builder(
            "Expected an expression, instead none was found.",
            range.sub_start(m.start()),
        )
    });

    p.expect(T!['}']);

    SvelteElementList::new()
        .with_stop_at_curly_colon()
        .parse_list(p);

    Present(m.complete(p, SVELTE_ELSE_IF_CLAUSE))
}

/// Parses `{:else} ...`
pub(crate) fn parse_else_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["{:"]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T!["{:"], HtmlLexContext::Svelte);
    p.expect(T![else]);
    p.expect(T!['}']);
    SvelteElementList::new().parse_list(p);
    Present(m.complete(p, SVELTE_ELSE_CLAUSE))
}

/// Parses a `{#<keyword> expression }` block.
///
/// `node` is the name of the node to emit
pub(crate) fn parse_opening_block(
    p: &mut HtmlParser,
    keyword: HtmlSyntaxKind,
    node: HtmlSyntaxKind,
    m: Marker,
) -> ParsedSyntax {
    if !p.at(keyword) {
        m.abandon(p);
        return Absent;
    }

    p.bump_with_context(keyword, HtmlLexContext::single_expression());
    parse_single_text_expression_content(p).or_add_diagnostic(p, |p, range| {
        p.err_builder(
            "Expected an expression, instead none was found.",
            range.sub_start(m.start()),
        )
    });

    p.expect(T!['}']);

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

    p.expect(T!['}']);

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
        T![html] => parse_html_block(p, m),
        T![render] => parse_render_block(p, m),
        T![const] => parse_const_block(p, m),
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

    p.expect(T!['}']);

    Present(marker.complete(p, SVELTE_DEBUG_BLOCK))
}

pub(crate) fn parse_html_block(p: &mut HtmlParser, marker: Marker) -> ParsedSyntax {
    if !p.at(T![html]) {
        return Absent;
    }
    p.bump_with_context(T![html], HtmlLexContext::single_expression());

    parse_single_text_expression_content(p).or_add_diagnostic(p, expected_text_expression);

    p.expect(T!['}']);

    Present(marker.complete(p, SVELTE_HTML_BLOCK))
}

pub(crate) fn parse_render_block(p: &mut HtmlParser, marker: Marker) -> ParsedSyntax {
    if !p.at(T![render]) {
        return Absent;
    }
    p.bump_with_context(T![render], HtmlLexContext::single_expression());

    parse_single_text_expression_content(p).or_add_diagnostic(p, expected_text_expression);

    p.expect(T!['}']);

    Present(marker.complete(p, SVELTE_RENDER_BLOCK))
}

pub(crate) fn parse_attach_attribute(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["{@"]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T!["{@"], HtmlLexContext::Svelte);
    p.expect_with_context(T![attach], HtmlLexContext::single_expression());

    parse_single_text_expression_content(p).or_add_diagnostic(p, expected_text_expression);

    p.expect_with_context(T!['}'], HtmlLexContext::InsideTag);

    Present(m.complete(p, SVELTE_ATTACH_ATTRIBUTE))
}

pub(crate) fn parse_const_block(p: &mut HtmlParser, marker: Marker) -> ParsedSyntax {
    if !p.at(T![const]) {
        return Absent;
    }
    p.bump_with_context(T![const], HtmlLexContext::single_expression());

    parse_single_text_expression_content(p).or_add_diagnostic(p, expected_text_expression);

    p.expect(T!['}']);

    Present(marker.complete(p, SVELTE_CONST_BLOCK))
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
    let m = p.start();
    p.bump_remap_with_context(SVELTE_IDENT, HtmlLexContext::Svelte);

    Present(m.complete(p, SVELTE_NAME))
}

#[derive(Default)]
struct SvelteElementList {
    /// If `true`, the list parsing stops at `{:` too when calling [ParseNodeList::is_at_list_end]
    stop_at_curly_colon: bool,
}

impl SvelteElementList {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_stop_at_curly_colon(mut self) -> Self {
        self.stop_at_curly_colon = true;
        self
    }
}

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
        at_l_angle0 && at_slash1
            || at_eof
            || p.at(T!["{/"])
            || (self.stop_at_curly_colon && p.at(T!["{:"]))
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS_ELEMENT, token_set![T![<], T![>], T!["{/"]]),
            expected_child_or_block,
        )
    }
}

#[derive(Debug)]
struct SvelteElseIfClauseLit;

impl ParseNodeList for SvelteElseIfClauseLit {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = SVELTE_ELSE_IF_CLAUSE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_else_if_clause(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        let closing = p.at(T!["{/"]);
        if closing {
            return true;
        }
        // Here we need to get creative. At the moment svelte keywords are correctly lexed
        // only when we use the `Svelte` context. To retrieve them, we use the relex
        // feature to bump two tokens, and relex them with the proper context.
        // Once we retrieved the relexed tokens, we rewind the parser.
        let curly_colon = p.at(T!["{:"]);
        let checkpoint = p.checkpoint();
        p.bump_any();
        p.re_lex(HtmlReLexContext::Svelte);
        let at_else = p.at(T![else]);
        p.bump_any();
        p.re_lex(HtmlReLexContext::Svelte);
        let at_if = p.at(T![if]);

        let condition = curly_colon && at_else && !at_if;
        p.rewind(checkpoint);
        condition
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                SVELTE_BOGUS_BLOCK,
                token_set![T![<], T![>], T!["{/"], T!["{:"]],
            ),
            expected_child_or_block,
        )
    }
}
