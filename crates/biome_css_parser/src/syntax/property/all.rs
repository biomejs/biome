use crate::parser::CssParser;
use crate::syntax::parse_regular_identifier;
use biome_css_syntax::{CssSyntaxKind::*, T};
use biome_parser::prelude::ParsedSyntax::Present;
use biome_parser::prelude::*;

use super::parse_any_implicit_property_value;

/// https://drafts.csswg.org/css-cascade/#all-shorthand
///
/// ```ebnf
/// all =
///   initial |
///   inherit |
///   unset |
///   revert
/// ```
///
/// The only valid values are `CssWideKeyword`s. The gramamr doesn't include
/// all of them because some are still drafts (`revert-layer`) or have been
/// renamed (`default` -> `revert`).
#[inline]
pub(crate) fn parse_all_property(p: &mut CssParser) -> ParsedSyntax {
    // Assumes the parent has confirmed we're at the `all` identifier.
    let m = p.start();
    parse_regular_identifier(p).ok();
    p.expect(T![:]);

    // The `all` property _only_ accepts keyword values as valid input, so we
    // can skip `parse_property_value_with_fallbacks`, since it will just
    // re-check the same case.
    parse_any_implicit_property_value(p).ok();

    Present(m.complete(p, CSS_ALL_PROPERTY))
}
