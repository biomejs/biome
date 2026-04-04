use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::declaration::{
    complete_declaration_with_semicolon, parse_declaration_important,
};
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::scss::{
    SCSS_NESTING_VALUE_END_SET, complete_empty_scss_expression, is_at_scss_interpolated_identifier,
    is_at_scss_interpolated_property, parse_scss_interpolated_identifier,
    parse_scss_optional_value_until,
};
use crate::syntax::{CssSyntaxFeatures, is_at_dashed_identifier, is_at_identifier, try_parse};
use biome_css_syntax::CssSyntaxKind::{
    CSS_DECLARATION, CSS_DECLARATION_WITH_SEMICOLON, CSS_GENERIC_PROPERTY, SCSS_NESTING_DECLARATION,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Marker, Parser, SyntaxFeature};

/// Parses a SCSS nested property declaration block, or falls back to a regular
/// declaration when no block follows.
///
/// Example:
/// ```scss
/// .button {
///   font: {
///     family: sans-serif;
///     size: 12px;
///   }
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/style-rules/declarations#nested-properties
#[inline]
pub(crate) fn parse_scss_nesting_declaration(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_nesting_declaration(p) {
        return Absent;
    }

    parse_scss_nesting_declaration_candidate(p).map_or(Absent, |(syntax, _)| syntax)
}

#[inline]
pub(crate) fn is_at_scss_nesting_declaration(p: &mut CssParser) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p)
        && !is_at_dashed_identifier(p)
        && !p.at(T![composes])
        && (is_at_scss_interpolated_property(p) || (is_at_identifier(p) && p.nth_at(1, T![:])))
}

struct ScssNestingMarkers {
    declaration: Marker,
    property: Marker,
}

/// Parses a SCSS nested-property/declaration candidate and returns both the
/// parsed syntax and whether the same prefix could still be interpreted as a
/// selector by the caller.
///
/// This keeps the real parsing shared between the committed and speculative
/// nesting entrypoints.
#[inline]
fn parse_scss_nesting_declaration_candidate(p: &mut CssParser) -> Option<(ParsedSyntax, bool)> {
    let (markers, could_be_selector) = parse_scss_nesting_declaration_prefix(p)?;
    let syntax = parse_scss_nesting_declaration_after_prefix(p, markers);

    Some((syntax, could_be_selector))
}

/// Parses the remainder of a SCSS nesting candidate after its `name:` prefix
/// has already been recognized.
///
/// This decides whether the candidate becomes a nested-property block or a
/// regular declaration once the value and following token are known.
#[inline]
fn parse_scss_nesting_declaration_after_prefix(
    p: &mut CssParser,
    markers: ScssNestingMarkers,
) -> ParsedSyntax {
    let missing_value =
        // Allow an empty value here because nested-property syntax may continue
        // directly into `{ ... }`, and the explicit missing-value diagnostic is
        // handled by the caller via `missing_value`.
        parse_scss_optional_value_until(p, SCSS_NESTING_VALUE_END_SET).is_absent();

    if p.at(T!['{']) {
        // A following `{` turns the parsed prefix into nested-property syntax.
        if missing_value {
            complete_empty_scss_expression(p);
        }
        complete_scss_nested_property_block(p, markers)
    } else {
        complete_scss_nesting_regular_declaration(p, markers, missing_value)
    }
}

/// Parses the shared `name:` prefix for SCSS nested properties and regular
/// declarations.
///
/// The prefix parser opens the marker wrappers needed to finish the construct
/// either as `ScssNestingDeclaration` or as `CssGenericProperty ->
/// CssDeclaration`, and returns `None` only when an interpolation-bearing start
/// is not actually followed by `:`.
#[inline]
fn parse_scss_nesting_declaration_prefix(p: &mut CssParser) -> Option<(ScssNestingMarkers, bool)> {
    let declaration = p.start();
    let property = p.start();

    // Guarded by `is_at_scss_nesting_declaration`, so a name parse cannot fail
    // here. The only real rejection point in this prefix parser is a missing `:`.
    parse_scss_interpolated_identifier(p).ok();

    if !p.at(T![:]) {
        declaration.abandon(p);
        property.abandon(p);
        return None;
    }

    p.expect(T![:]);

    let could_be_selector = !p.has_preceding_whitespace()
        && (is_at_identifier(p) || is_at_scss_interpolated_identifier(p) || p.at(T![:]));

    Some((
        ScssNestingMarkers {
            declaration,
            property,
        },
        could_be_selector,
    ))
}

#[inline]
fn complete_scss_nested_property_block(
    p: &mut CssParser,
    markers: ScssNestingMarkers,
) -> ParsedSyntax {
    markers.property.abandon(p);
    parse_declaration_or_rule_list_block(p);
    Present(markers.declaration.complete(p, SCSS_NESTING_DECLARATION))
}

#[inline]
fn complete_scss_nesting_regular_declaration(
    p: &mut CssParser,
    markers: ScssNestingMarkers,
    missing_value: bool,
) -> ParsedSyntax {
    if missing_value {
        complete_empty_scss_expression(p);
        p.error(expected_component_value(p, p.cur_range()));
    }

    // Otherwise, reinterpret the parsed property/value as a regular declaration.
    let property = markers.property.complete(p, CSS_GENERIC_PROPERTY);
    let declaration = property.precede(p);
    parse_declaration_important(p).ok();
    let declaration = declaration.complete(p, CSS_DECLARATION);

    markers.declaration.abandon(p);
    Present(complete_declaration_with_semicolon(p, declaration))
}

/// Speculatively parses a SCSS nested-property/declaration candidate and keeps
/// it only if the result is unambiguously declaration-like for the caller's
/// statement boundary.
///
/// Callers use this to prefer declaration parsing for Sass-compatible forms
/// such as `font: bold;` or `font: { ... }`, while still rewinding for selector
/// syntax like `font:bold { ... }`.
///
/// Example:
/// ```scss
/// .button {
///   font:bold { color: red; }
///   font: bold;
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/style-rules/declarations#nested-properties
#[inline]
pub(crate) fn try_parse_scss_nesting_declaration(
    p: &mut CssParser,
    end_kind: CssSyntaxKind,
) -> Result<ParsedSyntax, ()> {
    try_parse(p, |p| {
        if !is_at_scss_nesting_declaration(p) {
            return Err(());
        }

        let Some((syntax, could_be_selector)) = parse_scss_nesting_declaration_candidate(p) else {
            return Err(());
        };

        match syntax.kind(p) {
            Some(SCSS_NESTING_DECLARATION) if !could_be_selector => Ok(syntax),
            Some(CSS_DECLARATION_WITH_SEMICOLON)
                if matches!(p.last(), Some(T![;])) || p.at(end_kind) =>
            {
                Ok(syntax)
            }
            _ => Err(()),
        }
    })
}
