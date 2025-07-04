mod error;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::at_rule::container::error::{
    expected_any_container_style_in_parens, expected_any_container_style_query,
};
use crate::syntax::at_rule::error::{
    AnyInParensChainParseRecovery, AnyInParensParseRecovery, AnyQueryParseRecovery,
};
use crate::syntax::at_rule::feature::{expected_any_query_feature, parse_any_query_feature};
use crate::syntax::block::parse_conditional_block;
use crate::syntax::parse_error::expected_non_css_wide_keyword_identifier;
use crate::syntax::{is_at_declaration, parse_custom_identifier, parse_declaration};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;
use error::{expected_any_container_query, expected_any_container_query_in_parens};

/// Checks if the current token in the parser is an `@container` at-rule.
#[inline]
pub(crate) fn is_at_container_at_rule(p: &mut CssParser) -> bool {
    p.at(T![container])
}

/// Parses an `@container` at-rule in a CSS stylesheet.
///
/// This function processes the `@container` rule, as specified in the
/// [CSS Conditional Rules Module Level 5](https://drafts.csswg.org/css-conditional-5/#container-rule),
///
/// # Examples
/// Basic usage in CSS:
///
/// ```css
/// @container (min-width: 400px) {
///     /* Styles apply only if the container is at least 400px wide */
///     .sidebar {
///         padding: 1rem;
///     }
/// }
/// ```
///
/// Named containers:
///
/// ```css
/// @container sidebar (min-width: 300px) {
///     /* Styles apply only if the container named "sidebar" is at least 300px wide */
///     .item {
///         display: flex;
///     }
/// }
/// ```
///
/// Multiple container queries chained with logical operators:
///
/// ```css
/// @container (width > 300px) and (height > 200px) {
///     /* Only applies if both width and height exceed the specified thresholds */
///     .article {
///         margin: 2rem;
///     }
/// }
/// ```
#[inline]
pub(crate) fn parse_container_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![container]);

    if !is_at_container_style_query_in_parens(p) {
        let name = parse_custom_identifier(p, CssLexContext::Regular);
        // Because the name is optional, we have to indirectly check if it's
        // a CSS-wide keyword that can't be used. If it was required, we could
        // use `.or_recover` or `.or_add_diagnostic` here instead.
        if name.is_absent() && p.cur().is_css_wide_keyword() {
            p.err_and_bump(
                expected_non_css_wide_keyword_identifier(p, p.cur_range()),
                CSS_BOGUS,
            )
        }
    }

    parse_any_container_query(p)
        .or_recover(p, &AnyQueryParseRecovery, expected_any_container_query)
        .ok();
    parse_conditional_block(p);

    Present(m.complete(p, CSS_CONTAINER_AT_RULE))
}

#[inline]
fn parse_any_container_query(p: &mut CssParser) -> ParsedSyntax {
    if is_at_container_not_query(p) {
        parse_container_not_query(p)
    } else {
        parse_any_container_query_in_parens(p).map(|lhs| match p.cur() {
            T![and] => parse_container_and_query(p, lhs),
            T![or] => parse_container_or_query(p, lhs),
            _ => lhs,
        })
    }
}

/// Parses an `and` condition for a container query, chaining multiple conditions.
///
/// # Example
/// ```css
/// @container (width > 300px) and (height > 200px) {
///   /* Styles apply only if both conditions are true. */
/// }
/// ```
#[inline]
fn parse_container_and_query(p: &mut CssParser, lhs: CompletedMarker) -> CompletedMarker {
    if !p.at(T![and]) {
        return lhs;
    }

    let m = lhs.precede(p);
    p.bump(T![and]);

    let recovery_result = parse_any_container_query_in_parens(p)
        .or_recover(
            p,
            &AnyInParensChainParseRecovery::new(T![and]),
            expected_any_container_query_in_parens,
        )
        .map(|rhs| parse_container_and_query(p, rhs));

    if recovery_result.is_err() && p.at(T![and]) {
        // If we're here, it seems that we have
        // @container (width > 300px) and <missing exp> and <missing exp> and ...
        // parse_any_container_query_in_parens failed to parse,
        // but the parser is already at a recovered position.
        let m = p.start();
        let rhs = m.complete(p, CSS_BOGUS);
        parse_container_and_query(p, rhs);
    }

    m.complete(p, CSS_CONTAINER_AND_QUERY)
}

/// Parses an `or` condition for a container query, allowing alternative conditions.
///
/// # Example
/// ```css
/// @container (orientation: landscape) or (width > 400px) { }
/// ```
#[inline]
fn parse_container_or_query(p: &mut CssParser, lhs: CompletedMarker) -> CompletedMarker {
    if !p.at(T![or]) {
        return lhs;
    }

    let m = lhs.precede(p);
    p.bump(T![or]);

    let recovery_result = parse_any_container_query_in_parens(p)
        .or_recover(
            p,
            &AnyInParensChainParseRecovery::new(T![or]),
            expected_any_container_query_in_parens,
        )
        .map(|rhs| parse_container_or_query(p, rhs));

    if recovery_result.is_err() && p.at(T![or]) {
        // If we're here, it seems that we have
        // @container (width > 300px) or or or
        // and parse_any_container_query_in_parens failed to parse,
        // but the parser is already at a recovered position.
        let m = p.start();
        let rhs = m.complete(p, CSS_BOGUS);
        parse_container_or_query(p, rhs);
    }

    m.complete(p, CSS_CONTAINER_OR_QUERY)
}

#[inline]
fn is_at_container_not_query(p: &mut CssParser) -> bool {
    p.at(T![not])
}

/// Parses a negated container query using the `not(...)` syntax.
///
/// # Example
/// ```css
/// @container not(width > 300px) { }
/// ```
#[inline]
fn parse_container_not_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_not_query(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![not]);
    parse_any_container_query_in_parens(p)
        .or_recover(
            p,
            &AnyQueryParseRecovery,
            expected_any_container_query_in_parens,
        )
        .ok();

    Present(m.complete(p, CSS_CONTAINER_NOT_QUERY))
}

#[inline]
fn parse_any_container_query_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if is_at_container_style_query_in_parens(p) {
        parse_container_style_query_in_parens(p)
    } else if is_at_container_query_in_parens(p) {
        parse_container_query_in_parens(p)
    } else if is_at_container_size_feature_in_parens(p) {
        parse_container_size_feature_in_parens(p)
    } else {
        Absent
    }
}

#[inline]
fn is_at_container_query_in_parens(p: &mut CssParser) -> bool {
    p.at(T!['(']) && (p.nth_at(1, T![not]) || p.nth_at(1, T!['(']))
}

/// Parses a parenthesized container query.
///
/// # Examples
///
/// ```css
/// @container ( (width > 300px) ) { }
/// ```
#[inline]
fn parse_container_query_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_query_in_parens(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    parse_any_container_query(p)
        .or_recover(p, &AnyInParensParseRecovery, expected_any_container_query)
        .ok();
    p.expect(T![')']);

    Present(m.complete(p, CSS_CONTAINER_QUERY_IN_PARENS))
}

#[inline]
fn is_at_container_size_feature_in_parens(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

/// Parses a parenthesized [container size feature] query.
/// # Examples
///
/// ```css
/// @container (min-width: 400px) { }
/// ```
#[inline]
fn parse_container_size_feature_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_size_feature_in_parens(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    parse_any_query_feature(p)
        .or_recover(p, &AnyInParensParseRecovery, expected_any_query_feature)
        .ok();
    p.expect(T![')']);

    Present(m.complete(p, CSS_CONTAINER_SIZE_FEATURE_IN_PARENS))
}

#[inline]
fn is_at_container_style_query_in_parens(p: &mut CssParser) -> bool {
    p.at(T![style])
}

/// Parses a parenthesized _container style query_ following the `style` keyword.
///
/// # Example
///
/// ```css
/// @container style(--my-prop: some-value) { }
/// ```
#[inline]
fn parse_container_style_query_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_style_query_in_parens(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![style]);
    p.expect(T!['(']);
    parse_any_container_style_query(p)
        .or_recover(
            p,
            &AnyInParensParseRecovery,
            expected_any_container_style_query,
        )
        .ok();
    p.expect(T![')']);

    Present(m.complete(p, CSS_CONTAINER_STYLE_QUERY_IN_PARENS))
}

#[inline]
fn parse_any_container_style_query(p: &mut CssParser) -> ParsedSyntax {
    if is_at_declaration(p) {
        parse_declaration(p)
    } else if is_at_container_style_not_query(p) {
        parse_container_style_not_query(p)
    } else {
        parse_container_style_in_parens(p).map(|lhs| match p.cur() {
            T![and] => parse_container_style_combinable_and_query(p, lhs),
            T![or] => parse_container_style_combinable_or_query(p, lhs),
            _ => lhs,
        })
    }
}

/// Parses a logical _and_ sequence in a container style query chain.
///
/// # Examples
///
/// ```css
/// @container style((color: red) and (background: blue)) { }
/// ```
#[inline]
fn parse_container_style_combinable_and_query(
    p: &mut CssParser,
    lhs: CompletedMarker,
) -> CompletedMarker {
    if !p.at(T![and]) {
        return lhs;
    }

    let m = lhs.precede(p);
    p.bump(T![and]);

    let recovery_result = parse_container_style_in_parens(p)
        .or_recover(
            p,
            &AnyContainerStyleQueryInParensChainParseRecovery::new(T![and]),
            expected_any_container_style_in_parens,
        )
        .map(|rhs| parse_container_style_combinable_and_query(p, rhs));

    if recovery_result.is_err() && p.at(T![and]) {
        // If we're here, it seems that we have
        // @container style((--b: color) and <missing exp> and <missing exp> and ...
        // parse_container_style_in_parens failed to parse,
        // but the parser is already at a recovered position.
        let m = p.start();
        let rhs = m.complete(p, CSS_BOGUS);
        parse_container_style_combinable_and_query(p, rhs);
    }

    m.complete(p, CSS_CONTAINER_STYLE_AND_QUERY)
}

/// Parses a logical _or_ sequence in a container style query chain.
///
/// # Examples
///
/// ```css
/// @container style((color: red) or (background: blue)) { }
/// ```
#[inline]
fn parse_container_style_combinable_or_query(
    p: &mut CssParser,
    lhs: CompletedMarker,
) -> CompletedMarker {
    if !p.at(T![or]) {
        return lhs;
    }

    let m = lhs.precede(p);
    p.bump(T![or]);

    let recovery_result = parse_container_style_in_parens(p)
        .or_recover(
            p,
            &AnyContainerStyleQueryInParensChainParseRecovery::new(T![or]),
            expected_any_container_style_in_parens,
        )
        .map(|rhs| parse_container_style_combinable_or_query(p, rhs));

    if recovery_result.is_err() && p.at(T![or]) {
        // If we're here, it seems that we have
        // @container style((--b: color) or <missing exp> or <missing exp> and ...
        // parse_container_style_in_parens failed to parse,
        // but the parser is already at a recovered position.
        let m = p.start();
        let rhs = m.complete(p, CSS_BOGUS);
        parse_container_style_combinable_or_query(p, rhs);
    }

    m.complete(p, CSS_CONTAINER_STYLE_OR_QUERY)
}

struct AnyContainerStyleQueryInParensChainParseRecovery {
    chain_kind: CssSyntaxKind,
}

impl AnyContainerStyleQueryInParensChainParseRecovery {
    fn new(chain_kind: CssSyntaxKind) -> Self {
        Self { chain_kind }
    }
}

impl ParseRecovery for AnyContainerStyleQueryInParensChainParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // We need to recover from an invalid style query in parentheses:
        // 1. A sub-expression boundary (another "(" or ")").
        // 2. The start of a new block ("{").
        // 3. Another chain token (like `and` or `or`), which we stored as `chain_kind`.
        // 4. A preceding line break that indicates a likely end to the query.
        p.at(T!['('])
            || p.at(T![')'])
            || p.at(T!['{'])
            || p.at(self.chain_kind)
            || p.has_preceding_line_break()
    }
}

#[inline]
fn is_at_container_style_not_query(p: &mut CssParser) -> bool {
    p.at(T![not])
}

#[inline]
fn parse_container_style_not_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_style_not_query(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![not]);

    parse_container_style_in_parens(p)
        .or_recover(
            p,
            &AnyInParensParseRecovery,
            expected_any_container_style_in_parens,
        )
        .ok();

    Present(m.complete(p, CSS_CONTAINER_STYLE_NOT_QUERY))
}

#[inline]
fn is_at_container_style_in_parens(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
fn parse_container_style_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_style_in_parens(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    parse_any_container_style_query(p)
        .or_recover(
            p,
            &AnyInParensParseRecovery,
            expected_any_container_style_query,
        )
        .ok();
    p.expect(T![')']);
    Present(m.complete(p, CSS_CONTAINER_STYLE_IN_PARENS))
}
