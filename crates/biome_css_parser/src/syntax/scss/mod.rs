mod declaration;

use crate::parser::CssParser;
use crate::syntax::{is_nth_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::SCSS_IDENTIFIER;
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

pub(crate) use declaration::{is_at_scss_declaration, parse_scss_declaration};

#[inline]
pub(crate) fn is_at_scss_identifier(p: &mut CssParser) -> bool {
    p.at(T![$]) && is_nth_at_identifier(p, 1)
}

#[inline]
pub(crate) fn parse_scss_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![$]);
    parse_regular_identifier(p).ok();
    Present(m.complete(p, SCSS_IDENTIFIER))
}
