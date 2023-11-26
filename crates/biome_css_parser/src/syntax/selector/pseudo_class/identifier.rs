use crate::parser::CssParser;
use crate::syntax::is_at_identifier;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::selector::parse_selector_identifier;
use biome_css_syntax::CssSyntaxKind::*;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::Parser;

#[inline]
pub(crate) fn parse_pseudo_class_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    parse_selector_identifier(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, CSS_PSEUDO_CLASS_IDENTIFIER))
}
