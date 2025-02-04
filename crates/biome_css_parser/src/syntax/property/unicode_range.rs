use crate::lexer::{CssLexContext, CssReLexContext};
use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::{
    self, CSS_BOGUS_UNICODE_RANGE_VALUE, CSS_DIMENSION_VALUE, CSS_NUMBER_LITERAL,
    CSS_UNICODE_CODEPOINT, CSS_UNICODE_CODEPOINT_LITERAL, CSS_UNICODE_RANGE,
    CSS_UNICODE_RANGE_INTERVAL, CSS_UNICODE_RANGE_WILDCARD, CSS_UNICODE_RANGE_WILDCARD_LITERAL,
};
use biome_css_syntax::{TextRange, T};
use biome_parser::diagnostic::{expected_any, expected_node, ParseDiagnostic};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

const UNICODE: TokenSet<CssSyntaxKind> = token_set![
    // u+;
    T![+],
    // u+000;
    CSS_NUMBER_LITERAL,
    // u+00ff?;
    CSS_DIMENSION_VALUE,
];

/// Checks if the parser is positioned to potentially start parsing a Unicode range in CSS, identified by "U" or "u".
pub(crate) fn is_at_unicode_range(p: &mut CssParser) -> bool {
    matches!(p.cur_text(), "U" | "u") && p.nth_at_ts(1, UNICODE)
}

/// Parses a Unicode range in CSS starting from "U+" or "u+".
///
/// This function first performs lexical analysis to confirm the presence of a Unicode range indicator
/// before proceeding to parse the range or fallback to a bogus value.
///
/// Specification:
/// - [Unicode Range Descriptor](https://drafts.csswg.org/css-fonts/#unicode-range-desc)
/// - [CSS Syntax Level 3](https://www.w3.org/TR/css-syntax-3/#typedef-urange)
///
/// # Examples
///
/// Basic usage in CSS:
///
/// ```css
/// @font-face {
///     unicode-range: U+000-49F, U+2000-27FF, U+2900-2BFF, U+1D400-1D7FF, U+ff??;
/// }
/// ```
///
/// This function is integral in parsing and interpreting Unicode range descriptors in CSS.
pub(crate) fn parse_unicode_range(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_unicode_range(p) {
        return Absent;
    }

    // Perform lexical analysis to confirm the presence of a Unicode range indicator.
    // This is necessary to ensure that the parser is positioned to parse a Unicode range.
    let kind = p.re_lex(CssReLexContext::UnicodeRange);

    // If the parser is not positioned to parse a Unicode range, return an absent value.
    if kind != T!["U+"] {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T!["U+"], CssLexContext::UnicodeRange);

    // Checks if the parser is positioned to parse a Unicode range wildcard.
    // A wildcard cannot be combined with a range interval. For example, `U+????-U+????` is invalid.
    if is_at_unicode_range_wildcard(p) {
        parse_unicode_range_wildcard(p).ok();

        return Present(m.complete(p, CSS_UNICODE_RANGE));
    }

    let codepoint = parse_unicode_codepoint(p).or_add_diagnostic(p, expected_codepoint_value);

    let Some(codepoint) = codepoint else {
        return Present(m.complete(p, CSS_BOGUS_UNICODE_RANGE_VALUE));
    };

    // Checks if the parser is positioned to parse a Unicode range interval.
    // A range interval is identified by a hyphen (`-`) followed by another Unicode codepoint.
    if p.at(T![-]) {
        let range = codepoint.precede(p);
        p.bump_with_context(T![-], CssLexContext::UnicodeRange);

        // Abandon the range if the parser is not positioned to parse a Unicode codepoint.

        if parse_unicode_codepoint(p).is_absent() {
            // If the parser is positioned to parse a Unicode range wildcard add a diagnostic.
            if parse_unicode_range_wildcard(p)
                .add_diagnostic_if_present(p, wildcard_not_allowed)
                .is_none()
            {
                // If the parser is not positioned to parse a Unicode codepoint, add a diagnostic.
                p.error(expected_codepoint(p, p.cur_range()));
            }
            range.abandon(p);
            return Present(m.complete(p, CSS_BOGUS_UNICODE_RANGE_VALUE));
        }

        range.complete(p, CSS_UNICODE_RANGE_INTERVAL);
    }

    Present(m.complete(p, CSS_UNICODE_RANGE))
}

/// Checks if the parser is positioned at a Unicode codepoint.
fn is_at_unicode_codepoint(p: &mut CssParser) -> bool {
    p.at(CSS_UNICODE_CODEPOINT_LITERAL)
}

/// Parses a Unicode codepoint from the current position in the CSS parser.
/// # Examples
///
/// Basic usage in CSS:
///
/// ```css
/// .class {
///     unicode-range: U+000-49F, U+2000-27FF, U+2900-2BFF, U+1D400-1D7FF;
/// }
/// ```
fn parse_unicode_codepoint(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_unicode_codepoint(p) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(CSS_UNICODE_CODEPOINT_LITERAL, CssLexContext::UnicodeRange);

    Present(m.complete(p, CSS_UNICODE_CODEPOINT))
}

/// Checks if the parser is positioned at a Unicode range wildcard in CSS.
fn is_at_unicode_range_wildcard(p: &mut CssParser) -> bool {
    p.at(CSS_UNICODE_RANGE_WILDCARD_LITERAL)
}

/// Parses a Unicode range wildcard from the current position in the CSS parser.
/// # Examples
///
/// Basic usage in CSS:
///
/// ```css
/// .class {
///     unicode-range: U+ff??;
/// }
/// ```
///
fn parse_unicode_range_wildcard(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_unicode_range_wildcard(p) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(
        CSS_UNICODE_RANGE_WILDCARD_LITERAL,
        CssLexContext::UnicodeRange,
    );

    Present(m.complete(p, CSS_UNICODE_RANGE_WILDCARD))
}

/// Provides a diagnostic for invalid wildcard placement within a Unicode range.
/// Wildcards (`U+????`) are only valid at the beginning of a Unicode range descriptor.
/// When specifying a range interval (`U+XXXX-YYYY`), wildcards cannot be used in the second position.
pub(crate) fn wildcard_not_allowed(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Codepoint range wildcard is not valid here.", range)
        .with_hint(
            "Wildcards (`U+????`) are only allowed at the beginning of a Unicode range descriptor. \
             When specifying a range interval (`U+XXXX-YYYY`), wildcards cannot be used in the second position."
        )
}

/// Generates a parse diagnostic for an expected "codepoint" error message at the given range.
pub(crate) fn expected_codepoint(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("codepoint", range, p)
        .with_hint("Expected a valid Unicode codepoint (e.g., U+1234).")
}

/// Generates a parse diagnostic for an expected "codepoint or wildcard" error message at the given range.
pub(crate) fn expected_codepoint_value(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["codepoint", "codepoint range wildcard"], range, p)
        .with_hint("Expected a valid Unicode codepoint (e.g., U+1234) or a codepoint range wildcard (e.g., U+????).")
}
