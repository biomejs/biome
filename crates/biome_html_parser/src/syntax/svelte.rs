use crate::parser::HtmlParser;
use crate::syntax::parse_error::{
    expected_child_or_block, expected_svelte_closing_block, expected_text_expression,
};
use crate::syntax::{parse_html_element, parse_single_text_expression_content};
use crate::token_source::{HtmlLexContext, HtmlReLexContext, RestrictedExpressionStopAt};
use biome_html_syntax::HtmlSyntaxKind::{
    EOF, HTML_BOGUS_ELEMENT, HTML_ELEMENT_LIST, IDENT, SVELTE_ATTACH_ATTRIBUTE,
    SVELTE_BINDING_LIST, SVELTE_BOGUS_BLOCK, SVELTE_CONST_BLOCK, SVELTE_DEBUG_BLOCK,
    SVELTE_EACH_AS_KEYED_ITEM, SVELTE_EACH_BLOCK, SVELTE_EACH_CLOSING_BLOCK, SVELTE_EACH_INDEX,
    SVELTE_EACH_KEY, SVELTE_EACH_KEYED_ITEM, SVELTE_EACH_OPENING_BLOCK, SVELTE_ELSE_CLAUSE,
    SVELTE_ELSE_IF_CLAUSE, SVELTE_ELSE_IF_CLAUSE_LIST, SVELTE_HTML_BLOCK, SVELTE_IF_BLOCK,
    SVELTE_IF_CLOSING_BLOCK, SVELTE_IF_OPENING_BLOCK, SVELTE_KEY_BLOCK, SVELTE_KEY_CLOSING_BLOCK,
    SVELTE_KEY_OPENING_BLOCK, SVELTE_NAME, SVELTE_RENDER_BLOCK,
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
        T![each] => parse_each_block(p, m),
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

fn parse_each_block(p: &mut HtmlParser, parent_marker: Marker) -> ParsedSyntax {
    if !p.at(T![each]) {
        parent_marker.abandon(p);
        return Absent;
    }

    let (result, has_errors) = parse_each_opening_block(p, parent_marker);

    let m = result.precede(p);

    SvelteElementList::new()
        .with_stop_at_curly_colon()
        .parse_list(p);

    // Parse optional {:else} clause
    if at_else_opening_block(p) {
        parse_else_clause(p).ok();
    }

    parse_closing_block(p, T![each], SVELTE_EACH_CLOSING_BLOCK).or_add_diagnostic(p, |p, range| {
        expected_svelte_closing_block(p, range)
            .with_detail(range.sub(m.start()), "This is where the block started.")
    });
    if has_errors {
        Present(m.complete(p, SVELTE_BOGUS_BLOCK))
    } else {
        Present(m.complete(p, SVELTE_EACH_BLOCK))
    }
}

/// Parses the "as item, index (key)" part of an each block
/// This includes the 'as' keyword, the item binding name, optional index, and optional key
fn parse_each_as_keyed_item(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![as]) {
        return Absent;
    }

    let m = p.start();

    // Consume 'as' and switch context for name (stop at comma for optional index)
    p.bump_with_context(
        T![as],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::OpeningParenOrComma),
    );

    // Parse name (required)
    parse_single_text_expression_content(p).or_add_diagnostic(p, |p, range| {
        p.err_builder("Expected a binding pattern after 'as'", range)
    });

    // Re-lex to Svelte context to recognize ',' and other tokens
    p.re_lex(HtmlReLexContext::Svelte);

    // Parse optional index: , index
    if p.at(T![,]) {
        parse_each_index(p).ok();
        // Re-lex again to recognize a key expression opening paren
        p.re_lex(HtmlReLexContext::Svelte);
    }

    // Parse optional key: (key_expression)
    // The key expression includes parentheses as part of the literal
    parse_each_key(p).ok();

    Present(m.complete(p, SVELTE_EACH_AS_KEYED_ITEM))
}

/// Parse the `( key )` inside the `#each` block
fn parse_each_key(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(
        T!['('],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::ClosingParen),
    );

    parse_single_text_expression_content(p).or_add_diagnostic(p, |p, range| {
        p.err_builder("Expected a key expression in parentheses", range)
    });

    // Re-lex to Svelte context to recognize ',) and other tokens
    p.re_lex(HtmlReLexContext::Svelte);

    p.expect(T![')']);

    Present(m.complete(p, SVELTE_EACH_KEY))
}

/// Parses the ", index" part for index-only syntax (without 'as')
/// Example: {#each items, i}
fn parse_each_keyed_item(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![,]) {
        return Absent;
    }

    let m = p.start();

    // Parse the index
    parse_each_index(p).ok();

    Present(m.complete(p, SVELTE_EACH_KEYED_ITEM))
}

/// Parses the ", index" part for index-only syntax (without 'as')
/// Example: {#each items, i}
fn parse_each_index(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![,]) {
        return Absent;
    }
    // Parse the index
    let m = p.start();
    p.bump_with_context(
        T![,],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::OpeningParenOrComma),
    );
    parse_single_text_expression_content(p).or_add_diagnostic(p, |p, range| {
        p.err_builder("Expected an index binding after ','", range)
    });
    Present(m.complete(p, SVELTE_EACH_INDEX))
}

/// Determines which variant of AnySvelteBlockItem to parse based on lookahead
fn parse_svelte_block_item(p: &mut HtmlParser) -> ParsedSyntax {
    if p.at(T![as]) {
        parse_each_as_keyed_item(p)
    } else if p.at(T![,]) {
        parse_each_keyed_item(p)
    } else {
        // Error: missing 'as' or ','
        p.error(p.err_builder(
            "Expected 'as' keyword for item binding or ',' for index-only syntax",
            p.cur_range(),
        ));
        Absent
    }
}

fn parse_each_opening_block(p: &mut HtmlParser, parent_marker: Marker) -> (ParsedSyntax, bool) {
    if !p.at(T![each]) {
        parent_marker.abandon(p);
        return (Absent, false);
    }

    p.bump_with_context(
        T![each],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::AsOrComma),
    );
    // Flags used to track possible errors so that the final block can be emitted as a bogus node
    let mut has_errors = false;

    // Parse the collection expression (stops at 'as' or ',')
    let result = parse_single_text_expression_content(p).or_add_diagnostic(p, |p, range| {
        p.err_builder(
            "Expected an expression after 'each'",
            range.sub_start(parent_marker.start()),
        )
    });

    // In case there's nothing parsed, it's possible we have whitespaces or noice.
    // We consume any possible token, so we can recover and resume normal parsing.
    if result.is_none() {
        has_errors |= true;
        p.bump_any();
    }

    // After parsing the expression, switch back to the Svelte context so we can properly
    // tokenize 'as', ',', and other tokens
    p.re_lex(HtmlReLexContext::Svelte);

    // Parse the optional item binding (either 'as item...' or ', index')
    parse_svelte_block_item(p).ok();

    p.expect(T!['}']);

    (
        Present(parent_marker.complete(p, SVELTE_EACH_OPENING_BLOCK)),
        has_errors,
    )
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
    p.bump_remap_with_context(IDENT, HtmlLexContext::Svelte);

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

        p.at(T!["{:"]) && p.nth_at(1, T![else]) && !p.nth_at(2, T![if])
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

pub(crate) fn is_at_svelte_keyword(p: &HtmlParser) -> bool {
    matches!(
        p.cur(),
        T![if]
            | T![else]
            | T![each]
            | T![debug]
            | T![const]
            | T![attach]
            | T![render]
            | T![key]
            | T![as]
    )
}

fn at_else_opening_block(p: &mut HtmlParser) -> bool {
    p.at(T!["{:"]) && p.nth_at(1, T![else])
}
