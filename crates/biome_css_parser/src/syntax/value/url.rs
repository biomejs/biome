use crate::lexer::CssLexContext;
use crate::parser::CssParser;

use crate::syntax::value::function::{is_at_function, is_nth_at_function, parse_function};
use crate::syntax::value::parse_error::expected_url_modifier;
use crate::syntax::{is_at_identifier, is_at_string, parse_regular_identifier, parse_string};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

const URL_SET: TokenSet<CssSyntaxKind> = token_set![T![url], T![src]];

/// Determines if the current position of the parser is at the beginning of a URL function.
pub(crate) fn is_at_url_function(p: &mut CssParser) -> bool {
    p.at_ts(URL_SET) && p.nth_at(1, T!['('])
}

/// Parses a URL function from the current position of the CSS parser.
///
/// This function is designed to parse URL functions as specified in the CSS Values and Units Module.
/// The URL function can be of various forms and may include different types of content, like a standard
/// URL, a URL with quotes, a URL with variables, or a URL with special characters. This parser handles
/// these variations according to the syntax rules defined in the CSS specification.
///
/// For more detailed information on the CSS URL function syntax, refer to the
/// [CSS Values and Units Module](https://drafts.csswg.org/css-values-4/#url-value).
///
/// # URL Function Syntax Examples
///
/// - Standard URL: `url(image.jpg)`
///   Represents a basic URL function without any additional modifiers or quotes.
///
/// - URL with quotes: `url("image.jpg")`
///   A URL function where the URL is enclosed within quotes.
///
/// - URL with a variable: `url(var(--image-path))`
///   Demonstrates a URL function that uses a CSS variable as its value.
///
/// - URL with special characters: `url(http://example.com/image.jpg?size=large)`
///   Shows a URL function with a full HTTP URL, including special characters like query parameters.
///
/// # Grammar
///
/// The grammar for the URL function according to the CSS specification is as follows:
///
/// `<url>` = `<url()>` | `<src()>`
///
/// `<url()>` = url( `<string>` `<url-modifier>*` ) | `<url-token>`
/// `<src()>` = src( `<string>` `<url-modifier>*` )
///
/// Here, `<url-modifier>` represents any modifiers that might be applied to the URL, such as
/// resolution-based adjustments or format hints.
pub(crate) fn parse_url_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_url_function(p) {
        return Absent;
    }
    let m = p.start();

    p.bump_ts(URL_SET);

    if is_nth_at_function(p, 1) {
        // we need to check if the next token is a function or not
        // to cover the case of `src(var(--foo));`
        p.bump(T!['(']);
    } else {
        p.bump_with_context(T!['('], CssLexContext::UrlRawValue);
        parse_url_value(p).ok();
    }

    UrlModifierList.parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, CSS_URL_FUNCTION))
}

/// Determines if the current position of the parser is at a URL value.
///
/// This function checks if the parser's current position is at the beginning of either a raw URL value
/// or a string.
#[inline]
pub(crate) fn is_at_url_value(p: &mut CssParser) -> bool {
    is_at_url_value_raw(p) || is_at_string(p)
}

/// Parses a URL value from the current position of the CSS parser.
///
/// This function attempts to parse a URL value starting from the parser's current position.
/// If the current position is at a string, it parses using `parse_string`; otherwise, it uses `parse_url_value_raw` for a raw URL.
#[inline]
pub(crate) fn parse_url_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_url_value(p) {
        return Absent;
    }

    if is_at_string(p) {
        parse_string(p)
    } else {
        parse_url_value_raw(p)
    }
}

/// Determines if the current position of the parser is at a raw URL value.
#[inline]
pub(crate) fn is_at_url_value_raw(p: &mut CssParser) -> bool {
    p.at(CSS_URL_VALUE_RAW_LITERAL)
}

/// Parses a raw URL value from the current position of the CSS parser.
#[inline]
pub(crate) fn parse_url_value_raw(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_url_value_raw(p) {
        return Absent;
    }

    let m = p.start();
    p.expect(CSS_URL_VALUE_RAW_LITERAL);
    Present(m.complete(p, CSS_URL_VALUE_RAW))
}

struct UrlModifierListParseRecovery;

impl ParseRecovery for UrlModifierListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_URL_MODIFIER;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // url("//aa.com/img.svg" foo "bar" func(test));
        //                              ^    ^
        // "bar" is an invalid modifier |____| func is the recovery point
        //
        // url("//aa.com/img.svg" foo "bar"  );
        //                              ^    ^
        // "bar" is an invalid modifier |____| ')' is the recovery point
        p.at(T![')']) || is_at_url_modifier(p)
    }
}

struct UrlModifierList;

impl ParseNodeList for UrlModifierList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_URL_MODIFIER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_url_modifier(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &UrlModifierListParseRecovery, expected_url_modifier)
    }
}

/// This function determines if the current token is either an identifier or any function,
/// indicating a potential modifier for a URL in CSS.
#[inline]
pub(crate) fn is_at_url_modifier(p: &mut CssParser) -> bool {
    is_at_identifier(p) || is_at_function(p)
}

/// Parses a URL modifier, which can be either a simple function or a regular identifier.
#[inline]
pub(crate) fn parse_url_modifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_url_modifier(p) {
        return Absent;
    }

    if is_at_function(p) {
        parse_function(p)
    } else {
        parse_regular_identifier(p)
    }
}
