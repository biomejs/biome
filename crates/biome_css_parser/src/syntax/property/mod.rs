mod all;
mod border;
mod z_index;

use crate::parser::CssParser;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::{
    is_at_any_value, is_at_identifier, parse_any_value, parse_regular_identifier, try_parse,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

use self::all::parse_all_property;
use self::border::parse_border_property;
use self::z_index::parse_z_index_property;

pub(crate) fn is_at_any_property(p: &mut CssParser) -> bool {
    is_at_generic_property(p)
}

pub(crate) fn parse_any_property(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_property(p) {
        return Absent;
    }

    match p.cur_text() {
        "all" => parse_all_property(p),
        "border" => parse_border_property(p),
        "z-index" => parse_z_index_property(p),
        _ => parse_generic_property(p),
    }
}

#[inline]
pub(crate) fn is_at_generic_property(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.nth_at(1, T![:])
}

#[inline]
pub(crate) fn parse_generic_property(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_generic_property(p) {
        return Absent;
    }

    let m = p.start();
    parse_regular_identifier(p).ok();

    p.expect(T![:]);

    GenericComponentValueList.parse_list(p);

    Present(m.complete(p, CSS_GENERIC_PROPERTY))
}

/// Attempt to parse the value of a property using the provided parse function.
/// If the parsing succeeds, then check if the entire value definition has been
/// consumed. If it has, return the parsed content as the value for the
/// property. If more content remains, then rewind and re-parse the value
/// definition as either a CSS-wide keyword value (like `width: unset;`), or
/// fallback to a generic component value list otherwise.
///
/// This function is guaranteed to return Present syntax, handling all cases
/// including Bogus values. See [parse_any_implicit_property_value] for more
/// information.
///
/// ```rust,ignore
/// /// This function parses a property with the defined grammar of:
/// ///   my_property : <length> | <percentage>
/// /// `parse_property_value_with_fallbacks` takes care of handling syntax
/// /// errors, invalid values, and css-wide keyword values.
/// fn parse_my_property(p: &mut CssParser) -> ParsedSyntax {
///   let m = p.start();
///   parse_regular_identifer(p).ok();
///   p.expect(T![:])
///
///   parse_property_value_with_fallbacks(p, |p| {
///     parse_length(p).or_else(|| parse_percentage(p))
///   }).ok();
///
///   Present(m.complete(p, CSS_MY_PROPERTY))
/// }
/// ```
#[inline]
pub(crate) fn parse_property_value_with_fallbacks(
    p: &mut CssParser,
    func: impl FnOnce(&mut CssParser) -> ParsedSyntax,
) -> ParsedSyntax {
    let result = try_parse(p, |p| {
        let parsed = func(p);

        if parsed.is_present() && is_at_end_of_property_value(p) {
            return Ok(parsed);
        }

        Err(())
    });

    match result {
        Ok(parsed) => parsed,
        Err(_) => parse_any_implicit_property_value(p),
    }
}

/// Parse any property value other than what is explicitly defined in the formal syntax grammar
/// for that property. It is guaranteed to always return `Present`, as it handles Bogus nodes
/// internally. This includes:
///   - Any of the CSS-wide keywords, which are implicitly part of all property value definitions.
///     These become a `CssWideKeyword` node and are valid as children.
///   - Any syntactically-correct value list that does _not_ match the explicit grammar for the
///     node, which become `CssUnknownPropertyValue` (a wrapper around `CssGenericComponentValueList`).
///   - Syntactically-_incorrect_ values, which become `CssBogusPropertyValue`.
///
/// This function is automatically called when using [parse_property_value_with_fallbacks] to
/// guarantee that a value is always fully parsed and consumed.
pub(crate) fn parse_any_implicit_property_value(p: &mut CssParser) -> ParsedSyntax {
    // Attempt to parse a single keyword value as a valid definition.
    let keyword_result = try_parse(p, |p| {
        let parsed = parse_css_wide_keyword(p);
        if is_at_end_of_property_value(p) {
            return Ok(parsed);
        }

        Err(())
    });

    if let Ok(parsed) = keyword_result {
        return parsed;
    }

    // If the keyword value didn't fully consume the value, then fallback to
    // parsing a generic value list.
    let m = p.start();
    GenericComponentValueList.parse_list(p);
    Present(m.complete(p, CSS_UNKNOWN_PROPERTY_VALUE))
}

const CSS_WIDE_KEYWORD_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set![
    T![initial],
    T![inherit],
    T![unset],
    T![revert],
    T![revert_layer],
    T![default]
];

/// Parse any of the CSS-wide keywords to be used as a property value.
pub(crate) fn parse_css_wide_keyword(p: &mut CssParser) -> ParsedSyntax {
    if !p.at_ts(CSS_WIDE_KEYWORD_TOKEN_SET) {
        return Absent;
    }

    let m = p.start();
    p.bump_any();
    Present(m.complete(p, CSS_WIDE_KEYWORD))
}

const CSS_END_OF_PROPERTY_VALUE_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set!(T!['}'], T![;]);

/// Returns true if the parser has reached the end of what should be considered
/// the value for a property. This is used to ensure that all expected values
/// are reached (i.e., by looping until this condition is `false`), and that no
/// extranneous values come after the parsed content (i.e., by checking for this
/// condition at the end of successfully parsing a property value).
///
/// A property value definition ends when either a `;` is reached, which ends
/// the declaration, or a `}` is reached, which ends the declaration _list_.
pub(crate) fn is_at_end_of_property_value(p: &mut CssParser) -> bool {
    p.at_ts(CSS_END_OF_PROPERTY_VALUE_TOKEN_SET)
}

pub(crate) struct GenericComponentValueList;

impl ParseNodeList for GenericComponentValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_GENERIC_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_generic_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        !is_at_generic_component_value(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                CSS_BOGUS_PROPERTY_VALUE,
                CSS_END_OF_PROPERTY_VALUE_TOKEN_SET,
            ),
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
pub(crate) fn parse_generic_delimiter(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_generic_delimiter(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(GENERIC_DELIMITER_SET);
    Present(m.complete(p, CSS_GENERIC_DELIMITER))
}
