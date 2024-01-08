use crate::parser::CssParser;
use crate::syntax::at_rule::parse_error::expected_any_namespace_url;
use crate::syntax::value::url::{is_at_url_function, parse_url_function};
use crate::syntax::{is_at_string, parse_regular_identifier, parse_string};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;
use biome_rowan::SyntaxKind;

/// Checks if the current token in the parser is a `@namespace` at-rule.
///
/// This function verifies if the current token matches the `@namespace` rule.
#[inline]
pub(crate) fn is_at_namespace_at_rule(p: &mut CssParser) -> bool {
    p.at(T![namespace])
}

/// Parses a `@namespace` at-rule in a CSS stylesheet.
/// For specification details, see [CSS Namespaces Module](https://www.w3.org/TR/css-namespaces-3/).
/// # Examples
/// Basic usage in CSS:
/// ```css
/// @namespace url(http://www.w3.org/1999/xhtml);
/// @namespace svg url(http://www.w3.org/2000/svg);
/// ```
/// This function identifies and parses these `@namespace` rules within CSS stylesheets.
#[inline]
pub(crate) fn parse_namespace_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_namespace_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![namespace]);

    if !is_at_namespace_url(p) {
        // If we aren't at a namespace URL, then we are at a prefix,
        // and we need to try to parse it.
        // The problem is that both `url` and `src` are valid identifiers.
        parse_regular_identifier(p).ok();
    }

    let kind = match parse_namespace_url(p).or_recover(
        p,
        &NamespaceUrlParseRecovery,
        expected_any_namespace_url,
    ) {
        Ok(m) => {
            if m.kind(p).is_bogus() {
                CSS_BOGUS_AT_RULE
            } else {
                CSS_NAMESPACE_AT_RULE
            }
        }
        Err(_) => CSS_BOGUS_AT_RULE,
    };

    p.expect(T![;]);

    Present(m.complete(p, kind))
}

struct NamespaceUrlParseRecovery;

impl ParseRecovery for NamespaceUrlParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // @namespace  2131 ; <--- recovery point
        // invalid url ^^^^
        p.at(T![;]) || p.has_nth_preceding_line_break(1)
    }
}

/// Checks if the current token in the parser is either a URL function or a string, indicating a namespace URL.
///
/// This function determines whether the current token can represent a namespace URL in a CSS `@namespace` rule.
/// It checks for either a URL function or a string token.
#[inline]
pub(crate) fn is_at_namespace_url(p: &mut CssParser) -> bool {
    is_at_url_function(p) || is_at_string(p)
}

/// Parses the URL of a namespace in a `@namespace` rule.
#[inline]
pub(crate) fn parse_namespace_url(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_namespace_url(p) {
        return Absent;
    }

    if is_at_url_function(p) {
        parse_url_function(p)
    } else {
        parse_string(p)
    }
}
