use biome_css_syntax::CssSyntaxKind;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::parse_recovery::RecoveryResult;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::{CompletedMarker, ParsedSyntax};

use crate::parser::CssParser;
use crate::syntax::at_rule::container::error::expected_any_container_style_query;
use crate::syntax::at_rule::container::parse_any_container_style_query;
use crate::syntax::at_rule::error::AnyInParensParseRecovery;
use crate::syntax::at_rule::feature::expected_any_query_feature;
use crate::syntax::at_rule::feature::parse_any_query_feature;
use crate::syntax::at_rule::media::is_at_any_media_condition;
use crate::syntax::at_rule::media::parse_any_media_condition;
use crate::syntax::at_rule::supports::AnySupportsConditionParseRecovery;
use crate::syntax::at_rule::supports::error::expected_any_supports_condition;
use crate::syntax::at_rule::supports::parse_any_supports_condition;
use crate::syntax::is_at_declaration;
use crate::syntax::parse_declaration;
use crate::syntax::property::GenericComponentValueList;
use crate::syntax::value::parse_error::expected_if_branch;
use crate::syntax::value::parse_error::expected_if_test_boolean_expr_group;

pub(crate) fn is_at_if_function(p: &mut CssParser) -> bool {
    p.at(T![if])
}

/// Parses an if function from the current position of the CSS parser.
///
/// For more detailed information on the CSS if function syntax, refer to the
/// [CSS Values and Units Module](https://drafts.csswg.org/css-values-5/#if-notation).
///
/// # If Function Syntax Examples
///
/// - Style if condition:
///   ``` css
///   if(style(--scheme: dark): #eeeeee;)
///   ```
///   Demonstrates a style if condition that checks if the --scheme variable is set to dark.
///
/// - Media if condition:
///   ``` css
///   if(media(print): white; else: black;)
///   ```
///   Demonstrates a media if condition that checks if the media query is active for print.
///
/// - Supports if condition:
///   ``` css
///   if(supports(color: lch(7.1% 60.23 300.16)): lch(7.1% 60.23 300.16);)
///   ```
///   Demonstrates a supports if condition that checks if lch color is supported.
///
/// - Else if condition:
///   ``` css
///   if(style(--size: "2xl"): 1em; else: 0.25em;)
///   ```
///   Demonstrates an else if condition that checks if the --size variable is set to "2xl".
///
/// - Multiple if conditions:
///   ``` css
///   if(
///     style(--scheme: ice): linear-gradient(#caf0f8, white, #caf0f8);
///     style(--scheme: fire): linear-gradient(#ffc971, white, #ffc971);
///     else: none;
///   )
///   ```
///  - If test with shortand:
///  ``` css
///  3px yellow if(
///    style(--color: green): dashed;
///    style(--color: yellow): inset;
///    else: solid;
///  )
///  ```
///
/// # Grammar
///
/// ``` txt
/// <if()> = if( [ <if-branch> ; ]* <if-branch> ;? )
/// <if-branch> = <if-condition> : <declaration-value>?
/// <if-condition> = <boolean-expr[ <if-test> ]> | else
/// <if-test> =
///   supports( [ <ident> : <declaration-value> ] | <supports-condition> ) |
///   media( <media-feature> | <media-condition> ) |
///   style( <style-query> )
/// ```
pub(crate) fn parse_if_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_function(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![if]);
    p.expect(T!['(']);

    CssIfBranchList.parse_list(p);

    p.expect(T![')']);

    Present(m.complete(p, CSS_IF_FUNCTION))
}

#[inline]
fn is_at_if_supports_test(p: &mut CssParser) -> bool {
    p.at(T![supports]) && p.nth_at(1, T!['('])
}

/// Parses a supports if condition test.
///
/// # Example
///
/// ```css
/// supports(color: lch(7.1% 60.23 300.16)
/// ```
#[inline]
fn parse_if_supports_test(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_supports_test(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![supports]);
    p.bump(T!['(']);

    if is_at_declaration(p) {
        parse_declaration(p).ok();
    } else {
        parse_any_supports_condition(p)
            .or_recover(
                p,
                &AnySupportsConditionParseRecovery,
                expected_any_supports_condition,
            )
            .ok();
    }

    p.expect(T![')']);
    Present(m.complete(p, CSS_IF_SUPPORTS_TEST))
}

#[inline]
fn is_at_if_style_test(p: &mut CssParser) -> bool {
    p.at(T![style]) && p.nth_at(1, T!['('])
}

/// Parses a style if condition test.
///
/// # Example
///
/// ``` css
/// style(--scheme: dark)
/// ```
#[inline]
fn parse_if_style_test(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_style_test(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![style]);
    p.bump(T!['(']);

    parse_any_container_style_query(p)
        .or_recover(
            p,
            &AnyInParensParseRecovery,
            expected_any_container_style_query,
        )
        .ok();
    p.expect(T![')']);

    Present(m.complete(p, CSS_IF_STYLE_TEST))
}

#[inline]
fn is_at_if_media_test(p: &mut CssParser) -> bool {
    p.at(T![media]) && p.nth_at(1, T!['('])
}

/// Parses a media if condition test.
///
/// # Example
///
/// ``` css
/// media(print)
/// ```
#[inline]
fn parse_if_media_test(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_media_test(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![media]);
    p.bump(T!['(']);

    if is_at_any_media_condition(p) {
        parse_any_media_condition(p)
            .or_recover(p, &AnyInParensParseRecovery, expected_any_query_feature)
            .ok();
    } else {
        parse_any_query_feature(p)
            .or_recover(p, &AnyInParensParseRecovery, expected_any_query_feature)
            .ok();
    }

    p.expect(T![')']);

    Present(m.complete(p, CSS_IF_MEDIA_TEST))
}

#[inline]
fn parse_if_test(p: &mut CssParser) -> ParsedSyntax {
    if is_at_if_supports_test(p) {
        return parse_if_supports_test(p);
    }

    if is_at_if_style_test(p) {
        return parse_if_style_test(p);
    }

    if is_at_if_media_test(p) {
        return parse_if_media_test(p);
    }

    Absent
}

#[inline]
fn parse_any_if_test_boolean_expr_group(p: &mut CssParser) -> ParsedSyntax {
    // ( <boolean-expr> )
    if p.at(T!['(']) {
        let m = p.start();
        p.bump(T!['(']);
        parse_any_if_test_boolean_expr(p).ok();
        p.expect(T![')']);
        return Present(m.complete(p, CSS_IF_TEST_BOOLEAN_EXPR_IN_PARENS));
    }

    // <if-test>
    parse_if_test(p)
}

fn is_at_if_test_boolean_not_expr(p: &mut CssParser) -> bool {
    p.at(T![not])
}

/// Parses `not <boolean-expr-group>`
///
/// # Example
///
/// ``` css
/// not style(--color: green)
/// ```
#[inline]
fn parse_if_test_boolean_not_expr(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_test_boolean_not_expr(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![not]);
    parse_any_if_test_boolean_expr_group(p).ok();

    Present(m.complete(p, CSS_IF_TEST_BOOLEAN_NOT_EXPR))
}

#[inline]
fn is_at_if_test_boolean_and_expr(p: &mut CssParser) -> bool {
    p.at(T![and])
}

/// Parses `<boolean-expr-group> and <boolean-expr-group>`
///
/// # Example
///
/// ``` css
/// style(--color: green) and style(--color: yellow)
/// ```
#[inline]
fn parse_if_test_boolean_and_expr(p: &mut CssParser, lhs: CompletedMarker) -> CompletedMarker {
    if !is_at_if_test_boolean_and_expr(p) {
        return lhs;
    }

    let m = lhs.precede(p);
    p.bump(T![and]);

    let recovery_result = parse_any_if_test_boolean_expr_group(p)
        .or_recover(
            p,
            &AnyIfTestBooleanExprChainParseRecovery,
            expected_if_test_boolean_expr_group,
        )
        .map(|rhs| parse_if_test_boolean_and_expr(p, rhs));

    if recovery_result.is_err() && p.at(T![and]) {
        // If we're here, it seems that we have
        // if(...) and <missing exp> and <missing exp> and ...
        // parse_any_if_test_boolean_expr_group failed to parse,
        // but the parser is already at a recovered position.
        let m = p.start();
        let rhs = m.complete(p, CSS_BOGUS);
        parse_if_test_boolean_and_expr(p, rhs);
    }

    m.complete(p, CSS_IF_TEST_BOOLEAN_AND_EXPR)
}

#[inline]
fn is_at_if_test_boolean_or_expr(p: &mut CssParser) -> bool {
    p.at(T![or])
}

/// Parses `<boolean-expr-group> or <boolean-expr-group>`
///
/// # Example
///
/// ``` css
/// style(--color: green) or style(--color: yellow)
/// ```
#[inline]
fn parse_if_test_boolean_or_expr(p: &mut CssParser, lhs: CompletedMarker) -> CompletedMarker {
    if !is_at_if_test_boolean_or_expr(p) {
        return lhs;
    }

    let m = lhs.precede(p);
    p.bump(T![or]);

    let recovery_result = parse_any_if_test_boolean_expr_group(p)
        .or_recover(
            p,
            &AnyIfTestBooleanExprChainParseRecovery,
            expected_if_test_boolean_expr_group,
        )
        .map(|rhs| parse_if_test_boolean_or_expr(p, rhs));

    if recovery_result.is_err() && p.at(T![or]) {
        // If we're here, it seems that we have
        // if(...) or <missing exp> or <missing exp> or ...
        // parse_any_if_test_boolean_expr_group failed to parse,
        // but the parser is already at a recovered position.
        let m = p.start();
        let rhs = m.complete(p, CSS_BOGUS);
        parse_if_test_boolean_or_expr(p, rhs);
    }

    m.complete(p, CSS_IF_TEST_BOOLEAN_OR_EXPR)
}

#[inline]
fn parse_any_if_test_boolean_expr(p: &mut CssParser) -> ParsedSyntax {
    if is_at_if_test_boolean_not_expr(p) {
        return parse_if_test_boolean_not_expr(p);
    }

    parse_any_if_test_boolean_expr_group(p).map(|lhs| match p.cur() {
        T![and] => parse_if_test_boolean_and_expr(p, lhs),
        T![or] => parse_if_test_boolean_or_expr(p, lhs),
        _ => lhs,
    })
}

#[inline]
fn parse_any_if_condition(p: &mut CssParser) -> ParsedSyntax {
    if p.at(T![else]) {
        let m = p.start();
        p.bump(T![else]);
        return Present(m.complete(p, CSS_ELSE_KEYWORD));
    }

    parse_any_if_test_boolean_expr(p)
}

#[inline]
fn parse_if_branch(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();

    parse_any_if_condition(p)
        .or_recover(p, &AnyIfTestParseRecovery, expected_if_branch)
        .ok();

    p.expect(T![:]);

    GenericComponentValueList.parse_list(p);

    Present(m.complete(p, CSS_IF_BRANCH))
}

struct AnyIfTestBooleanExprChainParseRecovery;

impl ParseRecovery for AnyIfTestBooleanExprChainParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_if_test_boolean_not_expr(p)
            || is_at_if_test_boolean_and_expr(p)
            || is_at_if_test_boolean_or_expr(p)
            || p.has_preceding_line_break()
    }
}

struct AnyIfTestParseRecovery;

impl ParseRecovery for AnyIfTestParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_IF_TEST;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')']) || p.has_preceding_line_break()
    }
}

struct IfBranchListParseRecovery;

impl ParseRecovery for IfBranchListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_IF_BRANCH;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![;]) || p.at(T![')']) || p.has_preceding_line_break()
    }
}

struct CssIfBranchList;

impl ParseSeparatedList for CssIfBranchList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_IF_BRANCH_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_if_branch(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &IfBranchListParseRecovery, expected_if_branch)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![;]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }

    fn allow_empty(&self) -> bool {
        false
    }
}
