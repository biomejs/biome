use crate::parser::CssParser;
use crate::syntax::CssSyntaxFeatures;
use biome_css_syntax::CssSyntaxKind::SCSS_PARENT_SELECTOR_VALUE;
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::SyntaxFeature;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

/// Detects the SCSS parent selector value `&`.
#[inline]
pub(crate) fn is_at_scss_parent_selector_value(p: &mut CssParser) -> bool {
    // `&` is a generic token in CSS parsing/recovery. Keep the SCSS gate here so
    // plain CSS doesn't accidentally route through SCSS-only diagnostics.
    CssSyntaxFeatures::Scss.is_supported(p) && p.at(T![&])
}

/// Parses the SCSS parent selector value `&`.
#[inline]
pub(crate) fn parse_scss_parent_selector_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_parent_selector_value(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![&]);
    Present(m.complete(p, SCSS_PARENT_SELECTOR_VALUE))
}
