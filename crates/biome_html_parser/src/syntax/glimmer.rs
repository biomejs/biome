use crate::parser::HtmlParser;
use crate::syntax::parse_error::*;
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::{HtmlSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::Parser;

const RECOVER_GLIMMER_EXPRESSION: TokenSet<HtmlSyntaxKind> = token_set!(T![<], T![>], T!["}}"]);

/// Parse a Glimmer mustache expression: {{path args}}
///
/// Examples:
/// - `{{this.count}}`
/// - `{{@arg}}`
/// - `{{helper arg1 arg2 key=value}}`
pub(crate) fn parse_glimmer_mustache_expression(
    p: &mut HtmlParser,
    context: HtmlLexContext,
) -> ParsedSyntax {
    if !p.at(T!["{{"]) {
        return Absent;
    }

    let m = p.start();
    let opening_range = p.cur_range();
    p.bump_with_context(T!["{{"], context);

    // Parse the path
    parse_glimmer_path(p).or_add_diagnostic(p, expected_glimmer_path);

    // Parse arguments
    GlimmerArgumentList.parse_list(p);

    if p.at(T!["}}"]) {
        p.expect_with_context(T!["}}"], context);
        Present(m.complete(p, GLIMMER_MUSTACHE_EXPRESSION))
    } else {
        let diagnostic = expected_closing_mustache(p, p.cur_range(), opening_range);
        p.error(diagnostic);
        Present(m.complete(p, GLIMMER_BOGUS_EXPRESSION))
    }
}

/// Parse a Glimmer block helper: {{#helper args}}...{{/helper}}
///
/// Examples:
/// - `{{#if condition}}...{{/if}}`
/// - `{{#each items as |item index|}}...{{/each}}`
pub(crate) fn parse_glimmer_block_helper(p: &mut HtmlParser) -> ParsedSyntax {
    if !is_at_block_helper_opening(p) {
        return Absent;
    }

    let m = p.start();

    // Parse opening: {{#helper args}}
    parse_glimmer_block_helper_opening(p).ok();

    // Parse body (HTML element list)
    crate::syntax::ElementList.parse_list(p);

    // Parse closing: {{/helper}}
    parse_glimmer_block_helper_closing(p)
        .or_add_diagnostic(p, |p, range| expected_block_helper_closing(p, range));

    Present(m.complete(p, GLIMMER_BLOCK_HELPER))
}

fn is_at_block_helper_opening(p: &mut HtmlParser) -> bool {
    p.at(T!["{{"]) && p.nth_at(1, T![#])
}

fn parse_glimmer_block_helper_opening(p: &mut HtmlParser) -> ParsedSyntax {
    if !is_at_block_helper_opening(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T!["{{"], HtmlLexContext::Regular);
    p.bump(T![#]);

    // Parse helper name (path)
    parse_glimmer_path(p).or_add_diagnostic(p, expected_glimmer_path);

    // Parse arguments
    GlimmerArgumentList.parse_list(p);

    // Parse optional block params: as |item index|
    parse_glimmer_block_params(p).ok();

    if !p.expect(T!["}}"]) {
        return Present(m.complete(p, GLIMMER_BOGUS_EXPRESSION));
    }

    Present(m.complete(p, GLIMMER_BLOCK_HELPER_OPENING))
}

fn parse_glimmer_block_helper_closing(p: &mut HtmlParser) -> ParsedSyntax {
    if !(p.at(T!["{{"]) && p.nth_at(1, T![/])) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T!["{{"], HtmlLexContext::Regular);
    p.bump(T![/]);

    // Parse helper name (path)
    parse_glimmer_path(p).or_add_diagnostic(p, expected_glimmer_path);

    if !p.expect(T!["}}"]) {
        return Present(m.complete(p, GLIMMER_BOGUS_EXPRESSION));
    }

    Present(m.complete(p, GLIMMER_BLOCK_HELPER_CLOSING))
}

/// Parse block params: as |item index|
fn parse_glimmer_block_params(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![as]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![as]);
    p.expect(T![|]);

    GlimmerBlockParamList.parse_list(p);

    p.expect(T![|]);

    Present(m.complete(p, GLIMMER_BLOCK_PARAMS))
}

/// Parse a Glimmer path: this.foo, @arg, helper
///
/// Examples:
/// - `this`
/// - `this.foo.bar`
/// - `@arg`
/// - `helper`
fn parse_glimmer_path(p: &mut HtmlParser) -> ParsedSyntax {
    if !is_at_path_start(p) {
        return Absent;
    }

    let m = p.start();

    // Parse path segments
    GlimmerPathSegmentList.parse_list(p);

    Present(m.complete(p, GLIMMER_PATH))
}

fn is_at_path_start(p: &HtmlParser) -> bool {
    p.at(T![this]) || p.at(T![@]) || p.at(T![ident])
}

/// Parse a splattribute: ...attrs
pub(crate) fn parse_glimmer_splattribute(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![...]);

    if !p.expect(T![ident]) {
        return Present(m.complete(p, HTML_BOGUS_ATTRIBUTE));
    }

    Present(m.complete(p, GLIMMER_SPLATTRIBUTE))
}

// Parse lists

struct GlimmerArgumentList;

impl ParseNodeList for GlimmerArgumentList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;

    const LIST_KIND: Self::Kind = GLIMMER_ARGUMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_glimmer_argument(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        // Stop at closing delimiters or block params
        p.at(T!["}}"]) || p.at(T![as]) || p.at(T![|])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GLIMMER_BOGUS_EXPRESSION, RECOVER_GLIMMER_EXPRESSION),
            expected_glimmer_argument,
        )
    }
}

fn parse_glimmer_argument(p: &mut HtmlParser) -> ParsedSyntax {
    // Check if it's a named argument: key=value
    if is_at_named_argument(p) {
        parse_glimmer_named_argument(p)
    } else {
        parse_glimmer_positional_argument(p)
    }
}

fn is_at_named_argument(p: &mut HtmlParser) -> bool {
    p.at(T![ident]) && p.nth_at(1, T![=])
}

fn parse_glimmer_named_argument(p: &mut HtmlParser) -> ParsedSyntax {
    if !is_at_named_argument(p) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![ident]);
    p.expect(T![=]);

    parse_glimmer_argument_value(p).or_add_diagnostic(p, expected_glimmer_argument_value);

    Present(m.complete(p, GLIMMER_NAMED_ARGUMENT))
}

fn parse_glimmer_positional_argument(p: &mut HtmlParser) -> ParsedSyntax {
    if !is_at_argument_value(p) {
        return Absent;
    }

    let m = p.start();

    parse_glimmer_argument_value(p).or_add_diagnostic(p, expected_glimmer_argument_value);

    Present(m.complete(p, GLIMMER_POSITIONAL_ARGUMENT))
}

fn parse_glimmer_argument_value(p: &mut HtmlParser) -> ParsedSyntax {
    if is_at_path_start(p) {
        parse_glimmer_path(p)
    } else if p.at(HTML_STRING_LITERAL) {
        let m = p.start();
        p.bump(HTML_STRING_LITERAL);
        Present(m.complete(p, GLIMMER_STRING_LITERAL))
    } else if p.at(HTML_LITERAL) {
        let m = p.start();
        p.bump(HTML_LITERAL);
        Present(m.complete(p, GLIMMER_LITERAL))
    } else {
        Absent
    }
}

fn is_at_argument_value(p: &HtmlParser) -> bool {
    is_at_path_start(p) || p.at(HTML_STRING_LITERAL) || p.at(HTML_LITERAL)
}

struct GlimmerPathSegmentList;

impl ParseNodeList for GlimmerPathSegmentList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;

    const LIST_KIND: Self::Kind = GLIMMER_PATH_SEGMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_glimmer_path_segment(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        !is_at_path_segment(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GLIMMER_BOGUS_EXPRESSION, RECOVER_GLIMMER_EXPRESSION),
            expected_glimmer_path_segment,
        )
    }
}

fn parse_glimmer_path_segment(p: &mut HtmlParser) -> ParsedSyntax {
    if !is_at_path_segment(p) {
        return Absent;
    }

    let m = p.start();

    if p.at(T![this]) {
        p.bump(T![this]);
    } else if p.at(T![@]) {
        p.bump(T![@]);
    } else if p.at(T![.]) {
        p.bump(T![.]);
    } else if p.at(T![ident]) {
        p.bump(T![ident]);
    } else {
        return Absent;
    }

    Present(m.complete(p, GLIMMER_PATH_SEGMENT))
}

fn is_at_path_segment(p: &HtmlParser) -> bool {
    p.at(T![this]) || p.at(T![@]) || p.at(T![.]) || p.at(T![ident])
}

struct GlimmerBlockParamList;

impl ParseSeparatedList for GlimmerBlockParamList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;

    const LIST_KIND: Self::Kind = GLIMMER_BLOCK_PARAM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_glimmer_block_param(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![|]) || p.at(T!["}}"]) || p.at_ts(token_set![T![<], T![>]])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(GLIMMER_BOGUS_EXPRESSION, RECOVER_GLIMMER_EXPRESSION),
            expected_glimmer_block_param,
        )
    }

    fn separating_element_kind(&mut self) -> HtmlSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        false
    }
}

fn parse_glimmer_block_param(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![ident]);

    Present(m.complete(p, GLIMMER_BLOCK_PARAM))
}
