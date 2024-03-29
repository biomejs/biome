use crate::parser::HtmlParser;
use biome_html_syntax::HtmlSyntaxKind::{HTML_DIRECTIVE, HTML_ROOT, UNICODE_BOM};
use biome_html_syntax::T;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;
use biome_parser::Parser;

pub(crate) fn parse_root(p: &mut HtmlParser) {
    let m = p.start();

    p.eat(UNICODE_BOM);

    parse_doc_type(p).ok();

    m.complete(p, HTML_ROOT);
}

fn parse_doc_type(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![<]) {
        return Absent;
    }

    let m = p.start();
    p.eat(T![<]);
    p.bump(T![!]);

    if p.at(T![doctype]) {
        p.eat(T![doctype]);
    }

    p.eat(T![>]);

    ParsedSyntax::Present(m.complete(p, HTML_DIRECTIVE))
}
