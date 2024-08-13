use crate::JsParser;
use crate::JsSyntaxKind::JS_METAVARIABLE;
use biome_parser::{prelude::ParsedSyntax, Parser};

#[inline]
pub(crate) fn is_at_metavariable(p: &mut JsParser) -> bool {
    is_nth_at_metavariable(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_metavariable(p: &mut JsParser, n: usize) -> bool {
    p.nth(n).is_metavariable()
}

#[inline]
pub(crate) fn parse_metavariable(p: &mut JsParser) -> ParsedSyntax {
    if is_at_metavariable(p) {
        let m = p.start();
        p.bump_any();
        ParsedSyntax::Present(m.complete(p, JS_METAVARIABLE))
    } else {
        ParsedSyntax::Absent
    }
}
