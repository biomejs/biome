use crate::parser::CssParser;
use crate::syntax::{parse_css_auto, parse_regular_identifier, parse_regular_number};
use biome_css_syntax::{CssSyntaxKind::*, T};
use biome_parser::prelude::ParsedSyntax::Present;
use biome_parser::prelude::*;

use super::parse_property_value_with_fallbacks;

/// https://drafts.csswg.org/css2/#z-index
///
/// ```ebnf
/// z-index =
///   auto |
///   <integer> |
///   inherit
/// ```
///
/// `inherit` is covered by the `CssWideKeyword` set.
#[inline]
pub(crate) fn parse_z_index_property(p: &mut CssParser) -> ParsedSyntax {
    // Assumes the parent has confirmed we're at the `z-index` identifier.
    let m = p.start();
    parse_regular_identifier(p).ok();
    p.expect(T![:]);

    parse_property_value_with_fallbacks(p, |p| {
        parse_css_auto(p).or_else(|| parse_regular_number(p))
    })
    .ok();

    Present(m.complete(p, CSS_Z_INDEX_PROPERTY))
}
