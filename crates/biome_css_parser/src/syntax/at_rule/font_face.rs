use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_block;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

#[inline]
pub(crate) fn is_at_font_face_at_rule(p: &mut CssParser) -> bool {
    p.at(T![font_face])
}

#[inline]
pub(crate) fn parse_font_face_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_font_face_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![font_face]);

    parse_declaration_block(p);

    Present(m.complete(p, CSS_FONT_FACE_AT_RULE))
}
