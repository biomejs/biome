pub(crate) mod error;

use crate::parser::CssParser;
use crate::syntax::at_rule::error::{AnyInParensChainParseRecovery, AnyInParensParseRecovery};
use crate::syntax::at_rule::supports::error::{
    expected_any_supports_condition, expected_any_supports_condition_in_parens,
};
use crate::syntax::block::parse_conditional_block;
use crate::syntax::parse_error::{expected_declaration, expected_selector};
use crate::syntax::selector::parse_selector;
use crate::syntax::{is_nth_at_identifier, parse_any_value, parse_declaration};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

/// Checks if the current token in the parser is an `@supports` at-rule.
#[inline]
pub(crate) fn is_at_supports_at_rule(p: &mut CssParser) -> bool {
    p.at(T![supports])
}

/// Parses an `@supports` at-rule in a CSS stylesheet.
///
/// This function processes the `@supports` rule, as specified in the
/// [CSS Conditional Rules Module Level 3](https://drafts.csswg.org/css-conditional-3/#at-supports),
/// which checks whether the user agent supports certain property-value pairs
/// or other CSS features.
///
/// # Examples
///
/// Basic usage:
/// ```css
/// @supports (display: grid) {
///     .container {
///         display: grid;
///     }
/// }
/// ```
///
/// Chaining multiple conditions with logical operators:
/// ```css
/// @supports (display: flex) and (not (animation-name: slidein)) {
///     /* Only apply these styles if flex is supported
///        and the user agent does not support animations named slidein */
///     .sidebar {
///         padding: 1rem;
///     }
/// }
/// ```
///
/// Using functional or selector checks:
/// ```css
/// @supports selector(:focus-visible) {
///     /* apply these styles if the :focus-visible pseudo-class is recognized */
/// }
/// ```
#[inline]
pub(crate) fn parse_supports_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_supports_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![supports]);

    parse_any_supports_condition(p)
        .or_recover(
            p,
            &AnySupportsConditionParseRecovery,
            expected_any_supports_condition,
        )
        .ok();
    parse_conditional_block(p);

    Present(m.complete(p, CSS_SUPPORTS_AT_RULE))
}

struct AnySupportsConditionParseRecovery;

impl ParseRecovery for AnySupportsConditionParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_SUPPORTS_CONDITION;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // We need to recover the invalid container query:
        // 1. The next block starts.
        //    @supports -- name { <--------------------+
        //              ^^^^^^^ we skip it until the block starts
        //   }
        // 2. The line break is missing before the next block.
        //    @supports -- name
        //              ^^^^^^^ we skip it but the block start token is missing
        //   }
        p.at(T!['{']) || p.has_preceding_line_break()
    }
}

#[inline]
pub(crate) fn parse_any_supports_condition(p: &mut CssParser) -> ParsedSyntax {
    if is_at_supports_not_condition(p) {
        parse_supports_not_condition(p)
    } else {
        parse_any_supports_condition_in_parens(p, None).map(|lhs| match p.cur() {
            T![and] => parse_supports_and_condition(p, lhs),
            T![or] => parse_supports_or_condition(p, lhs),
            _ => lhs,
        })
    }
}

#[inline]
fn parse_supports_and_condition(p: &mut CssParser, lhs: CompletedMarker) -> CompletedMarker {
    if !p.at(T![and]) {
        return lhs;
    }

    let m = lhs.precede(p);
    p.bump(T![and]);

    let recovery_result = parse_any_supports_condition_in_parens(p, Some(T![and]))
        .or_recover(
            p,
            &AnyInParensChainParseRecovery::new(T![and]),
            expected_any_supports_condition_in_parens,
        )
        .map(|rhs| parse_supports_and_condition(p, rhs));

    if recovery_result.is_err() && p.at(T![and]) {
        // If we're here, it seems that we have
        // @supports (display: flex) and <missing exp> and <missing exp> and ...
        // parse_any_supports_condition_in_parens failed to parse,
        // but the parser is already at a recovered position.
        let m = p.start();
        let rhs = m.complete(p, CSS_BOGUS);
        parse_supports_and_condition(p, rhs);
    }

    m.complete(p, CSS_SUPPORTS_AND_CONDITION)
}

#[inline]
fn parse_supports_or_condition(p: &mut CssParser, lhs: CompletedMarker) -> CompletedMarker {
    if !p.at(T![or]) {
        return lhs;
    }

    let m = lhs.precede(p);
    p.bump(T![or]);

    let recovery_result = parse_any_supports_condition_in_parens(p, Some(T![or]))
        .or_recover(
            p,
            &AnyInParensChainParseRecovery::new(T![or]),
            expected_any_supports_condition_in_parens,
        )
        .map(|rhs| parse_supports_or_condition(p, rhs));

    if recovery_result.is_err() && p.at(T![or]) {
        // If we're here, it seems that we have
        // @supports (display: flex) or <missing exp> or <missing exp> or ...
        // parse_any_supports_condition_in_parens failed to parse,
        // but the parser is already at a recovered position.
        let m = p.start();
        let rhs = m.complete(p, CSS_BOGUS);
        parse_supports_or_condition(p, rhs);
    }

    m.complete(p, CSS_SUPPORTS_OR_CONDITION)
}

#[inline]
pub(crate) fn is_at_supports_not_condition(p: &mut CssParser) -> bool {
    p.at(T![not])
}
#[inline]
fn parse_supports_not_condition(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_supports_not_condition(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![not]);
    parse_any_supports_condition_in_parens(p, None)
        .or_recover(
            p,
            &AnyInParensParseRecovery,
            expected_any_supports_condition_in_parens,
        )
        .ok();

    Present(m.complete(p, CSS_SUPPORTS_NOT_CONDITION))
}

#[inline]
fn parse_any_supports_condition_in_parens(
    p: &mut CssParser,
    chain_token: Option<CssSyntaxKind>,
) -> ParsedSyntax {
    if is_at_supports_feature_selector(p) {
        parse_supports_feature_selector(p)
    } else if is_at_supports_feature_declaration(p) {
        parse_supports_feature_declaration(p)
    } else if is_at_supports_condition_in_parens(p) {
        parse_supports_condition_in_parens(p)
    } else {
        // Here we're inside a <general-enclosed> branch,
        // which means that the parser is at unknown syntax.

        // If we're inside a chain, we can try to recover over a chain token.
        if let Some(chain_token) = chain_token
            && p.at(chain_token)
        {
            return Absent;
        }
        parse_any_value(p)
    }
}

#[inline]
fn is_at_supports_condition_in_parens(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
fn parse_supports_condition_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_supports_condition_in_parens(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    parse_any_supports_condition(p)
        .or_recover(
            p,
            &AnyInParensParseRecovery,
            expected_any_supports_condition,
        )
        .ok();
    p.expect(T![')']);

    Present(m.complete(p, CSS_SUPPORTS_CONDITION_IN_PARENS))
}

#[inline]
fn is_at_supports_feature_selector(p: &mut CssParser) -> bool {
    p.at(T![selector]) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_supports_feature_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_supports_feature_selector(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![selector]);
    p.bump(T!['(']);
    parse_selector(p)
        .or_recover(p, &AnyInParensParseRecovery, expected_selector)
        .ok();
    p.expect(T![')']);

    Present(m.complete(p, CSS_SUPPORTS_FEATURE_SELECTOR))
}

#[inline]
fn is_at_supports_feature_declaration(p: &mut CssParser) -> bool {
    p.at(T!['(']) && is_nth_at_identifier(p, 1) && p.nth_at(2, T![:])
}

#[inline]
fn parse_supports_feature_declaration(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_supports_feature_declaration(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    parse_declaration(p)
        .or_recover(p, &AnyInParensParseRecovery, expected_declaration)
        .ok();
    p.expect(T![')']);

    Present(m.complete(p, CSS_SUPPORTS_FEATURE_DECLARATION))
}
