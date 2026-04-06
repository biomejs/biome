pub(crate) mod color;
pub(crate) mod unicode_range;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::css_modules::{
    composes_not_allowed, expected_classes_list, expected_composes_import_source,
};
use crate::syntax::parse_error::{
    expected_component_value, expected_identifier, tailwind_disabled,
};
use crate::syntax::scss::{
    is_at_scss_interpolated_property, parse_required_scss_value_until,
    parse_scss_interpolated_property_name,
};
use crate::syntax::{
    CssSyntaxFeatures, is_at_any_value, is_at_dashed_identifier, is_at_identifier, is_at_string,
    is_nth_at_identifier, parse_any_value, parse_custom_identifier_with_keywords,
    parse_dashed_identifier, parse_regular_identifier, parse_string,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{
    ParseRecovery, ParseRecoveryTokenSet, RecoveryError, RecoveryResult,
};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, SyntaxFeature, TokenSet, token_set};

#[inline]
pub(crate) fn is_at_any_property(p: &mut CssParser) -> bool {
    is_at_generic_property(p)
}

#[inline]
pub(crate) fn parse_any_property(p: &mut CssParser) -> ParsedSyntax {
    parse_any_property_with_value_end_set(
        p,
        END_OF_PROPERTY_VALUE_COMPONENT_LIST_TOKEN_SET,
        END_OF_PROPERTY_VALUE_TOKEN_SET,
    )
}

#[inline]
pub(crate) fn parse_any_property_with_value_end_set(
    p: &mut CssParser,
    value_end_set: TokenSet<CssSyntaxKind>,
    recovery_end_set: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    if !is_at_any_property(p) {
        return Absent;
    }

    match p.cur() {
        T![composes] => {
            parse_composes_property_with_value_end_set(p, value_end_set, recovery_end_set)
        }
        _ => parse_generic_property_with_value_end_set(p, value_end_set, recovery_end_set),
    }
}

/// Checks if the current parser position is at a `composes` property.
///
/// This function determines if the parser is currently positioned at a `composes` property,
/// which is indicated by the presence of the `composes` keyword followed by a colon (`:`).
#[inline]
fn is_at_composes_property(p: &mut CssParser) -> bool {
    p.at(T![composes]) && p.nth_at(1, T![:])
}

/// Parses a `composes` property in CSS Modules.
///
/// This function parses a `composes` property, which is used in CSS Modules to compose classes from other modules.
/// If the current parser position is not at a `composes` property, it returns `Absent`. If CSS Modules are disabled,
/// it generates a diagnostic error and falls back to parsing a generic property.
///
/// Basic usage in CSS:
/// ```css
/// .button {
///     composes: baseButton alertButton from 'base.css';
/// }
///
/// .alert {
///     composes: alertText;
/// }
/// ```
fn parse_composes_property_with_value_end_set(
    p: &mut CssParser,
    value_end_set: TokenSet<CssSyntaxKind>,
    recovery_end_set: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    if !is_at_composes_property(p) {
        return Absent;
    }

    if CssSyntaxFeatures::CssModules.is_unsupported(p) {
        // `composes` is not a standard CSS feature.
        // Provide a hint on how to enable parsing of the `composes` declaration.
        p.error(composes_not_allowed(p, p.cur_range()));

        // Fallback to a generic property
        return parse_generic_property_with_value_end_set(p, value_end_set, recovery_end_set);
    }

    let m = p.start();
    // remap the `composes` keyword to a regular identifier
    parse_regular_identifier(p).ok();
    p.bump(T![:]);

    {
        let m = p.start();

        let class_list_end_set = value_end_set.union(token_set!(T![from]));
        let classes = ComposesClassList::new(class_list_end_set, recovery_end_set).parse_list(p);

        // If the list of classes is empty, generate a diagnostic error.
        if classes.range(p).is_empty() {
            p.error(expected_classes_list(p, p.cur_range()));
        }

        if p.at(T![from]) {
            let m = p.start();
            p.bump(T![from]);

            if is_at_identifier(p) {
                parse_regular_identifier(p).ok();
            } else if is_at_string(p) {
                parse_string(p).ok();
            } else {
                p.error(expected_composes_import_source(p, p.cur_range()));
            }

            m.complete(p, CSS_COMPOSES_IMPORT_SPECIFIER);
        }
        m.complete(p, CSS_COMPOSES_PROPERTY_VALUE);
    }

    Present(m.complete(p, CSS_COMPOSES_PROPERTY))
}

/// A struct representing a list of classes in a `composes` property.
struct ComposesClassList {
    end_set: TokenSet<CssSyntaxKind>,
    recovery_end_set: TokenSet<CssSyntaxKind>,
}

impl ComposesClassList {
    fn new(end_set: TokenSet<CssSyntaxKind>, recovery_end_set: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            end_set,
            recovery_end_set,
        }
    }
}

impl ParseNodeList for ComposesClassList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_COMPOSES_CLASS_LIST;

    /// Parses an individual element in the `composes` class list.
    ///
    /// This function parses an identifier as a custom identifier because it is a selector,
    /// which is case-sensitive. For more information, see:
    /// https://github.com/css-modules/css-modules/blob/master/docs/composition.md
    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_custom_identifier_with_keywords(p, CssLexContext::Regular, true)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(self.end_set)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ComposesClassListParseRecovery::new(self.end_set, self.recovery_end_set),
            expected_identifier,
        )
    }
}

struct ComposesClassListParseRecovery {
    end_set: TokenSet<CssSyntaxKind>,
    recovery_end_set: TokenSet<CssSyntaxKind>,
}

impl ComposesClassListParseRecovery {
    fn new(end_set: TokenSet<CssSyntaxKind>, recovery_end_set: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            end_set,
            recovery_end_set,
        }
    }
}

impl ParseRecovery for ComposesClassListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // If the next token is the end of the list or the next element, we're at a recovery point.
        p.at_ts(self.end_set) || p.at_ts(self.recovery_end_set) || is_at_identifier(p)
    }
}

/// Detects the start of a generic property name.
///
/// This covers both direct property names and SCSS interpolation-bearing names:
///
/// ```scss
/// color: red;
/// --color-*: initial;
/// #{$name}: 1px;
/// margin-#{$side}: 1px;
/// ```
#[inline]
pub(crate) fn is_at_generic_property(p: &mut CssParser) -> bool {
    is_at_direct_generic_property(p) || is_at_scss_interpolated_property(p)
}

/// Detects the direct, non-interpolated property-name forms handled by the
/// generic property parser.
///
/// This includes plain identifiers followed by `:` as well as the Tailwind
/// `--*:` and `--name-*:` theme-reference forms.
#[inline]
fn is_at_direct_generic_property(p: &mut CssParser) -> bool {
    is_nth_at_direct_generic_property(p, 0)
}

#[inline]
fn is_at_tailwind_theme_reference_property(p: &mut CssParser) -> bool {
    is_nth_at_tailwind_theme_reference_property(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_direct_generic_property(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_identifier(p, n)
        && (p.nth_at(n + 1, T![:]) || is_nth_at_tailwind_theme_reference_property(p, n))
}

#[inline]
fn is_nth_at_tailwind_theme_reference_property(p: &mut CssParser, n: usize) -> bool {
    // handle --*:
    (p.nth_at(n + 1, T![*]) && p.nth_at(n + 2, T![:]))
        // handle --color-*:
        || (p.nth_at(n + 1, T![-]) && p.nth_at(n + 2, T![*]) && p.nth_at(n + 3, T![:]))
}

#[inline]
pub(crate) fn parse_generic_property_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_dashed_identifier(p) && is_at_tailwind_theme_reference_property(p) {
        let Present(ident) = parse_dashed_identifier(p) else {
            return Absent;
        };

        return if p.at_ts(token_set![T![-], T![*]]) {
            CssSyntaxFeatures::Tailwind.parse_exclusive_syntax(
                p,
                |p| {
                    let m = ident.precede(p);
                    if p.at(T![-]) {
                        p.expect(T![-]);
                    }
                    p.expect(T![*]);
                    Present(m.complete(p, TW_VALUE_THEME_REFERENCE))
                },
                |p, m| tailwind_disabled(p, m.range(p)),
            )
        } else {
            Present(ident)
        };
    }

    if CssSyntaxFeatures::Scss.is_supported(p) {
        return parse_scss_interpolated_property_name(p).or_else(|| parse_plain_property_name(p));
    }

    parse_plain_property_name(p)
}

#[inline]
fn parse_plain_property_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_dashed_identifier(p) {
        parse_dashed_identifier(p)
    } else {
        parse_regular_identifier(p)
    }
}

/// Parses a generic property/value pair, using SCSS expression parsing when enabled
/// so lists, maps, and `!important` terminate correctly.
///
/// Example:
/// ```scss
/// margin: 1px 2px, 3px 4px !important;
/// ```
///
/// Docs: https://sass-lang.com/documentation/style-rules/declarations
#[inline]
fn parse_generic_property_with_value_end_set(
    p: &mut CssParser,
    value_end_set: TokenSet<CssSyntaxKind>,
    recovery_end_set: TokenSet<CssSyntaxKind>,
) -> ParsedSyntax {
    let m = p.start();
    if parse_generic_property_name(p).is_absent() {
        m.abandon(p);
        return Absent;
    }

    p.expect(T![:]);
    parse_property_value_with_end_set(p, value_end_set, recovery_end_set);

    Present(m.complete(p, CSS_GENERIC_PROPERTY))
}

#[inline]
pub(crate) fn parse_property_value_with_end_set(
    p: &mut CssParser,
    value_end_set: TokenSet<CssSyntaxKind>,
    recovery_end_set: TokenSet<CssSyntaxKind>,
) {
    if CssSyntaxFeatures::Scss.is_supported(p) {
        parse_required_scss_value_until(p, value_end_set);
    } else {
        GenericComponentValueList::new(value_end_set, recovery_end_set).parse_list(p);
    }
}

pub(crate) const END_OF_PROPERTY_VALUE_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set!(T!['}'], T![;]);
// Include `)` to recover malformed values without turning the whole declaration
// into a bogus property, and `!` so `!important` isn't parsed as a value token.
pub(crate) const END_OF_PROPERTY_VALUE_COMPONENT_LIST_TOKEN_SET: TokenSet<CssSyntaxKind> =
    END_OF_PROPERTY_VALUE_TOKEN_SET.union(token_set!(T![')'], T![!]));

pub(crate) struct GenericComponentValueList {
    end_set: TokenSet<CssSyntaxKind>,
    recovery_set: TokenSet<CssSyntaxKind>,
    boundary: Option<fn(&mut CssParser) -> bool>,
}

impl GenericComponentValueList {
    pub(crate) fn new(
        end_set: TokenSet<CssSyntaxKind>,
        recovery_set: TokenSet<CssSyntaxKind>,
    ) -> Self {
        Self {
            end_set,
            recovery_set,
            boundary: None,
        }
    }

    pub(crate) fn with_boundary(mut self, boundary: fn(&mut CssParser) -> bool) -> Self {
        self.boundary = Some(boundary);
        self
    }

    #[inline]
    fn at_boundary(&self, p: &mut CssParser) -> bool {
        self.boundary.is_some_and(|boundary| boundary(p))
    }
}

impl Default for GenericComponentValueList {
    fn default() -> Self {
        Self::new(
            END_OF_PROPERTY_VALUE_COMPONENT_LIST_TOKEN_SET,
            END_OF_PROPERTY_VALUE_TOKEN_SET,
        )
    }
}

impl ParseNodeList for GenericComponentValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_GENERIC_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_generic_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(self.end_set) || self.at_boundary(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        if self.at_boundary(p) {
            return Err(RecoveryError::AlreadyRecovered);
        }

        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_PROPERTY_VALUE, self.recovery_set)
                .enable_recovery_on_line_break(),
            expected_component_value,
        )
    }
}

#[inline]
pub(crate) fn is_at_generic_component_value(p: &mut CssParser) -> bool {
    is_at_any_value(p) || is_at_generic_delimiter(p)
}

#[inline]
pub(crate) fn parse_generic_component_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_generic_component_value(p) {
        return Absent;
    }

    if is_at_generic_delimiter(p) {
        parse_generic_delimiter(p)
    } else {
        parse_any_value(p)
    }
}

const GENERIC_DELIMITER_SET: TokenSet<CssSyntaxKind> = token_set![T![,], T![/]];
#[inline]
pub(crate) fn is_at_generic_delimiter(p: &mut CssParser) -> bool {
    p.at_ts(GENERIC_DELIMITER_SET)
}

#[inline]
fn parse_generic_delimiter(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_generic_delimiter(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(GENERIC_DELIMITER_SET);
    Present(m.complete(p, CSS_GENERIC_DELIMITER))
}
