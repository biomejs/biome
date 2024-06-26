pub(crate) mod color;
pub(crate) mod unicode_range;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::css_modules::{
    composes_not_allowed, expected_classes_list, expected_composes_import_source,
};
use crate::syntax::parse_error::{expected_component_value, expected_identifier};
use crate::syntax::{
    is_at_any_value, is_at_dashed_identifier, is_at_identifier, is_at_string, parse_any_value,
    parse_custom_identifier_with_keywords, parse_dashed_identifier, parse_regular_identifier,
    parse_string,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecovery, ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

#[inline]
pub(crate) fn is_at_any_property(p: &mut CssParser) -> bool {
    is_at_generic_property(p)
}

#[inline]
pub(crate) fn parse_any_property(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_property(p) {
        return Absent;
    }

    match p.cur() {
        T![composes] => parse_composes_property(p),
        _ => parse_generic_property(p),
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
fn parse_composes_property(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_composes_property(p) {
        return Absent;
    }

    if p.options().is_css_modules_disabled() {
        // `composes` is not a standard CSS feature.
        // Provide a hint on how to enable parsing of the `composes` declaration.
        p.error(composes_not_allowed(p, p.cur_range()));

        // Fallback to a generic property
        return parse_generic_property(p);
    }

    let m = p.start();
    // remap the `composes` keyword to a regular identifier
    parse_regular_identifier(p).ok();
    p.bump(T![:]);

    {
        let m = p.start();

        let classes = ComposesClassList.parse_list(p);

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
struct ComposesClassList;

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
        p.at_ts(END_OF_COMPOSES_CLASS_TOKEN_SET)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &ComposesClassListParseRecovery, expected_identifier)
    }
}

struct ComposesClassListParseRecovery;

impl ParseRecovery for ComposesClassListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // If the next token is the end of the list or the next element, we're at a recovery point.
        p.at_ts(END_OF_COMPOSES_CLASS_TOKEN_SET) || is_at_identifier(p)
    }
}

const END_OF_COMPOSES_CLASS_TOKEN_SET: TokenSet<CssSyntaxKind> =
    END_OF_PROPERTY_VALUE_TOKEN_SET.union(token_set!(T![from]));

#[inline]
fn is_at_generic_property(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.nth_at(1, T![:])
}

#[inline]
fn parse_generic_property(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_generic_property(p) {
        return Absent;
    }

    let m = p.start();

    if is_at_dashed_identifier(p) {
        parse_dashed_identifier(p).ok();
    } else {
        parse_regular_identifier(p).ok();
    }

    p.expect(T![:]);

    GenericComponentValueList.parse_list(p);

    Present(m.complete(p, CSS_GENERIC_PROPERTY))
}
const END_OF_PROPERTY_VALUE_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set!(T!['}'], T![;]);

struct GenericComponentValueList;

impl ParseNodeList for GenericComponentValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_GENERIC_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_generic_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(END_OF_PROPERTY_VALUE_TOKEN_SET) || p.at(T![')']) || /* !token is !important */ p.at(T![!])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_PROPERTY_VALUE, END_OF_PROPERTY_VALUE_TOKEN_SET),
            expected_component_value,
        )
    }
}

#[inline]
fn is_at_generic_component_value(p: &mut CssParser) -> bool {
    is_at_any_value(p) || is_at_generic_delimiter(p)
}

#[inline]
fn parse_generic_component_value(p: &mut CssParser) -> ParsedSyntax {
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
fn is_at_generic_delimiter(p: &mut CssParser) -> bool {
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
