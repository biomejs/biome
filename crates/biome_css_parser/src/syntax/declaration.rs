use crate::parser::CssParser;
use crate::syntax::CssSyntaxFeatures;
use crate::syntax::parse_error::{expected_declaration_item, scss_only_syntax_error};
use crate::syntax::property::{
    is_at_any_property, parse_any_property, parse_any_property_with_value_end_set,
};
use crate::syntax::scss::{
    is_at_scss_declaration, is_at_scss_interpolated_property, is_at_scss_nesting_declaration,
    parse_scss_declaration, parse_scss_interpolated_property_declaration,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Parser, SyntaxFeature, TokenSet, token_set};

pub(crate) struct DeclarationList;

impl ParseNodeList for DeclarationList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_DECLARATION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if is_at_scss_nesting_declaration(p) || is_at_scss_interpolated_property(p) {
            parse_scss_interpolated_property_declaration(p)
        } else {
            parse_any_declaration_with_semicolon(p)
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, token_set!(T!['}'])),
            expected_declaration_item,
        )
    }
}

pub(crate) fn is_at_declaration(p: &mut CssParser) -> bool {
    is_at_any_property(p)
}

#[inline]
pub(crate) fn parse_declaration(p: &mut CssParser) -> ParsedSyntax {
    parse_declaration_with(p, parse_any_property)
}

/// Parses an interpolation-bearing SCSS property in declaration-only contexts.
///
/// Nested-property parsing gets the first chance so `name: { ... }` stays intact.
/// If no nested-property form is found, this falls back to a regular declaration.
#[inline]
pub(crate) fn parse_declaration_with_value_end_set(
    p: &mut CssParser,
    value_end_set: TokenSet<CssSyntaxKind>,
    recovery_end_set: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    parse_declaration_with(p, |p| {
        parse_any_property_with_value_end_set(p, value_end_set, recovery_end_set)
    })
}

#[inline]
fn parse_declaration_with<F>(p: &mut CssParser, parse_property: F) -> ParsedSyntax
where
    F: FnOnce(&mut CssParser) -> ParsedSyntax,
{
    let m = p.start();

    if parse_property(p).is_absent() {
        m.abandon(p);
        return Absent;
    }
    parse_declaration_important(p).ok();

    Present(m.complete(p, CSS_DECLARATION))
}

#[inline]
pub(crate) fn is_at_any_declaration_with_semicolon(p: &mut CssParser) -> bool {
    is_at_any_declaration(p) || is_at_empty_declaration(p)
}

#[inline]
pub(crate) fn parse_any_declaration_with_semicolon(p: &mut CssParser) -> ParsedSyntax {
    if is_at_empty_declaration(p) {
        parse_empty_declaration(p)
    } else if is_at_scss_declaration(p) {
        CssSyntaxFeatures::Scss.parse_exclusive_syntax(p, parse_scss_declaration, |p, marker| {
            scss_only_syntax_error(p, "SCSS variable declarations", marker.range(p))
        })
    } else if is_at_any_declaration_with_semicolon(p) {
        parse_declaration_with_semicolon(p)
    } else {
        Absent
    }
}

#[inline]
pub(crate) fn is_at_any_declaration(p: &mut CssParser) -> bool {
    is_at_declaration(p) || is_at_scss_declaration(p)
}

/// Parses a CSS declaration that may optionally end with a semicolon.
///
/// This function attempts to parse a single CSS declaration from the current position
/// of the parser. It handles the optional semicolon (';') at the end of the declaration,
/// adhering to CSS syntax rules where the semicolon is mandatory for all declarations
/// except the last one in a block. In the case of the last declaration before a closing
/// brace ('}'), the semicolon is optional. If the semicolon is omitted for declarations
/// that are not at the end, the parser will raise an error.
#[inline]
pub(crate) fn parse_declaration_with_semicolon(p: &mut CssParser) -> ParsedSyntax {
    parse_declaration(p).map(|declaration| complete_declaration_with_semicolon(p, declaration))
}

#[inline]
pub(crate) fn complete_declaration_with_semicolon(
    p: &mut CssParser,
    declaration: CompletedMarker,
) -> CompletedMarker {
    let declaration_with_semicolon = declaration.precede(p);

    parse_optional_declaration_semicolon(p);

    declaration_with_semicolon.complete(p, CSS_DECLARATION_WITH_SEMICOLON)
}

#[inline]
pub(crate) fn parse_optional_declaration_semicolon(p: &mut CssParser) {
    // If the next token is a closing brace ('}'), the semicolon is optional.
    // Otherwise, a semicolon is expected and the parser will enforce its presence.
    // div { color: red; }
    // div { color: red }
    if !p.at(T!['}']) {
        if p.nth_at(1, T!['}']) {
            p.eat(T![;]);
        } else {
            p.expect(T![;]);
        }
    }
}

#[inline]
pub(crate) fn is_at_empty_declaration(p: &mut CssParser) -> bool {
    p.at(T![;])
}

#[inline]
pub(crate) fn parse_empty_declaration(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_empty_declaration(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![;]);
    Present(m.complete(p, CSS_EMPTY_DECLARATION))
}

#[inline]
pub(crate) fn is_at_declaration_important(p: &mut CssParser) -> bool {
    p.at(T![!]) && p.nth_at(1, T![important])
}

#[inline]
pub(crate) fn parse_declaration_important(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_declaration_important(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![!]);
    p.bump(T![important]);
    Present(m.complete(p, CSS_DECLARATION_IMPORTANT))
}
