use crate::parser::CssParser;
use crate::syntax::{is_at_dashed_identifier, parse_dashed_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};

/// Parses theme references: --tab-size-*
pub(crate) fn parse_tailwind_value_theme_reference(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_dashed_identifier(p) {
        return Absent;
    }

    let m = p.start();

    parse_dashed_identifier(p).ok();
    p.expect(T![-]);
    p.expect(T![*]);

    Present(m.complete(p, TW_VALUE_THEME_REFERENCE))
}
