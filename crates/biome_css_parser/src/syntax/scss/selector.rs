use crate::parser::CssParser;
use crate::syntax::CssSyntaxFeatures;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::scss::is_nth_at_scss_interpolated_identifier;
use crate::syntax::selector::parse_selector_custom_identifier;
use biome_css_syntax::CssSyntaxKind::SCSS_PLACEHOLDER_SELECTOR;
use biome_css_syntax::T;
use biome_parser::SyntaxFeature;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

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
