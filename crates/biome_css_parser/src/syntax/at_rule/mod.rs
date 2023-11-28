mod charset;

use crate::parser::CssParser;
use crate::syntax::at_rule::charset::{is_at_charset_rule, parse_at_charset_rule};
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

#[inline]
pub(crate) fn at_at_rule(p: &mut CssParser) -> bool {
    p.at(T![@])
}

#[inline]
pub(crate) fn parse_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !at_at_rule(p) {
        return Absent;
    }

    if is_at_charset_rule(p) {
        parse_at_charset_rule(p)
    } else {
        Absent
    }
}
