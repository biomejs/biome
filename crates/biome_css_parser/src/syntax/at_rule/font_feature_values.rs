use super::parse_error::{
    expected_any_font_family_name, expected_any_font_feature_value_item,
    expected_font_feature_values_item,
};
use crate::syntax::block::{parse_declaration_block, ParseBlockBody};
use crate::syntax::is_at_string;
use crate::{
    lexer::CssLexContext,
    parser::CssParser,
    syntax::{
        is_at_identifier, parse_custom_identifier,
        parse_error::expected_non_css_wide_keyword_identifier, parse_string,
    },
};
use biome_css_syntax::{
    CssSyntaxKind::{self, *},
    T,
};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::{
    parse_lists::ParseNodeList,
    parse_recovery::{ParseRecoveryTokenSet, RecoveryResult},
    parsed_syntax::ParsedSyntax::{self, Absent, Present},
    prelude::*,
    Parser, TokenSet,
};

/// Checks if the current token in the parser is a `@font-feature-values` at-rule.
///
/// This function verifies whether the current token matches the `@font-feature-values` rule,
/// which is used for applying styles to specific parts of a document.
#[inline]
pub(crate) fn is_at_font_feature_values_at_rule(p: &mut CssParser) -> bool {
    p.at(T![font_feature_values])
}

/// Parses a `@font-feature-values` at-rule in a CSS stylesheet.
///
/// This function processes the `@font-feature-values` at-rule, as defined in the CSS Fonts Module Level 4.
///
/// Specification:
/// [CSS Fonts Module Level 4](https://drafts.csswg.org/css-fonts/#font-feature-values)
/// The `@font-feature-values` rule allows authors to use a common name in the `font-feature-settings` property
/// for different types of font feature values. It aids in the readability and maintainability of CSS code
/// by allowing custom identifiers for various font feature settings.
///
/// # Examples
/// Basic usage in CSS:
///
/// ```css
/// @font-feature-values Taisho Gothic, Bar {
///   @stylistic { /* CSS declarations here */ }
///   @styleset { /* CSS declarations here */ }
/// }
/// ```
///
/// This function is crucial for parsing and interpreting the `@font-feature-values` rules,
/// allowing CSS authors to define custom names for complex font feature settings, enhancing the CSS's expressiveness and flexibility.
#[inline]
pub(crate) fn parse_font_feature_values_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_font_feature_values_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![font_feature_values]);

    CssFontFamilyNameList.parse_list(p);
    FontFeatureValuesBlock.parse_block_body(p);

    Present(m.complete(p, CSS_FONT_FEATURE_VALUES_AT_RULE))
}

struct CssFontFamilyNameList;

impl ParseSeparatedList for CssFontFamilyNameList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_FONT_FAMILY_NAME_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_font_family_name(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['{'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &CssFontFamilyNameListParseRecovery,
            expected_any_font_family_name,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

struct CssFontFamilyNameListParseRecovery;

impl ParseRecovery for CssFontFamilyNameListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_FONT_FAMILY_NAME;
    /// Determines if the parser has reached a point where it can recover from an error
    /// while parsing a font family name list.
    ///
    /// This function checks if the parser is at a position where it can safely resume parsing
    /// after encountering an error in a font family name list. The recovery points are:
    /// - The start of a new font family name.
    /// - An opening curly brace '{', indicating the start of a block.
    /// - A comma ',', indicating the `CssFontFamilyNameList` list separator.
    /// # Examples
    /// Basic usage in CSS:
    ///
    /// ```css
    /// @font-feature-values Font,
    /// 123123, /* Error in name, recover here */
    /// "string font name" { /* Start of block, another recovery point */
    ///     /* CSS declarations here */
    /// }
    /// ```
    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['{']) || p.at(T![,]) || is_at_font_family_name(p)
    }
}

/// Determines whether the current token in the parser is a font family name.
///
/// This function checks if the current position in the CSS parser is pointing to a valid
/// font family name by looking for either an identifier or a string token. Font family names
/// can be specified either as named fonts, like `Arial`, or as strings, like `"Times New Roman"`.
///
/// Font family names are used within various CSS properties, such as `font-family`, to specify
/// which font should be applied to text.
#[inline]
pub(crate) fn is_at_font_family_name(p: &mut CssParser) -> bool {
    is_at_identifier(p) || is_at_string(p)
}

/// Parses a font family name from the CSS content being analyzed.
///
/// This function attempts to parse a font family name at the current position of the CSS parser.
/// Font family names can be provided either as strings (e.g., "Times New Roman") or as identifiers
/// (e.g., Arial). The function first checks if the current position indeed represents a font family
/// name by calling `is_at_font_family_name`. If a font family name is present, it then determines
/// whether the name is provided as a string or an identifier and parses it accordingly.
#[inline]
pub(crate) fn parse_font_family_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_font_family_name(p) {
        return Absent;
    }

    if is_at_string(p) {
        parse_string(p)
    } else {
        let m = p.start();
        CssCustomIdentifierList.parse_list(p);
        Present(m.complete(p, CSS_FONT_FAMILY_NAME))
    }
}

struct CssCustomIdentifierList;

impl ParseNodeList for CssCustomIdentifierList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_CUSTOM_IDENTIFIER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_custom_identifier(p, CssLexContext::Regular)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['{']) || p.at(T![,])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &CssCustomIdentifierListParseRecovery,
            expected_non_css_wide_keyword_identifier,
        )
    }
}

struct CssCustomIdentifierListParseRecovery;

impl ParseRecovery for CssCustomIdentifierListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_CUSTOM_IDENTIFIER;
    /// Determines if the parser has reached a point where it can recover from an error
    /// while parsing a custom identifier list.
    ///
    /// This function checks if the parser is at a position where it can safely resume parsing
    /// after encountering an error in a custom identifier list. The recovery points are:
    /// - The start of a new custom identifier.
    /// - An opening curly brace '{', indicating the start of a ruleset.
    /// - A comma ',', indicating the `CssFontFamilyNameList` list separator.
    /// # Examples
    /// Basic usage in CSS:
    ///
    /// ```css
    /// @font-feature-values Font
    /// 123123, /* Error in name, recover here */
    /// Second "error" { /* Start of block, another recovery point */
    ///     /* CSS declarations here */
    /// }
    /// ```
    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['{']) || p.at(T![,]) || is_at_identifier(p)
    }
}

struct FontFeatureValuesBlock;

impl ParseBlockBody for FontFeatureValuesBlock {
    const BLOCK_KIND: CssSyntaxKind = CSS_FONT_FEATURE_VALUES_BLOCK;

    fn is_at_element(&self, p: &mut CssParser) -> bool {
        p.at(T![@])
    }

    fn parse_list(&mut self, p: &mut CssParser) {
        FontFeatureValuesItemList.parse_list(p);
    }
}

struct FontFeatureValuesItemList;

impl ParseNodeList for FontFeatureValuesItemList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_FONT_FEATURE_VALUES_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_font_feature_values_item(p)
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
            &ParseRecoveryTokenSet::new(
                CSS_BOGUS_FONT_FEATURE_VALUES_ITEM,
                FONT_FEATURE_VALUES_ITEM_LIST_RECOVERY_SET,
            ),
            expected_font_feature_values_item,
        )
    }
}

/// Parses a single item within a `@font-feature-values` at-rule in a CSS stylesheet.
///
/// This function targets the parsing of individual feature value definitions inside the `@font-feature-values`
/// rule, which is a part of the CSS Fonts Module. Each item represents font feature values associated with a
/// specific font feature tag, such as `@stylistic`, `@swash`, etc. The parsing begins by checking for the
/// presence of an at-rule indicator (@). If found, it proceeds to verify the feature tag and then parse the
/// declaration block that defines the values for this feature.
/// # CSS Example
/// ```css
/// @font-feature-values Lato {
///   @styleset {
///     /* CSS declarations here */
///   }
///   @swash {
///     /* CSS declarations here */
///   }
/// }
/// ```
#[inline]
fn parse_font_feature_values_item(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![@]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![@]);

    if !p.eat_ts(FONT_FEATURE_VALUES_ITEM_SET) {
        p.error(expected_any_font_feature_value_item(p, p.cur_range()))
    }

    parse_declaration_block(p);

    Present(m.complete(p, CSS_FONT_FEATURE_VALUES_ITEM))
}
const FONT_FEATURE_VALUES_ITEM_SET: TokenSet<CssSyntaxKind> = token_set![
    T![stylistic],
    T![historical_forms],
    T![styleset],
    T![character_variant],
    T![swash],
    T![ornaments],
    T![annotation]
];
const FONT_FEATURE_VALUES_ITEM_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    FONT_FEATURE_VALUES_ITEM_SET.union(token_set![T!['}']]);
