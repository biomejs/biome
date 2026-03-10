use crate::parser::HtmlParser;
use crate::syntax::HtmlSyntaxFeatures::{SingleTextExpressions, Svelte};
use crate::syntax::parse_error::{
    expected_child_or_block, expected_expression, expected_name, expected_svelte_closing_block,
    expected_svelte_property, expected_text_expression, expected_valid_directive,
};
use crate::syntax::{
    TextExpression, parse_attribute_initializer, parse_html_element, parse_single_text_expression,
    parse_single_text_expression_content,
};
use crate::token_source::{HtmlLexContext, HtmlReLexContext, RestrictedExpressionStopAt};
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::{HtmlSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Marker, Parser, SyntaxFeature, TokenSet, token_set};
use biome_rowan::TextRange;
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
        T![await] => parse_await_block(p, m),
        T![snippet] => parse_snippet_block(p, m),
        _ => {
            m.abandon(p);
            Absent
        }
    }
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
        expected_expression(p, range.sub_start(parent_marker.start()))
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
        expected_expression(p, range.sub_start(m.start()))
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

// #region parse `{#each}` functions

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
    if is_at_else_opening_block(p) {
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
    p.bump_with_context(T![as], HtmlLexContext::Svelte);

    // This is the case where a name is actually an expression e.g. `[id, name]`
    // We need to re-lex so we recognize it as expression instead of name
    if p.at(T!['{']) {
        p.re_lex(HtmlReLexContext::Svelte);
        parse_curly_destructured_name(p)
    } else if p.at(T!['[']) {
        p.re_lex(HtmlReLexContext::Svelte);
        parse_square_destructured_name(p)
    } else {
        // Parse name (required)
        parse_svelte_name(p)
    }
    .or_add_diagnostic(p, |p, range| {
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
    p.bump_with_context(T![,], HtmlLexContext::Svelte);
    parse_svelte_name(p).or_add_diagnostic(p, |p, range| {
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

    if p.at(T!['}']) {
        has_errors |= true;
        p.error(p.err_builder(
            "Expected 'as' keyword for item binding or ',' for index-only syntax",
            p.cur_range(),
        ));
    }

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
    if p.at(T![as]) || p.at(T![,]) {
        parse_svelte_block_item(p).ok();
    }

    p.expect(T!['}']);

    (
        Present(parent_marker.complete(p, SVELTE_EACH_OPENING_BLOCK)),
        has_errors,
    )
}
// #endregion

/// Parses a spread attribute or a single text expression.
pub(crate) fn parse_svelte_spread_or_expression(p: &mut HtmlParser) -> ParsedSyntax {
    if !SingleTextExpressions.is_supported(p) {
        return Absent;
    }

    if !p.at(T!['{']) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();

    // We bump using svelte context because it's faster to lex a possible ..., which is also
    // only consumable when using the Svelte context
    p.bump_with_context(T!['{'], HtmlLexContext::Svelte);

    if p.at(T![...]) {
        p.bump_with_context(T![...], HtmlLexContext::single_expression());

        TextExpression::new_single()
            .parse_element(p)
            .or_add_diagnostic(p, expected_expression);

        p.expect_with_context(T!['}'], HtmlLexContext::InsideTagSvelte);
        Present(m.complete(p, HTML_SPREAD_ATTRIBUTE))
    } else {
        p.rewind(checkpoint);
        m.abandon(p);
        parse_single_text_expression(p, HtmlLexContext::InsideTagSvelte)
    }
}

// #region await parse functions

fn parse_await_block(p: &mut HtmlParser, parent_marker: Marker) -> ParsedSyntax {
    if !p.at(T![await]) {
        parent_marker.abandon(p);
        return Absent;
    }
    let ParseAwaitResult {
        result,
        has_then_clause,
        has_catch_clause,
    } = parse_await_opening_block(p, parent_marker);
    let m = result.precede(p);

    AwaitClausesList {
        has_then_clause,
        has_catch_clause,
        seen_then_block: None,
        seen_catch_block: None,
    }
    .parse_list(p);

    parse_closing_block(p, T![await], SVELTE_AWAIT_CLOSING_BLOCK).or_add_diagnostic(
        p,
        |p, range| {
            expected_svelte_closing_block(p, range)
                .with_detail(range.sub(m.start()), "This is where the block started.")
        },
    );

    Present(m.complete(p, SVELTE_AWAIT_BLOCK))
}

struct ParseAwaitResult {
    result: ParsedSyntax,
    /// Used to signal possible parse errors in case there's a then block
    has_then_clause: Option<TextRange>,
    /// Used to signal possible parse errors in case there's a catch block
    has_catch_clause: Option<TextRange>,
}

/// Parses a `{#await expression}` block.
fn parse_await_opening_block(p: &mut HtmlParser, parent_marker: Marker) -> ParseAwaitResult {
    if !p.at(T![await]) {
        parent_marker.abandon(p);
        return ParseAwaitResult {
            result: Absent,
            has_then_clause: None,
            has_catch_clause: None,
        };
    }
    let mut has_then_clause = None;
    let mut has_catch_clause = None;

    p.bump_with_context(
        T![await],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::ThenOrCatch),
    );

    parse_single_text_expression_content(p).or_add_diagnostic(p, |p, range| {
        expected_expression(p, range.sub_start(parent_marker.start()))
    });

    if p.cur_text().is_empty() {
        p.bump_remap(HTML_LITERAL);
        p.error(p.err_builder("Expected an expression after 'await'", p.cur_range()));
    }

    if p.at(T![then])
        && let Present(m) = parse_await_then_clause(p)
    {
        has_then_clause = Some(m.range(p));
    }
    if p.at(T![catch])
        && let Present(m) = parse_await_catch_clause(p)
    {
        has_catch_clause = Some(m.range(p));
    }

    p.expect(T!['}']);

    SvelteElementList::new()
        .with_stop_at_curly_colon()
        .parse_list(p);

    ParseAwaitResult {
        result: Present(parent_marker.complete(p, SVELTE_AWAIT_OPENING_BLOCK)),
        has_catch_clause,
        has_then_clause,
    }
}

fn parse_await_then_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![then]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T![then], HtmlLexContext::single_expression());

    parse_single_text_expression_content(p)
        .or_add_diagnostic(p, |p, range| expected_expression(p, range));

    if p.cur_text().is_empty() {
        p.bump_remap(HTML_LITERAL);
    }

    Present(m.complete(p, SVELTE_AWAIT_THEN_CLAUSE))
}

fn parse_await_catch_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![catch]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T![catch], HtmlLexContext::single_expression());

    parse_single_text_expression_content(p)
        .or_add_diagnostic(p, |p, range| expected_expression(p, range));
    if p.cur_text().is_empty() {
        p.bump_remap(HTML_LITERAL);
    }
    Present(m.complete(p, SVELTE_AWAIT_CATCH_CLAUSE))
}

struct AwaitClausesList {
    has_then_clause: Option<TextRange>,
    has_catch_clause: Option<TextRange>,
    seen_catch_block: Option<TextRange>,
    seen_then_block: Option<TextRange>,
}

impl ParseNodeList for AwaitClausesList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = SVELTE_AWAIT_CLAUSES_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        let (result, block_parsed) =
            parse_await_then_or_catch_block(p, self.has_then_clause, self.has_catch_clause);

        result
            .and_then(|parsed| {
                let range = parsed.range(p);

                if let Some(seen_catch_block) = self.seen_catch_block
                    && block_parsed == BlockParsed::Catch
                {
                    p.error(
                        p.err_builder(
                            "{:catch} cannot appear more than once within a block.",
                            p.cur_range(),
                        )
                        .with_detail(seen_catch_block, "This is where the block started."),
                    )
                } else if let Some(seen_then_block) = self.seen_then_block
                    && block_parsed == BlockParsed::Then
                {
                    p.error(
                        p.err_builder(
                            "{:then} cannot appear more than once within a block.",
                            p.cur_range(),
                        )
                        .with_detail(seen_then_block, "This is where the block started."),
                    )
                }

                if block_parsed == BlockParsed::Catch {
                    self.seen_catch_block = Some(range);
                } else if block_parsed == BlockParsed::Then {
                    self.seen_then_block = Some(range);
                }

                if let Some(catch_range) = self.seen_catch_block
                    && let Some(then_range) = self.seen_then_block
                    && catch_range < then_range
                {
                    p.error(
                        p.err_builder(
                            "{:catch} cannot appear before the {:then} block.",
                            catch_range,
                        )
                        .with_detail(then_range, "This is where the {:then} block starts."),
                    )
                }

                Present(parsed)
            })
            .or_else(|| Absent)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        !is_at_then_or_catch_block(p)
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
}

#[derive(Default, Eq, PartialEq)]
enum BlockParsed {
    #[default]
    None,
    Catch,
    Then,
}

fn parse_await_then_or_catch_block(
    p: &mut HtmlParser,
    seen_then_clause: Option<TextRange>,
    seen_catch_clause: Option<TextRange>,
) -> (ParsedSyntax, BlockParsed) {
    if !is_at_then_or_catch_block(p) {
        return (Absent, BlockParsed::None);
    }
    let m = p.start();
    p.bump(T!["{:"]);

    if p.at(T![then]) {
        (
            parse_await_then_block(p, m, seen_then_clause),
            BlockParsed::Then,
        )
    } else if p.at(T![catch]) {
        (
            parse_await_catch_block(p, m, seen_catch_clause),
            BlockParsed::Catch,
        )
    } else {
        m.abandon(p);
        (Absent, BlockParsed::None)
    }
}

fn parse_await_then_block(
    p: &mut HtmlParser,
    m: Marker,
    has_then_clause: Option<TextRange>,
) -> ParsedSyntax {
    if !p.at(T![then]) {
        m.abandon(p);
        return Absent;
    }
    p.bump_with_context(T![then], HtmlLexContext::single_expression());

    parse_single_text_expression_content(p)
        .or_add_diagnostic(p, |p, range| expected_expression(p, range));

    if p.cur_text().is_empty() {
        p.bump_remap(HTML_LITERAL);
        p.error(p.err_builder("Expected an expression after 'then'", p.cur_range()));
    }

    p.expect(T!['}']);

    SvelteElementList::new()
        .with_stop_at_curly_colon()
        .parse_list(p);

    if let Some(range) = has_then_clause {
        p.error(
            p.err_builder(
                "{:then} cannot appear more than once within a block.",
                p.cur_range(),
            )
            .with_detail(range, "This is where the block started."),
        )
    }

    Present(m.complete(p, SVELTE_AWAIT_THEN_BLOCK))
}

fn parse_await_catch_block(
    p: &mut HtmlParser,
    m: Marker,
    has_catch_clause: Option<TextRange>,
) -> ParsedSyntax {
    if !p.at(T![catch]) {
        m.abandon(p);
        return Absent;
    }
    p.bump_with_context(T![catch], HtmlLexContext::single_expression());

    parse_single_text_expression_content(p)
        .or_add_diagnostic(p, |p, range| expected_expression(p, range));

    if p.cur_text().is_empty() {
        p.bump_remap(HTML_LITERAL);
        p.error(p.err_builder("Expected an expression after 'catch'", p.cur_range()));
    }

    p.expect(T!['}']);

    SvelteElementList::new()
        .with_stop_at_curly_colon()
        .parse_list(p);

    if let Some(range) = has_catch_clause {
        p.error(
            p.err_builder(
                "{:catch} cannot appear more than once within a block.",
                p.cur_range(),
            )
            .with_detail(range, "This is where the block started."),
        );
    }

    Present(m.complete(p, SVELTE_AWAIT_CATCH_BLOCK))
}

// #endregion

// #region snippet parsing functions
fn parse_snippet_block(p: &mut HtmlParser, parent_marker: Marker) -> ParsedSyntax {
    if !p.at(T![snippet]) {
        parent_marker.abandon(p);
        return Absent;
    }
    let result = parse_snippet_opening_block(p, parent_marker);
    let m = result.precede(p);

    parse_closing_block(p, T![snippet], SVELTE_SNIPPET_CLOSING_BLOCK).or_add_diagnostic(
        p,
        |p, range| {
            expected_svelte_closing_block(p, range)
                .with_detail(range.sub(m.start()), "This is where the block started.")
        },
    );

    Present(m.complete(p, SVELTE_SNIPPET_BLOCK))
}

fn parse_snippet_opening_block(p: &mut HtmlParser, parent_marker: Marker) -> ParsedSyntax {
    if !p.at(T![snippet]) {
        parent_marker.abandon(p);
        return Absent;
    }
    p.bump_with_context(T![snippet], HtmlLexContext::single_expression());

    parse_single_text_expression_content(p).or_add_diagnostic(p, |p, range| {
        expected_expression(p, range.sub_start(parent_marker.start()))
    });

    if p.cur_text().is_empty() {
        p.bump_remap(HTML_LITERAL);
        p.error(p.err_builder("Expected an expression after 'snippet'", p.cur_range()));
    }

    p.expect(T!['}']);

    SvelteElementList::new()
        .with_stop_at_curly_colon()
        .parse_list(p);

    Present(parent_marker.complete(p, SVELTE_SNIPPET_OPENING_BLOCK))
}

fn parse_curly_destructured_name(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();

    p.bump_with_context(T!['{'], HtmlLexContext::Svelte);

    SvelteBindingAssignmentBindingList.parse_list(p);

    p.expect(T!['}']);

    Present(m.complete(p, SVELTE_CURLY_DESTRUCTURED_NAME))
}

fn parse_square_destructured_name(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T!['['], HtmlLexContext::Svelte);

    SvelteBindingAssignmentBindingList.parse_list(p);

    p.expect(T![']']);

    Present(m.complete(p, SVELTE_SQUARE_DESTRUCTURED_NAME))
}

// #endregion

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
        expected_expression(p, range.sub_start(m.start()))
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

    p.expect_with_context(T!['}'], HtmlLexContext::InsideTagSvelte);

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
        parse_svelte_name(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}']) || p.at(T!['{']) || p.at(T![']'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SVELTE_BOGUS_BLOCK, BLOCK_RECOVER),
            expected_name,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        p.expect_with_context(self.separating_element_kind(), HtmlLexContext::Svelte)
    }
}

/// Parses a Svelte name
fn parse_svelte_name(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(IDENT) && !is_at_svelte_keyword(p) {
        return Absent;
    }
    let m = p.start();
    p.bump_remap_with_context(IDENT, HtmlLexContext::Svelte);

    Present(m.complete(p, SVELTE_NAME))
}

fn parse_binding_literal(p: &mut HtmlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump_with_context(HTML_LITERAL, HtmlLexContext::InsideTagSvelte);
    Present(m.complete(p, SVELTE_LITERAL))
}

/// Parses `...rest`
fn parse_rest_name(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T![...], HtmlLexContext::Svelte);
    parse_svelte_name(p).or_add_diagnostic(p, |p, range| {
        p.err_builder("Expected a valid Svelte name after '...'", range)
    });

    Present(m.complete(p, SVELTE_REST_BINDING))
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

#[derive(Debug)]
struct SvelteBindingAssignmentBindingList;

impl ParseSeparatedList for SvelteBindingAssignmentBindingList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = SVELTE_BINDING_ASSIGNMENT_BINDING_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if p.at(T![...]) {
            parse_rest_name(p)
        } else {
            parse_svelte_name(p)
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}']) || p.at(T![']'])
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

// #region Directives parsing functions

pub(crate) fn parse_svelte_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !is_at_svelte_directive_start(p) {
        return Absent;
    }

    // relex token so we acknowledge the keyword
    p.re_lex(HtmlReLexContext::Svelte);

    match p.cur() {
        T![bind] => parse_directive(p, T![bind], SVELTE_BIND_DIRECTIVE, HtmlLexContext::Svelte),
        T![transition] => parse_directive(
            p,
            T![transition],
            SVELTE_TRANSITION_DIRECTIVE,
            HtmlLexContext::Svelte,
        ),
        T![in] => parse_directive(p, T![in], SVELTE_IN_DIRECTIVE, HtmlLexContext::Svelte),
        T![out] => parse_directive(p, T![out], SVELTE_OUT_DIRECTIVE, HtmlLexContext::Svelte),
        T![class] => parse_directive(
            p,
            T![class],
            SVELTE_CLASS_DIRECTIVE,
            HtmlLexContext::SvelteBindingLiteral,
        ),
        T![style] => parse_directive(
            p,
            T![style],
            SVELTE_STYLE_DIRECTIVE,
            HtmlLexContext::SvelteBindingLiteral,
        ),
        T![use] => parse_directive(p, T![use], SVELTE_USE_DIRECTIVE, HtmlLexContext::Svelte),
        T![animate] => parse_directive(
            p,
            T![animate],
            SVELTE_ANIMATE_DIRECTIVE,
            HtmlLexContext::Svelte,
        ),
        _ => Absent,
    }
}

fn parse_directive_value(p: &mut HtmlParser, context_after_colon: HtmlLexContext) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T![:], context_after_colon);
    if p.cur_text().is_empty() {
        p.error(p.err_builder("The directive can't be empty.", p.cur_range()))
    } else if context_after_colon == HtmlLexContext::SvelteBindingLiteral {
        parse_binding_literal(p).or_add_diagnostic(p, expected_svelte_property);
    } else {
        parse_svelte_name(p).or_add_diagnostic(p, expected_name);
    }

    ModifiersList.parse_list(p);

    if p.at(T![=]) {
        parse_attribute_initializer(p).ok();
    } else {
        p.re_lex(HtmlReLexContext::InsideTag);
    }

    Present(m.complete(p, SVELTE_DIRECTIVE_VALUE))
}

/// Parses a general directive. `token` is the keyword to parse, and `node_kind` is the kind of the node to emit.
fn parse_directive(
    p: &mut HtmlParser,
    token: HtmlSyntaxKind,
    node_kind: HtmlSyntaxKind,
    context_after_colon: HtmlLexContext,
) -> ParsedSyntax {
    if !p.at(token) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(token, HtmlLexContext::Svelte);

    parse_directive_value(p, context_after_colon).or_add_diagnostic(p, expected_valid_directive);

    Present(m.complete(p, node_kind))
}

struct ModifiersList;

impl ParseNodeList for ModifiersList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = SVELTE_DIRECTIVE_MODIFIER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        let m = p.start();
        p.expect_with_context(T![|], HtmlLexContext::Svelte);
        parse_svelte_name(p).or_add_diagnostic(p, |p, range| {
            p.err_builder("Expected a valid Svelte modifier name", range)
        });
        Present(m.complete(p, SVELTE_DIRECTIVE_MODIFIER))
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        !p.at(T![|])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SVELTE_BOGUS_BLOCK, BLOCK_RECOVER),
            expected_name,
        )
    }
}
// #endregion

// #region Check functions

const SVELTE_KEYWORDS: TokenSet<HtmlSyntaxKind> = token_set!(
    T![if],
    T![else],
    T![each],
    T![debug],
    T![const],
    T![attach],
    T![render],
    T![key],
    T![as],
    T![await],
    T![catch],
    T![then],
    T![snippet],
)
.union(SVELTE_DIRECTIVE_KEYWORDS);

const SVELTE_DIRECTIVE_KEYWORDS: TokenSet<HtmlSyntaxKind> = token_set!(
    T![bind],
    T![transition],
    T![in],
    T![out],
    T![class],
    T![style],
    T![use],
    T![animate]
);

pub(crate) fn is_at_svelte_keyword(p: &HtmlParser) -> bool {
    p.at_ts(SVELTE_KEYWORDS)
}

fn is_at_svelte_directive_keyword(token: HtmlSyntaxKind) -> bool {
    SVELTE_DIRECTIVE_KEYWORDS.contains(token)
}

fn is_at_else_opening_block(p: &mut HtmlParser) -> bool {
    p.at(T!["{:"]) && p.nth_at(1, T![else])
}

fn is_at_then_or_catch_block(p: &mut HtmlParser) -> bool {
    p.at(T!["{:"]) && (p.nth_at(1, T![then]) || p.nth_at(1, T![catch]))
}

pub(crate) fn is_at_svelte_directive_start(p: &mut HtmlParser) -> bool {
    if Svelte.is_unsupported(p) {
        return false;
    }
    let checkpoint = p.checkpoint();
    p.re_lex(HtmlReLexContext::Svelte);
    let first_token = p.cur();

    p.bump_any_with_context(HtmlLexContext::Svelte);
    let second_token = p.cur();

    p.rewind(checkpoint);

    second_token == T![:] && is_at_svelte_directive_keyword(first_token)
}

// #endregion
