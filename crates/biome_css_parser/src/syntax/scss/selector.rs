use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::CssSyntaxFeatures;
use crate::syntax::parse_error::{expected_identifier, expected_number};
use crate::syntax::parse_number;
use crate::syntax::scss::{
    is_at_scss_interpolation, is_nth_at_scss_interpolated_identifier, is_nth_at_scss_interpolation,
    parse_scss_interpolation_inner_expression, parse_scss_interpolation_prefix,
};
use crate::syntax::selector::{
    PSEUDO_CLASS_NTH_SIGN_SET, parse_pseudo_class_nth_dimension_value,
    parse_selector_custom_identifier,
};
use biome_css_syntax::CssSyntaxKind::{
    CSS_BOGUS, CSS_DIMENSION_VALUE, CSS_NTH_OFFSET, CSS_NUMBER_LITERAL, CSS_PSEUDO_CLASS_NTH,
    CSS_PSEUDO_CLASS_NTH_NUMBER, SCSS_INTERPOLATED_NTH_VALUE,
    SCSS_INTERPOLATED_NTH_VALUE_PART_LIST, SCSS_INTERPOLATION, SCSS_PLACEHOLDER_SELECTOR,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{RecoveryError, RecoveryResult};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{CompletedMarker, Parser, SyntaxFeature};

#[inline]
pub(crate) fn is_nth_at_scss_placeholder_selector(p: &mut CssParser, n: usize) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p)
        && p.nth_at(n, T![%])
        && is_nth_at_scss_interpolated_identifier(p, n + 1)
}

/// Parses an SCSS placeholder selector such as `%toolbelt`.
#[inline]
pub(crate) fn parse_scss_placeholder_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_nth_at_scss_placeholder_selector(p, 0) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![%]);
    parse_selector_custom_identifier(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, SCSS_PLACEHOLDER_SELECTOR))
}

#[inline]
pub(crate) fn is_at_scss_pseudo_class_nth(p: &mut CssParser) -> bool {
    if !CssSyntaxFeatures::Scss.is_supported(p) {
        return false;
    }

    let n = if p.at_ts(PSEUDO_CLASS_NTH_SIGN_SET) {
        1
    } else {
        0
    };

    is_nth_at_scss_pseudo_class_nth_value_with_interpolation(p, n)
        || p.nth_at(n, CSS_DIMENSION_VALUE)
            && p.nth_at(n + 1, T![n])
            && is_at_scss_interpolated_nth_offset(p, n + 2)
        || p.nth_at(n, T![n]) && is_at_scss_interpolated_nth_offset(p, n + 1)
}

/// Parses an SCSS-interpolated nth selector argument.
///
/// Examples:
/// - `#{$i}`
/// - `#{$numPerRow}n + 1`
/// - `#{$sign}2n + #{$offset}`
/// - `2n + #{$offset}`
/// - `2n + 1#{$offset}`
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_scss_pseudo_class_nth(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_pseudo_class_nth(p) {
        return Absent;
    }

    let m = p.start();

    p.eat_ts_with_context(PSEUDO_CLASS_NTH_SIGN_SET, CssLexContext::PseudoNthSelector);

    let kind = if is_at_scss_pseudo_class_nth_value(p) {
        parse_scss_pseudo_class_nth_value(p).ok();

        if is_at_scss_pseudo_class_nth_symbol(p) {
            p.expect_with_context(T![n], CssLexContext::PseudoNthSelector);
            parse_scss_pseudo_class_nth_offset(p).ok();
            CSS_PSEUDO_CLASS_NTH
        } else {
            CSS_PSEUDO_CLASS_NTH_NUMBER
        }
    } else {
        match p.cur() {
            T![n] => {
                p.bump_with_context(T![n], CssLexContext::PseudoNthSelector);
                parse_scss_pseudo_class_nth_offset(p).ok();
                CSS_PSEUDO_CLASS_NTH
            }
            _ => CSS_BOGUS,
        }
    };

    Present(m.complete(p, kind))
}

#[inline]
fn is_at_scss_pseudo_class_nth_symbol(p: &mut CssParser) -> bool {
    !p.has_preceding_whitespace() && p.at(T![n])
}

#[inline]
fn is_at_scss_interpolated_nth_offset(p: &mut CssParser, n: usize) -> bool {
    p.nth_at_ts(n, PSEUDO_CLASS_NTH_SIGN_SET)
        && is_nth_at_scss_pseudo_class_nth_value_with_interpolation(p, n + 1)
        // `2n+1#{$offset}` can lex `+1` as one number before pseudo-nth
        // lexing splits the offset sign.
        || is_nth_at_scss_number_interpolation(p, n)
}

#[inline]
fn is_nth_at_scss_pseudo_class_nth_value_with_interpolation(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_scss_interpolation(p, n) || is_nth_at_scss_number_interpolation(p, n)
}

#[inline]
fn is_nth_at_scss_number_interpolation(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, CSS_NUMBER_LITERAL)
        && is_nth_at_scss_interpolation(p, n + 1)
        && !p.has_nth_preceding_whitespace(n + 1)
}

/// Parses an nth offset whose value may contain SCSS interpolation.
///
/// Examples:
/// - `+ 1`
/// - `- #{$offset}`
/// - `+ 1#{$offset}`
#[inline]
fn parse_scss_pseudo_class_nth_offset(p: &mut CssParser) -> ParsedSyntax {
    if !p.at_ts(PSEUDO_CLASS_NTH_SIGN_SET) {
        return Absent;
    }

    let m = p.start();

    p.bump_ts(PSEUDO_CLASS_NTH_SIGN_SET);
    parse_scss_pseudo_class_nth_value(p).or_add_diagnostic(p, expected_number);

    Present(m.complete(p, CSS_NTH_OFFSET))
}

/// Parses an nth value that may contain SCSS interpolation.
///
/// Examples:
/// - `#{$sign}2`
/// - `#{$offset}`
/// - `1#{$offset}`
/// - `2`
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
fn parse_scss_pseudo_class_nth_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_pseudo_class_nth_value(p) {
        return Absent;
    }

    let first_part = match parse_scss_pseudo_class_nth_value_part(p) {
        Present(first_part) => first_part,
        Absent => return Absent,
    };

    if is_at_adjacent_scss_pseudo_class_nth_value_part(p) {
        // `#{$sign}2n`: keep interpolation and `2` in one coefficient value.
        Present(
            ScssInterpolatedNthValuePartList::new(first_part)
                .parse_list(p)
                .precede(p)
                .complete(p, SCSS_INTERPOLATED_NTH_VALUE),
        )
    } else {
        // `#{$n}n`, `2n`, and `n + #{$offset}` use direct value leaves.
        Present(first_part)
    }
}

#[inline]
fn is_at_scss_pseudo_class_nth_value(p: &mut CssParser) -> bool {
    is_at_scss_interpolation(p) || p.at(CSS_DIMENSION_VALUE) || p.at(CSS_NUMBER_LITERAL)
}

/// Parses the mixed nth value part list in `#{$sign}2`.
struct ScssInterpolatedNthValuePartList {
    first_part: CompletedMarker,
}

impl ScssInterpolatedNthValuePartList {
    #[inline]
    fn new(first_part: CompletedMarker) -> Self {
        Self { first_part }
    }
}

impl ParseNodeList for ScssInterpolatedNthValuePartList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: CssSyntaxKind = SCSS_INTERPOLATED_NTH_VALUE_PART_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_scss_pseudo_class_nth_value_part(p)
    }

    fn start_list(&mut self, p: &mut CssParser) -> Marker {
        // `#{$sign}` is already owned; wrap it before parsing the adjacent `2`.
        self.first_part.clone().precede(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        !is_at_adjacent_scss_pseudo_class_nth_value_part(p)
    }

    fn recover(&mut self, _p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        match parsed_element {
            Present(marker) => Ok(marker),
            Absent => Err(RecoveryError::AlreadyRecovered),
        }
    }
}

#[inline]
fn is_at_adjacent_scss_pseudo_class_nth_value_part(p: &mut CssParser) -> bool {
    !p.has_preceding_whitespace() && is_at_scss_pseudo_class_nth_value(p)
}

/// Parses one nth value fragment.
///
/// Examples: `#{$n}`, `2`, `2n`
#[inline]
fn parse_scss_pseudo_class_nth_value_part(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolation(p) {
        parse_scss_interpolation_in_nth(p)
    } else if p.at(CSS_DIMENSION_VALUE) {
        parse_pseudo_class_nth_dimension_value(p)
    } else {
        parse_number(p, CssLexContext::PseudoNthSelector)
    }
}

/// Parses one interpolation inside an nth selector argument.
///
/// The closing `}` returns to pseudo-nth lexing so `#{$value}n` keeps the `n`
/// token separate from identifiers.
#[inline]
fn parse_scss_interpolation_in_nth(p: &mut CssParser) -> ParsedSyntax {
    let Some(m) = parse_scss_interpolation_prefix(p) else {
        return Absent;
    };

    parse_scss_interpolation_inner_expression(p);
    p.expect_with_context(T!['}'], CssLexContext::PseudoNthSelector);

    Present(m.complete(p, SCSS_INTERPOLATION))
}
