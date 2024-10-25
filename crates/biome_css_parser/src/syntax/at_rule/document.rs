use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::at_rule::parse_error::expected_any_document_matcher;
use crate::syntax::block::parse_rule_block;
use crate::syntax::parse_error::expected_string;
use crate::syntax::parse_string;
use crate::syntax::value::url::{is_at_url_function, parse_url_function, parse_url_value};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

/// Checks if the current token in the parser is a `@document` at-rule.
///
/// This function verifies whether the current token matches the `@document` rule,
/// which is used for applying styles to specific parts of a document.
#[inline]
pub(crate) fn is_at_document_at_rule(p: &mut CssParser) -> bool {
    p.at(T![document])
}

/// Parses a `@document` at-rule in a CSS stylesheet.
///
/// This function processes the `@document` at-rule, defined in the CSS Conditional Rules Module Level 4.
///
/// Specification:
/// [Initially](https://www.w3.org/TR/2012/WD-css3-conditional-20120911/#at-document) in Level 3, @document was [postponed](https://www.w3.org/TR/2012/WD-css3-conditional-20121213/#changes) to Level 4, but then subsequently removed.
///
/// # Examples
/// Basic usage in CSS:
///
/// ```css
/// @document url(http://www.example.com/), url-prefix(http://www.example.com/docs/), domain(example.com), regexp(".*") {
///     /* CSS rules here */
/// }
/// ```
///
/// This function is integral in parsing and interpreting `@document` rules as per the CSS Conditional Rules Module Level 4.
#[inline]
pub(crate) fn parse_document_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_document_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![document]);

    DocumentMatcherList.parse_list(p);
    parse_rule_block(p);

    Present(m.complete(p, CSS_DOCUMENT_AT_RULE))
}

struct DocumentMatcherListParseRecovery;

impl ParseRecovery for DocumentMatcherListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_DOCUMENT_MATCHER;
    /// Determines if the parser has reached a point where it can recover from an error
    /// while parsing a document matcher list.
    ///
    /// This function checks if the parser is at a position where it can safely resume parsing
    /// after encountering an error in a document matcher list. The recovery points are:
    /// - The start of a new document matcher.
    /// - An opening curly brace '{', indicating the start of a ruleset.
    /// - A comma ',', indicating the list separator.
    /// # Examples
    /// Basic usage in CSS:
    ///
    /// ```css
    /// @document url(http://example.com),
    /// invalid-url, /* Error in URL, recover here */
    /// url(http://example.org) { /* Start of ruleset, another recovery point */
    ///     /* CSS rules here */
    /// }
    /// ```
    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['{']) || p.at(T![,]) || is_at_document_matcher(p)
    }
}

pub(crate) struct DocumentMatcherList;

impl ParseSeparatedList for DocumentMatcherList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_DOCUMENT_MATCHER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_document_matcher(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['{'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &DocumentMatcherListParseRecovery,
            expected_any_document_matcher,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

/// Checks if the current token in the parser is a matcher for the `@document` at-rule.
///
/// This function determines whether the current token corresponds to either a custom matcher
/// for the `@document` rule or a URL function. It is used to identify the type of condition
/// under which the `@document` rule's styles should be applied.
#[inline]
pub(crate) fn is_at_document_matcher(p: &mut CssParser) -> bool {
    is_at_document_custom_matcher(p) || is_at_url_function(p)
}

/// Parses a matcher for the `@document` at-rule in a CSS stylesheet.
/// # Example
/// Basic usage in CSS:
///
/// ```css
/// @document url("http://example.com"), domain("example") {
///     /* CSS rules here */
/// }
/// ```
///
/// This function is crucial for parsing and interpreting the matchers in `@document` rules,
/// allowing for the specification of conditions under which CSS rules should be applied.
#[inline]
pub(crate) fn parse_document_matcher(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_document_matcher(p) {
        return Absent;
    }

    if is_at_url_function(p) {
        parse_url_function(p)
    } else {
        parse_document_custom_matcher(p)
    }
}

const DOCUMENT_CUSTOM_MATCHER_SET: TokenSet<CssSyntaxKind> =
    token_set!(T![url_prefix], T![domain], T![media_document], T![regexp]);

/// Checks if the current token in the parser is a custom matcher for the `@document` at-rule.
pub(crate) fn is_at_document_custom_matcher(p: &mut CssParser) -> bool {
    p.at_ts(DOCUMENT_CUSTOM_MATCHER_SET) && p.nth_at(1, T!['('])
}

// According to MDN, `url-prefix()`, `domain()` and `media-document()` functions
// can be optionally enclosed by single or double quotes.
// @see https://developer.mozilla.org/en-US/docs/Web/CSS/@document
const URL_PREFIX_SET: TokenSet<CssSyntaxKind> =
    token_set!(T![url_prefix], T![domain], T![media_document]);

pub(crate) fn is_at_url_prefix(p: &mut CssParser) -> bool {
    p.at_ts(URL_PREFIX_SET) && p.nth_at(1, T!['('])
}

/// Parses a custom matcher for the `@document` at-rule in a CSS stylesheet.
/// # Example
/// Basic usage in CSS:
///
/// ```css
/// @document domain("example") {
///     /* CSS rules here */
/// }
/// ```
///
/// This function is crucial for parsing custom matchers in `@document` rules, allowing for more specific rule application.
#[inline]
pub(crate) fn parse_document_custom_matcher(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_document_custom_matcher(p) {
        return Absent;
    }

    let m = p.start();

    if is_at_url_prefix(p) {
        p.bump_ts(URL_PREFIX_SET);
        p.bump_with_context(T!['('], CssLexContext::UrlRawValue);
        parse_url_value(p).ok();
        p.expect(T![')']);
        return Present(m.complete(p, CSS_DOCUMENT_CUSTOM_MATCHER));
    }

    p.bump_ts(DOCUMENT_CUSTOM_MATCHER_SET);
    p.bump(T!['(']);
    parse_string(p).or_add_diagnostic(p, expected_string);
    p.expect(T![')']);

    Present(m.complete(p, CSS_DOCUMENT_CUSTOM_MATCHER))
}
