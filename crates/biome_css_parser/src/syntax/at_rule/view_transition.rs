use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_block;
use biome_css_syntax::{CssSyntaxKind::*, T};
use biome_parser::{
    parsed_syntax::ParsedSyntax::{self, Present},
    prelude::ParsedSyntax::Absent,
    Parser,
};

#[inline]
pub(crate) fn is_at_view_transition_at_rule(p: &mut CssParser) -> bool {
    p.at(T![view_transition])
}

#[inline]
pub(crate) fn parse_view_transition_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_view_transition_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![view_transition]);

    let kind = if p.at(T!['{']) {
        parse_declaration_block(p);
        CSS_VIEW_TRANSITION_AT_RULE
    } else {
        CSS_BOGUS_AT_RULE
    };

    Present(m.complete(p, kind))
}
