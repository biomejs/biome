use crate::parser::CssParser;
use crate::syntax::at_rule::layer::LayerNameList;
use crate::syntax::at_rule::media::MediaQueryList;
use crate::syntax::at_rule::supports::error::expected_any_supports_condition;
use crate::syntax::at_rule::supports::parse_any_supports_condition;
use crate::syntax::value::url::{is_at_url_function, parse_url_function};
use crate::syntax::{is_at_declaration, is_at_string, parse_declaration, parse_string};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

/// Determines if the current parsing position is at an `@import` at-rule.
///
/// This function checks the current token in the `CssParser` to see if it matches
/// the `import` rule token.
#[inline]
pub(crate) fn is_at_import_at_rule(p: &mut CssParser) -> bool {
    p.at(T![import])
}

/// Parses a `@import` rule in a CSS stylesheet.
/// This rule is used to import style rules from other style sheets.
///
/// See [CSS Cascading and Inheritance Level 4](https://drafts.csswg.org/css-cascade/#at-import)
/// for more details on the `@import` rule.
#[inline]
pub(crate) fn parse_import_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_import_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![import]);

    let kind = if is_at_import_url(p) {
        parse_import_url(p).ok();
        CSS_IMPORT_AT_RULE
    } else {
        CSS_BOGUS_AT_RULE
    };

    //  An optional cascade layer name, or for an anonymous layer.
    if is_at_import_named_layer(p) {
        parse_import_named_layer(p).ok();
    } else if is_at_import_anonymous_layer(p) {
        parse_import_anonymous_layer(p).ok();
    }

    if is_at_import_supports(p) {
        // An optional supports condition, we don't have an error here
        // is_at_import_supports validates the supports condition
        parse_import_supports(p).ok();
    }

    MediaQueryList::new(T![;]).parse_list(p);

    p.expect(T![;]);

    Present(m.complete(p, kind))
}

/// Checks if the current token in the parser is a URL or a string, indicating the start of a URL import.
///
/// This function is utilized within the context of parsing an `@import` rule in CSS. It determines
/// whether the current token is either a URL function or a string token, both of which can specify
/// the location of the resource to import.
#[inline]
pub(crate) fn is_at_import_url(p: &mut CssParser) -> bool {
    is_at_url_function(p) || is_at_string(p)
}

/// Parses the URL component of an `@import` rule in CSS.
///
/// This function checks if the current token is a valid URL or string format for an `@import` rule.
/// If it is, the function then parses the URL, either as a URL function or as a string.
#[inline]
pub(crate) fn parse_import_url(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_import_url(p) {
        return Absent;
    }

    if is_at_url_function(p) {
        parse_url_function(p)
    } else {
        parse_string(p)
    }
}

/// Determines if the current parsing position is at an anonymous layer within an `@import` rule.
///
/// This function checks whether the current token in the `CssParser` matches the `layer` token,
/// indicating the start of an anonymous layer declaration in a CSS `@import` rule.
/// This function is typically used in parsing logic to identify anonymous layer declarations within `@import` rules.
#[inline]
pub(crate) fn is_at_import_anonymous_layer(p: &mut CssParser) -> bool {
    p.at(T![layer])
}

/// Parses an anonymous layer within an `@import` rule in a CSS stylesheet.
#[inline]
pub(crate) fn parse_import_anonymous_layer(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_import_anonymous_layer(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![layer]);
    Present(m.complete(p, CSS_IMPORT_ANONYMOUS_LAYER))
}

/// Checks if the current token in the parser is the start of a named layer in an `@import` rule.
///
/// This function verifies if the current token is `layer` and is immediately followed by a `'('` token.
/// It's used to identify named layer declarations in CSS `@import` rules.
#[inline]
pub(crate) fn is_at_import_named_layer(p: &mut CssParser) -> bool {
    p.at(T![layer]) && p.nth_at(1, T!['('])
}

/// Parses a named layer within an `@import` rule in a CSS stylesheet.
///
/// This function parses the named layer by marking its beginning,
/// processing the `layer` and `'('` tokens, parsing the layer name list,
/// and expecting a closing `')'` token to complete the parse.
#[inline]
pub(crate) fn parse_import_named_layer(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_import_named_layer(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![layer]);
    p.bump(T!['(']);
    LayerNameList.parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, CSS_IMPORT_NAMED_LAYER))
}

/// Checks if the current token in the parser is the start of a `supports` condition in an `@import` rule.
///
/// This function verifies if the current token is `supports`, used to identify `supports` conditions
/// in CSS `@import` rules.
#[inline]
pub(crate) fn is_at_import_supports(p: &mut CssParser) -> bool {
    p.at(T![supports])
}

/// Parses a `supports` condition within an `@import` rule in a CSS stylesheet.
#[inline]
pub(crate) fn parse_import_supports(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_import_supports(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![supports]);
    p.expect(T!['(']);

    if is_at_declaration(p) {
        // is_at_declaration validates the declaration
        // we don't have an error here
        parse_declaration(p).ok();
    } else {
        parse_any_supports_condition(p)
            .or_recover(
                p,
                &ImportSupportsConditionParseRecovery,
                expected_any_supports_condition,
            )
            .ok();
    }

    p.expect(T![')']);
    Present(m.complete(p, CSS_IMPORT_SUPPORTS))
}

pub(crate) struct ImportSupportsConditionParseRecovery;

impl ParseRecovery for ImportSupportsConditionParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // Skips malformed or incomplete queries in parentheses until:
        // 1) ')' (closing this query context)
        // 2) ';' end of an import rule
        p.at(T![')']) || p.at(T![;]) || p.has_preceding_line_break()
    }
}
