use crate::parser::CssParser;
use crate::syntax::value::dimension::{is_at_length_dimension, parse_regular_dimension};
use crate::syntax::{is_at_color, parse_color, parse_regular_identifier};
use biome_css_syntax::{
    CssSyntaxKind::{self, *},
    T,
};
use biome_parser::diagnostic::expect_one_of;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_property_value_with_fallbacks;

/// https://drafts.csswg.org/css-backgrounds/#propdef-border
///
/// ```ebnf
///  border =
///      <line-width>  ||
///      <line-style>  ||
///      <color>
///
///  <line-width> =
///      <length [0,∞]>  |
///      thin            |
///      medium          |
///      thick
///
///  <line-style> =
///      none    |
///      hidden  |
///      dotted  |
///      dashed  |
///      solid   |
///      double  |
///      groove  |
///      ridge   |
///      inset   |
///      outset
/// ```
#[inline]
pub(crate) fn parse_border_property(p: &mut CssParser) -> ParsedSyntax {
    // Assumes the parent has confirmed we're at the `border` identifier.
    let m = p.start();
    parse_regular_identifier(p).ok();
    p.expect(T![:]);

    parse_property_value_with_fallbacks(p, parse_css_border).ok();

    Present(m.complete(p, CSS_BORDER_PROPERTY))
}

/// Parse a complete border shorthand value:
///     <line-width>  ||
///     <line-style>  ||
///     <color>
fn parse_css_border(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    let mut map = [false; 3];
    let mut any = false;

    loop {
        if !map[0] && is_at_line_width(p) {
            parse_any_line_width(p).ok();
            map[0] = true;
            any = true;
        } else if !map[1] && is_at_line_style(p) {
            parse_line_style(p).ok();
            map[1] = true;
            any = true;
        } else if !map[2] && is_at_color(p) {
            parse_color(p).ok();
            map[2] = true;
            any = true;
        } else {
            break;
        }
    }

    if !any {
        p.error(expect_one_of(
            &["line width", "line style", "color"],
            p.cur_range(),
        ));
        m.abandon(p);
        return Absent;
    }

    Present(m.complete(p, CSS_BORDER))
}

fn parse_any_line_width(p: &mut CssParser) -> ParsedSyntax {
    if is_at_length_dimension(p) {
        parse_regular_dimension(p)
    } else if p.at_ts(LINE_WIDTH_TOKEN_SET) {
        let m = p.start();
        p.bump_ts(LINE_WIDTH_TOKEN_SET);
        Present(m.complete(p, CSS_LINE_WIDTH_KEYWORD))
    } else {
        Absent
    }
}

fn parse_line_style(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    p.bump_ts(LINE_STYLE_TOKEN_SET);
    Present(m.complete(p, CSS_LINE_STYLE))
}

const LINE_WIDTH_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set![T![thin], T![medium], T![thick]];

fn is_at_line_width(p: &mut CssParser<'_>) -> bool {
    is_at_length_dimension(p) || p.at_ts(LINE_WIDTH_TOKEN_SET)
}

const LINE_STYLE_TOKEN_SET: TokenSet<CssSyntaxKind> = token_set![
    T![none],
    T![hidden],
    T![dotted],
    T![dashed],
    T![solid],
    T![double],
    T![groove],
    T![ridge],
    T![inset],
    T![outset]
];

fn is_at_line_style(p: &mut CssParser<'_>) -> bool {
    p.at_ts(LINE_STYLE_TOKEN_SET)
}
