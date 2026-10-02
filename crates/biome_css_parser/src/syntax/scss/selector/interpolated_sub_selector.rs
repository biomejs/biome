use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::scss::{is_at_scss_interpolation, parse_scss_selector_custom_identifier};
use biome_css_syntax::CssSyntaxKind::SCSS_INTERPOLATED_SUB_SELECTOR;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

#[inline]
pub(crate) fn parse_scss_interpolated_sub_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolation(p) {
        return Absent;
    }

    let m = p.start();
    parse_scss_selector_custom_identifier(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, SCSS_INTERPOLATED_SUB_SELECTOR))
}
