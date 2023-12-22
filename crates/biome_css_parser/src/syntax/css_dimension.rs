use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

use super::parse_error::expected_unit;
use super::{parse_regular_identifier, parse_regular_number};

#[inline]
pub(crate) fn is_at_any_dimension(p: &mut CssParser) -> bool {
    is_at_percentage_dimension(p) || is_at_regular_dimension(p)
}

#[inline]
pub(crate) fn parse_any_dimension(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_dimension(p) {
        return Absent;
    }

    if is_at_percentage_dimension(p) {
        parse_percentage_dimension(p)
    } else {
        parse_regular_dimension(p)
    }
}

pub(crate) fn is_at_percentage_dimension(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER_LITERAL) && p.nth_at(1, T![%])
}
#[inline]
pub(crate) fn parse_percentage_dimension(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_percentage_dimension(p) {
        return Absent;
    }

    let m = p.start();
    parse_regular_number(p).ok();
    p.expect(T![%]);
    Present(m.complete(p, CSS_PERCENTAGE))
}
fn is_at_regular_dimension(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER_LITERAL) && is_nth_at_unit(p, 1)
}
#[inline]
fn parse_regular_dimension(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_regular_dimension(p) {
        return Absent;
    }
    let m = p.start();
    parse_regular_number(p).ok();
    parse_unit(p).or_add_diagnostic(p, expected_unit);
    Present(m.complete(p, CSS_REGULAR_DIMENSION))
}

#[inline]
fn parse_unit(p: &mut CssParser) -> ParsedSyntax {
    if !is_nth_at_unit(p, 0) {
        return Absent;
    }
    parse_regular_identifier(p)
}

#[inline]
fn is_nth_at_unit(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_length_unit(p, n)
        || is_nth_at_container_lengths_unit(p, n)
        || is_nth_at_angle_unit(p, n)
        || is_nth_at_time_unit(p, n)
        || is_nth_at_frequency_unit(p, n)
        || is_nth_at_resolution_unit(p, n)
        || is_nth_at_flex_unit(p, n)
}

const LENGTH_UNIT_SET: TokenSet<CssSyntaxKind> = token_set!(
    T![em],
    T![rem],
    T![ex],
    T![rex],
    T![cap],
    T![rcap],
    T![ch],
    T![rch],
    T![ic],
    T![ric],
    T![lh],
    T![rlh],
    //  Viewport-percentage Lengths
    T![vw],
    T![svw],
    T![lvw],
    T![dvw],
    T![vh],
    T![svh],
    T![lvh],
    T![dvh],
    T![vi],
    T![svi],
    T![lvi],
    T![dvi],
    T![vb],
    T![svb],
    T![lvb],
    T![dvb],
    T![vmin],
    T![svmin],
    T![lvmin],
    T![dvmin],
    T![vmax],
    T![svmax],
    T![lvmax],
    T![dvmax],
    // Absolute lengths
    T![cm],
    T![mm],
    T![q],
    T![in],
    T![pc],
    T![pt],
    T![px],
    T![mozmm],
    // mini app
    T![rpx],
);

#[inline]
fn is_nth_at_length_unit(p: &mut CssParser, n: usize) -> bool {
    p.nth_at_ts(n, LENGTH_UNIT_SET)
}
const CONTAINER_LENGTHS_UNIT_SET: TokenSet<CssSyntaxKind> =
    token_set!(T![cqw], T![cqh], T![cqi], T![cqb], T![cqmin], T![cqmax],);

#[inline]
fn is_nth_at_container_lengths_unit(p: &mut CssParser, n: usize) -> bool {
    p.nth_at_ts(n, CONTAINER_LENGTHS_UNIT_SET)
}

const ANGLE_UNIT_SET: TokenSet<CssSyntaxKind> = token_set!(T![deg], T![grad], T![rad], T![turn],);

#[inline]
fn is_nth_at_angle_unit(p: &mut CssParser, n: usize) -> bool {
    p.nth_at_ts(n, ANGLE_UNIT_SET)
}
const TIME_UNIT_SET: TokenSet<CssSyntaxKind> = token_set!(T![s], T![ms],);

#[inline]
fn is_nth_at_time_unit(p: &mut CssParser, n: usize) -> bool {
    p.nth_at_ts(n, TIME_UNIT_SET)
}
const FREQUENCY_UNIT_SET: TokenSet<CssSyntaxKind> = token_set!(T![hz], T![khz],);

#[inline]
fn is_nth_at_frequency_unit(p: &mut CssParser, n: usize) -> bool {
    p.nth_at_ts(n, FREQUENCY_UNIT_SET)
}
const RESOLUTION_UNIT_SET: TokenSet<CssSyntaxKind> =
    token_set!(T![dpi], T![dpcm], T![dppx], T![x],);

#[inline]
fn is_nth_at_resolution_unit(p: &mut CssParser, n: usize) -> bool {
    p.nth_at_ts(n, RESOLUTION_UNIT_SET)
}
const FLEX_UNIT_SET: TokenSet<CssSyntaxKind> = token_set!(T![fr],);

#[inline]
fn is_nth_at_flex_unit(p: &mut CssParser, n: usize) -> bool {
    p.nth_at_ts(n, FLEX_UNIT_SET)
}
