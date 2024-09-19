use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

#[inline]
pub(crate) fn is_at_any_dimension(p: &mut CssParser) -> bool {
    is_at_regular_dimension(p) || is_at_percentage_dimension(p)
}

#[inline]
pub(crate) fn parse_any_dimension(p: &mut CssParser) -> ParsedSyntax {
    if is_at_percentage_dimension(p) {
        parse_percentage_dimension(p)
    } else if is_at_regular_dimension(p) {
        parse_regular_dimension(p)
    } else {
        Absent
    }
}

pub(crate) fn is_at_percentage_dimension(p: &mut CssParser) -> bool {
    p.at(CSS_PERCENTAGE_VALUE)
}
#[inline]
pub(crate) fn parse_percentage_dimension(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_percentage_dimension(p) {
        return Absent;
    }

    let m = p.start();
    // Re-cast the value portion of the dimension as a number literal.
    p.bump_remap(CSS_NUMBER_LITERAL);
    // CSS_PERCENTAGE_VALUE guarantees the `%` will be the next token,
    // but we can use expect just to be safe.
    p.expect(T![%]);
    Present(m.complete(p, CSS_PERCENTAGE))
}

#[inline]
pub(crate) fn is_at_regular_dimension(p: &mut CssParser) -> bool {
    p.at(CSS_DIMENSION_VALUE)
}

#[inline]
pub(crate) fn parse_regular_dimension(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_regular_dimension(p) {
        return Absent;
    }

    let m = p.start();
    // Re-cast the value portion of the dimension as a number literal.
    p.bump_remap(CSS_NUMBER_LITERAL);

    // Any identifier is valid as a dimension unit, but for known units we
    // use `CssRegularDimension` as a convenience elsewhere to know that the
    // unit is valid. All other identifiers become `CssUnknownDimension`.
    let kind = if is_nth_at_unit(p, 0) {
        CSS_REGULAR_DIMENSION
    } else {
        CSS_UNKNOWN_DIMENSION
    };

    // CSS_DIMENSION_VALUE guarantees that an identifier will follow the
    // number parsed previously, but the lexer will most likely treat it
    // as a keyword (like `px` as PX_KW), so it needs to be re-cast to just
    // an ident.
    p.bump_remap(T![ident]);

    Present(m.complete(p, kind))
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

// TODO: In the future, remove the `#[expect(dead_code)]` as these get used.

/// Returns true if the parser is currently at the start of a RegularDimension
/// that will become a Length value
#[inline]
#[expect(dead_code)]
pub(crate) fn is_at_length_dimension(p: &mut CssParser) -> bool {
    is_at_regular_dimension(p) && is_nth_at_length_unit(p, 1)
}
/// Returns true if the parser is currently at the start of a RegularDimension
/// that will become a Length value
#[inline]
#[expect(dead_code)]
pub(crate) fn is_at_container_lengths_dimension(p: &mut CssParser) -> bool {
    is_at_regular_dimension(p) && is_nth_at_container_lengths_unit(p, 1)
}
/// Returns true if the parser is currently at the start of a RegularDimension
/// that will become an Angle value
#[inline]
#[expect(dead_code)]
pub(crate) fn is_at_angle_dimension(p: &mut CssParser) -> bool {
    is_at_regular_dimension(p) && is_nth_at_angle_unit(p, 1)
}
/// Returns true if the parser is currently at the start of a RegularDimension
/// that will become a Time value
#[inline]
#[expect(dead_code)]
pub(crate) fn is_at_time_dimension(p: &mut CssParser) -> bool {
    is_at_regular_dimension(p) && is_nth_at_time_unit(p, 1)
}
/// Returns true if the parser is currently at the start of a RegularDimension
/// that will become a Frequency value
#[inline]
#[expect(dead_code)]
pub(crate) fn is_at_frequency_dimension(p: &mut CssParser) -> bool {
    is_at_regular_dimension(p) && is_nth_at_frequency_unit(p, 1)
}
/// Returns true if the parser is currently at the start of a RegularDimension
/// that will become a Resolution value
#[inline]
#[expect(dead_code)]
pub(crate) fn is_at_resolution_dimension(p: &mut CssParser) -> bool {
    is_at_regular_dimension(p) && is_nth_at_resolution_unit(p, 1)
}
/// Returns true if the parser is currently at the start of a RegularDimension
/// that will become a Flex value
#[inline]
#[expect(dead_code)]
pub(crate) fn is_at_flex_dimension(p: &mut CssParser) -> bool {
    is_at_regular_dimension(p) && is_nth_at_flex_unit(p, 1)
}
