use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::Parser;

use super::parse_error::expected_unit;
use super::{parse_regular_identifier, parse_regular_number};

#[inline]
pub(crate) fn is_at_dimension(p: &mut CssParser) -> bool {
    is_at_percentage_dimension(p) || is_at_regular_dimension(p)
}

#[inline]
pub(crate) fn parse_dimension(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_dimension(p) {
        return Absent;
    }
    if is_at_percentage_dimension(p) {
        return parse_percentage_dimension(p);
    }
    parse_regular_dimension(p)
}

fn is_at_percentage_dimension(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER_LITERAL) && matches!(p.nth(1), T![%])
}
#[inline]
fn parse_percentage_dimension(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_percentage_dimension(p) {
        return Absent;
    }
    let m = p.start();
    parse_regular_number(p).ok();
    p.expect(T![%]);
    Present(m.complete(p, CSS_PERCENTAGE))
}
fn is_at_regular_dimension(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER_LITERAL) && p.nth_at(1, T![ident])
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
    if !(p.at(T![ident]) && is_unit_str(p.cur_text())) {
        return Absent;
    }
    parse_regular_identifier(p)
}

#[inline]
fn is_unit_str(unit: &str) -> bool {
    is_length_unit(unit)
        || is_container_lengths_unit(unit)
        || is_angle_unit(unit)
        || is_time_unit(unit)
        || is_frequency_unit(unit)
        || is_resolution_unit(unit)
        || is_flex_unit(unit)
}

#[inline]
fn is_length_unit(unit: &str) -> bool {
    matches!(
        unit,
        "em"| "rem"| "ex"| "rex"| "cap"| "rcap"| "ch"| "rch"| "ic"| "ric"| "lh"| "rlh"|
        //  Viewport-percentage Lengths
        "vw"| "svw"| "lvw"| "dvw"| "vh"| "svh"| "lvh"| "dvh"| "vi"| "svi"| "lvi"| "dvi"| "vb"|
        "svb"| "lvb"| "dvb"| "vmin"| "svmin"| "lvmin"| "dvmin"| "vmax"| "svmax"| "lvmax"| "dvmax"|
        // Absolute lengths
        "cm"| "mm"| "q"| "in"| "pc"| "pt"| "px"| "mozmm" |
        // mini app
        "rpx"
    )
}
#[inline]
fn is_container_lengths_unit(unit: &str) -> bool {
    matches!(unit, "cqw" | "cqh" | "cqi" | "cqb" | "cqmin" | "cqmax")
}
#[inline]
fn is_angle_unit(unit: &str) -> bool {
    matches!(unit, "deg" | "grad" | "rad" | "turn")
}
#[inline]
fn is_time_unit(unit: &str) -> bool {
    matches!(unit, "s" | "ms")
}
#[inline]
fn is_frequency_unit(unit: &str) -> bool {
    matches!(unit, "hz" | "khz")
}
#[inline]
fn is_resolution_unit(unit: &str) -> bool {
    matches!(unit, "dpi" | "dpcm" | "dppx" | "x")
}
#[inline]
fn is_flex_unit(unit: &str) -> bool {
    matches!(unit, "fr")
}
