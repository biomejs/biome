use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::scss::parse_scss_expression_allow_empty_value_until;
use crate::syntax::{
    CssSyntaxFeatures, is_at_dashed_identifier, is_at_identifier, parse_regular_identifier,
};
use biome_css_syntax::CssSyntaxKind::{
    CSS_DECLARATION, CSS_DECLARATION_IMPORTANT, CSS_DECLARATION_WITH_SEMICOLON,
    CSS_GENERIC_PROPERTY, EOF, SCSS_NESTING_DECLARATION,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Parser, SyntaxFeature, TokenSet, token_set};

const SCSS_NESTING_VALUE_END_SET: TokenSet<CssSyntaxKind> =
    token_set![T!['{'], T![;], T!['}'], T![!], EOF];

/// Detects nested property syntax (`prop: { ... }`) while excluding custom properties
/// and CSS Modules declarations that must remain regular properties.
///
/// Example:
/// ```scss
/// font: { size: 12px; }
/// ```
///
/// Docs: https://sass-lang.com/documentation/style-rules/declarations#nested-properties
#[inline]
pub(crate) fn is_at_scss_nesting_declaration(p: &mut CssParser) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p)
        && is_at_identifier(p)
        && p.nth_at(1, T![:])
        && !is_at_dashed_identifier(p)
        && !p.at(T![composes])
}

/// Parses a SCSS nested property declaration block, or falls back to a regular declaration
/// when no block follows.
///
/// Example:
/// ```scss
/// font: {
///   family: sans-serif;
///   size: 12px;
/// }
/// ```
///
/// Specification: https://sass-lang.com/documentation/style-rules/declarations#nested-properties
#[inline]
pub(crate) fn parse_scss_nesting_declaration(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_nesting_declaration(p) {
        return Absent;
    }

    let m = p.start();
    let property = p.start();
    parse_regular_identifier(p).ok();
    p.expect(T![:]);
    let missing_value = p.at(T![;]) || p.at(T!['}']) || p.at(EOF) || p.at(T![!]);
    parse_scss_expression_allow_empty_value_until(p, SCSS_NESTING_VALUE_END_SET).ok();

    if p.at(T!['{']) {
        // Upgrade to a nested-property block only if `{` follows the value.
        property.abandon(p);
        parse_declaration_or_rule_list_block(p);
        return Present(m.complete(p, SCSS_NESTING_DECLARATION));
    }

    if missing_value {
        p.error(expected_component_value(p, p.cur_range()));
    }

    // Otherwise, reinterpret the parsed property/value as a regular declaration.
    let property = property.complete(p, CSS_GENERIC_PROPERTY);
    let declaration = property.precede(p);
    parse_declaration_important(p).ok();
    let declaration = declaration.complete(p, CSS_DECLARATION);

    m.abandon(p);
    Present(complete_declaration_with_semicolon(p, declaration))
}

#[inline]
fn complete_declaration_with_semicolon(
    p: &mut CssParser,
    declaration: CompletedMarker,
) -> CompletedMarker {
    let m = declaration.precede(p);

    if !p.at(T!['}']) {
        if p.nth_at(1, T!['}']) {
            p.eat(T![;]);
        } else {
            p.expect(T![;]);
        }
    }

    m.complete(p, CSS_DECLARATION_WITH_SEMICOLON)
}

#[inline]
fn parse_declaration_important(p: &mut CssParser) -> ParsedSyntax {
    if !(p.at(T![!]) && p.nth_at(1, T![important])) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![!]);
    p.bump(T![important]);
    Present(m.complete(p, CSS_DECLARATION_IMPORTANT))
}
