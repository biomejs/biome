use crate::parser::CssParser;
use crate::syntax::at_rule::media::{is_at_any_media_query, parse_any_media_query};
use crate::syntax::at_rule::{
    expected_media_query, is_at_import_url, is_nth_at_import_url, parse_import_non_media_modifiers,
    parse_import_url,
};
use crate::syntax::parse_error::expected_string;
use crate::syntax::value::url::is_at_url_function;
use crate::syntax::{is_at_string, parse_string};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;
use biome_parser::{TokenSet, token_set};

const SCSS_IMPORT_ITEM_LIST_END_SET: TokenSet<CssSyntaxKind> = token_set![T![;]];
const SCSS_IMPORT_ITEM_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T![,], T![;]];

#[inline]
pub(crate) fn is_at_scss_import_at_rule(p: &mut CssParser) -> bool {
    p.at(T![import])
}

/// Parses the SCSS `@import` at-rule.
///
/// # Example
///
/// ```scss
/// @import "theme", "rounded-corners";
/// ```
#[inline]
pub(crate) fn parse_scss_import_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_import_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![import]);
    parse_scss_import_item_list(p);
    p.expect(T![;]);

    Present(m.complete(p, SCSS_IMPORT_AT_RULE))
}

/// Parses the comma-separated SCSS `@import` item list.
///
/// # Example
///
/// ```scss
/// @import "theme", "rounded-corners";
/// ```
#[inline]
fn parse_scss_import_item_list(p: &mut CssParser) -> CompletedMarker {
    ScssImportItemList.parse_list(p)
}

/// Parses a quoted SCSS `@import` item and classifies it after consuming the string.
///
/// A quoted import becomes a plain-CSS import if its target is classified as
/// plain CSS or if the following token starts CSS import modifiers such as
/// `layer(...)`, `supports(...)`, or media queries.
#[inline]
fn parse_scss_string_import_item(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_string(p) {
        return Absent;
    }

    let is_plain_target = is_at_plain_css_import_target(p.cur_text());
    // Guarded by `is_at_string` above.
    let Some(import_string) = parse_string(p).ok() else {
        return Absent;
    };

    if is_plain_target || is_at_import_modifier(p) {
        let m = import_string.precede(p);
        parse_scss_plain_import_modifiers(p);
        Present(m.complete(p, SCSS_PLAIN_IMPORT))
    } else {
        Present(import_string)
    }
}

/// Parses a single plain-CSS import item inside an SCSS `@import` list.
///
/// # Example
///
/// ```scss
/// @import "theme.css", url("fonts.css");
/// ```
#[inline]
fn parse_scss_plain_import(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_import_url(p) {
        return Absent;
    }

    let m = p.start();
    parse_import_url(p).ok();
    parse_scss_plain_import_modifiers(p);

    Present(m.complete(p, SCSS_PLAIN_IMPORT))
}

/// Parses the optional modifier clauses for a plain-CSS SCSS import item.
///
/// This mirrors CSS `@import` modifier parsing, but the media-query list stops
/// at a comma when that comma begins the next SCSS import item.
#[inline]
fn parse_scss_plain_import_modifiers(p: &mut CssParser) {
    parse_import_non_media_modifiers(p);
    ScssImportMediaQueryList.parse_list(p);
}

/// Returns `true` when the current token begins CSS import modifiers.
#[inline]
fn is_at_import_modifier(p: &mut CssParser) -> bool {
    p.at(T![layer]) || p.at(T![supports]) || is_at_any_media_query(p)
}

/// Returns `true` when a quoted SCSS import target is classified as plain CSS.
///
/// This mirrors Sass's plain-CSS import classification for quoted URLs:
/// absolute HTTP(S) URLs, protocol-relative URLs, and paths ending in `.css`
/// are treated as CSS imports rather than Sass module imports.
///
/// Docs: https://sass-lang.com/documentation/at-rules/import/#plain-css-imports
#[inline]
fn is_at_plain_css_import_target(text: &str) -> bool {
    let Some(unquoted) = text
        .strip_prefix('"')
        .and_then(|text| text.strip_suffix('"'))
        .or_else(|| {
            text.strip_prefix('\'')
                .and_then(|text| text.strip_suffix('\''))
        })
    else {
        return false;
    };

    if unquoted.starts_with("http://")
        || unquoted.starts_with("https://")
        || unquoted.starts_with("//")
    {
        return true;
    }

    unquoted.ends_with(".css")
}

struct ScssImportItemList;

impl ParseSeparatedList for ScssImportItemList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_IMPORT_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_scss_import_item(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(SCSS_IMPORT_ITEM_LIST_END_SET)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, SCSS_IMPORT_ITEM_LIST_RECOVERY_SET),
            expected_string,
        )
    }

    fn allow_empty(&self) -> bool {
        false
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

#[inline]
fn parse_scss_import_item(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_import_url(p) {
        return Absent;
    }

    if is_at_url_function(p) {
        parse_scss_plain_import(p)
    } else {
        parse_scss_string_import_item(p)
    }
}

struct ScssImportMediaQueryList;

impl ParseSeparatedList for ScssImportMediaQueryList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_MEDIA_QUERY_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_any_media_query(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![;]) || (p.at(T![,]) && is_nth_at_import_url(p, 1))
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_MEDIA_QUERY, token_set![T![,], T![;]]),
            expected_media_query,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}
