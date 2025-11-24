use biome_css_syntax::CssSyntaxKind;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::TokenSet;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::parse_recovery::RecoveryResult;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::token_set;
use biome_parser::{Parser, prelude::ParsedSyntax};

use crate::parser::CssParser;
use crate::syntax::is_at_identifier;
use crate::syntax::is_at_string;
use crate::syntax::parse_regular_identifier;
use crate::syntax::parse_string;
use crate::syntax::value::parse_error::expected_syntax_component;

const SYNTAX_MULTIPLIER_SET: TokenSet<CssSyntaxKind> = token_set![T![#], T![+]];

const KNOWN_SYNTAX_TYPE_NAMES: [&str; 14] = [
    "angle",
    "color",
    "custom-ident",
    "image",
    "integer",
    "length",
    "length-percentage",
    "number",
    "percentage",
    "resolution",
    "string",
    "time",
    "url",
    "transform-function",
];

const SYNTAX_KIND_WITHOUT_MULTIPLIER: [&str; 1] = ["transform-list"];

#[inline]
pub(crate) fn is_at_type_function(p: &mut CssParser) -> bool {
    p.at(T![type]) && p.nth_at(1, T!['('])
}

/// Parses a type function from the current position of the CSS parser.
/// For more detailed information on the CSS type function syntax, refer to the [CSS Values and
/// Units Module](https://drafts.csswg.org/css-values-5/#typedef-syntax)
///
/// # Type Function Syntax Examples
///
/// - Single value:
///   ``` css
///   type(<color>)
///   type(auto)
///   ```
/// - "|" combinator for multiple types:
///   ``` css
///   type(<length> | <percentage>)
///   ```
/// - Comma-separated list of values
///   ```css
///   type(<color>+)
///   ```
/// - Comma-separated list of values
///   ```css
///   type(<length>#)
///   ```
/// - Multiple keywords
///   ```css
///   type(red | blue | green)
///   ```
/// - Combination of data type and keyword
///   ```css
///   type(<percentage> | auto)
///   ```
/// - Universal syntax value
///   ```css
///   type(*)
///   ```
///
/// # Grammar
///
/// ``` txt
/// type( <syntax> )
///
/// <syntax> = '*' | <syntax-component> [ <syntax-combinator> <syntax-component> ]* | <syntax-string>
/// <syntax-component> = <syntax-single-component> <syntax-multiplier>?
///                    | '<' transform-list '>'
/// <syntax-single-component> = '<' <syntax-type-name> '>' | <ident>
/// <syntax-type-name> = angle | color | custom-ident | image | integer
///                    | length | length-percentage | number
///                    | percentage | resolution | string | time
///                    | url | transform-function
/// <syntax-combinator> = '|'
/// <syntax-multiplier> = [ '#' | '+' ]
///
/// <syntax-string> = <string>
#[inline]
pub(crate) fn parse_type_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_type_function(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![type]);
    p.bump(T!['(']);
    parse_any_syntax(p).ok();
    p.expect(T![')']);

    Present(m.complete(p, CSS_TYPE_FUNCTION))
}

#[inline]
fn parse_any_syntax(p: &mut CssParser) -> ParsedSyntax {
    if p.at(T![*]) {
        let m = p.start();
        p.bump(T![*]);
        return Present(m.complete(p, CSS_WILDCARD));
    }

    if is_at_string(p) {
        return parse_string(p);
    }

    if is_at_syntax_single_component(p) {
        return Present(SyntaxComponentList.parse_list(p));
    }

    Absent
}

#[inline]
fn is_at_syntax_single_component(p: &mut CssParser) -> bool {
    is_at_syntax_type(p) || is_at_identifier(p)
}

#[inline]
fn parse_any_syntax_component(p: &mut CssParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();

    // handle <transform-list> edge case
    if is_at_syntax_type(p) {
        let m = p.start();

        p.bump(T![<]);

        if SYNTAX_KIND_WITHOUT_MULTIPLIER.contains(&p.cur_text()) {
            p.bump_remap(T![ident]);
            p.bump(T![>]);

            return Present(m.complete(p, CSS_SYNTAX_COMPONENT_WITHOUT_MULTIPLIER));
        }

        // no <transform-list> found, fallback to parsing CssSyntaxComponent
        m.abandon(p);
        p.rewind(checkpoint);
    }

    let m = p.start();

    if parse_any_syntax_single_component(p).ok().is_none() {
        m.abandon(p);
        return Absent;
    }

    parse_syntax_multiplier(p).ok();

    Present(m.complete(p, CSS_SYNTAX_COMPONENT))
}

#[inline]
fn parse_any_syntax_single_component(p: &mut CssParser) -> ParsedSyntax {
    if is_at_syntax_type(p) {
        return parse_syntax_type(p);
    }

    if is_at_identifier(p) {
        return parse_regular_identifier(p);
    }

    Absent
}

#[inline]
fn is_at_syntax_multiplier(p: &mut CssParser) -> bool {
    p.at_ts(SYNTAX_MULTIPLIER_SET)
}

#[inline]
fn parse_syntax_multiplier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_syntax_multiplier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(SYNTAX_MULTIPLIER_SET);
    Present(m.complete(p, CSS_SYNTAX_MULTIPLIER))
}

#[inline]
fn is_at_syntax_type(p: &mut CssParser) -> bool {
    p.at(T![<])
}

#[inline]
fn parse_syntax_type(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_syntax_type(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![<]);
    parse_any_syntax_type_name(p).ok();
    p.expect(T![>]);

    Present(m.complete(p, CSS_SYNTAX_TYPE))
}

#[inline]
fn is_at_any_syntax_type_name(p: &mut CssParser) -> bool {
    p.at(T![ident]) || p.cur().is_keyword()
}

#[inline]
fn parse_any_syntax_type_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_syntax_type_name(p) {
        return Absent;
    }

    let m = p.start();

    let kind = if is_at_valid_syntax_type_name(p) {
        CSS_REGULAR_SYNTAX_TYPE_NAME
    } else {
        CSS_UNKNOWN_SYNTAX_TYPE_NAME
    };

    p.bump_remap(T![ident]);

    Present(m.complete(p, kind))
}

struct SyntaxTypeListParseRecovery;

impl ParseRecovery for SyntaxTypeListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_SYNTAX_SINGLE_COMPONENT;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![,]) || p.at(T![')']) || p.at(T![;]) || p.has_preceding_line_break()
    }
}

struct SyntaxComponentList;

impl ParseSeparatedList for SyntaxComponentList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_SYNTAX_COMPONENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_any_syntax_component(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &SyntaxTypeListParseRecovery, expected_syntax_component)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![|]
    }

    fn allow_empty(&self) -> bool {
        false
    }
}

#[inline]
fn is_at_valid_syntax_type_name(p: &mut CssParser) -> bool {
    KNOWN_SYNTAX_TYPE_NAMES.contains(&p.cur_text())
}
