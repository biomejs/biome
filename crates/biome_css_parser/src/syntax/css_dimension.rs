use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::Parser;

use super::{parse_regular_identifier, parse_regular_number};

#[inline]
pub(crate) fn is_at_css_dimension(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER_LITERAL) && is_unit(p.nth(1))
}

#[inline]
pub(crate) fn parse_css_dimension(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_css_dimension(p) {
        return Absent;
    }
    let m = p.start();
    let _css_number = parse_regular_number(p);
    let _ident = parse_regular_identifier(p);
    p.eat(T![%]);
    Present(m.complete(p, CSS_DIMENSION))
}

#[inline]
fn is_unit(unit: CssSyntaxKind) -> bool {
    match unit {
        T![%] => true,
        // how to get text ?
        // T![ident] => is_unit_str( p.text(unit)),
        _ => false,
    }
}
// #[inline]
// fn is_unit_str(unit: &str) -> bool {
//     is_length_unit(unit) || is_container_lengths_unit(unit) || is_angle_unit(unit) || is_time_unit(unit) || is_frequency_unit(unit) || is_resolution_unit(unit) || is_flex_unit(unit)
// }

// #[inline]
// fn is_length_unit(unit: &str) -> bool {
//     matches!(
//         unit, "em"| "rem"| "ex"| "rex"| "cap"| "rcap"| "ch"| "rch"| "ic"| "ric"| "lh"| "rlh"|
//         //  Viewport-percentage Lengths
//         "vw"| "svw"| "lvw"| "dvw"| "vh"| "svh"| "lvh"| "dvh"| "vi"| "svi"| "lvi"| "dvi"| "vb"|
//         "svb"| "lvb"| "dvb"| "vmin"| "svmin"| "lvmin"| "dvmin"| "vmax"| "svmax"| "lvmax"| "dvmax"|
//         // Absolute lengths
//         "cm"| "mm"| "q"| "in"| "pc"| "pt"| "px"| "mozmm" |
//         // mini app
//         "rpx"
//     )
// }
// #[inline]
// fn is_container_lengths_unit(unit: &str) -> bool {
//     matches!(unit, "cqw"| "cqh"| "cqi"| "cqb"| "cqmin"| "cqmax")
// }
// #[inline]
// fn is_angle_unit(unit: &str) -> bool {
//     matches!(unit, "deg"| "grad"| "rad"| "turn")
// }
// #[inline]
// fn is_time_unit(unit: &str) -> bool {
//     matches!(unit, "s"| "ms")
// }
// #[inline]
// fn is_frequency_unit(unit: &str) -> bool {
//     matches!(unit, "hz"| "khz")
// }
// #[inline]
// fn is_resolution_unit(unit: &str) -> bool {
//     matches!(unit, "dpi"| "dpcm"| "dppx"| "x")
// }
// #[inline]
// fn is_flex_unit(unit: &str) -> bool {
//     matches!(unit, "fr")
// }
