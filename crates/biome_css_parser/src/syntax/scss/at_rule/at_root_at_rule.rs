use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::selector::SelectorList;
use crate::syntax::{is_at_identifier, parse_custom_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::{
    CSS_BOGUS_CUSTOM_IDENTIFIER, SCSS_AT_ROOT_AT_RULE, SCSS_AT_ROOT_QUERY, SCSS_AT_ROOT_QUERY_LIST,
    SCSS_AT_ROOT_SELECTOR,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::diagnostic::expected_any;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

/// Parses the SCSS `@at-root` at-rule.
///
/// # Example
///
/// ```scss
/// @at-root (without: media) {
///   .root-only {
///     color: red;
///   }
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/at-root/
#[inline]
pub(crate) fn parse_scss_at_root_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_at_root_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![at_root]);
    let query = parse_scss_at_root_query(p);

    if query.is_present() || p.at(T!['{']) {
        parse_declaration_or_rule_list_block(p);
    } else {
        parse_scss_at_root_selector(p);
        parse_declaration_or_rule_list_block(p);
    }

    Present(m.complete(p, SCSS_AT_ROOT_AT_RULE))
}

#[inline]
fn is_at_scss_at_root_at_rule(p: &mut CssParser) -> bool {
    p.at(T![at_root])
}

/// Parses the optional `@at-root` query clause.
///
/// # Example
///
/// ```scss
/// @at-root (without: media supports) {
///           ^^^^^^^^^^^^^^^^^^^^^^^^^
///   .root-only {
///     color: red;
///   }
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/at-root/#beyond-style-rules
#[inline]
fn parse_scss_at_root_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_at_root_query(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    parse_scss_at_root_query_modifier(p);
    p.expect(T![:]);

    if p.at(T![')']) {
        p.error(expected_identifier(p, p.cur_range()));
    }
    ScssAtRootQueryList.parse_list(p);

    p.expect(T![')']);

    Present(m.complete(p, SCSS_AT_ROOT_QUERY))
}

#[inline]
fn is_at_scss_at_root_query(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
fn parse_scss_at_root_query_modifier(p: &mut CssParser) {
    if p.at(T![with]) {
        p.bump(T![with]);
    } else if p.at(T![without]) {
        p.bump(T![without]);
    } else {
        p.error(expected_any(&["with", "without"], p.cur_range(), p));
        // Consume a stray identifier so recovery can continue at the `:` token.
        parse_regular_identifier(p).ok();
    }
}

/// Parses the selector shorthand used by `@at-root <selector> { ... }`.
///
/// # Example
///
/// ```scss
/// @at-root .root-only, .another-root {
///          ^^^^^^^^^^^^^^^^^^^^^^^^^
///   color: red;
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/at-root/
#[inline]
fn parse_scss_at_root_selector(p: &mut CssParser) -> CompletedMarker {
    let m = p.start();

    SelectorList::default().parse_list(p);

    m.complete(p, SCSS_AT_ROOT_SELECTOR)
}

struct ScssAtRootQueryList;

impl ParseNodeList for ScssAtRootQueryList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_AT_ROOT_QUERY_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_custom_identifier(p, CssLexContext::Regular)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &ScssAtRootQueryListParseRecovery, expected_identifier)
    }
}

struct ScssAtRootQueryListParseRecovery;

impl ParseRecovery for ScssAtRootQueryListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_CUSTOM_IDENTIFIER;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')']) || is_at_identifier(p)
    }
}
