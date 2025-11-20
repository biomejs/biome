use crate::parser::HtmlParser;
use crate::syntax::parse_error::expected_closed_fence;
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind::{
    ASTRO_EMBEDDED_CONTENT, ASTRO_FRONTMATTER_ELEMENT, FENCE, HTML_LITERAL,
};
use biome_html_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::Absent;

pub(crate) fn parse_astro_fence(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![---]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(FENCE, HtmlLexContext::AstroFencedCodeBlock);
    if p.at(T![<]) {
        p.error(expected_closed_fence(p, p.cur_range()));
        let c = m.complete(p, ASTRO_FRONTMATTER_ELEMENT);
        return ParsedSyntax::Present(c);
    }
    parse_astro_embedded(p).ok();
    p.expect(T![---]);

    let c = m.complete(p, ASTRO_FRONTMATTER_ELEMENT);
    ParsedSyntax::Present(c)
}

pub(crate) fn parse_astro_embedded(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(HTML_LITERAL) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(HTML_LITERAL, HtmlLexContext::AstroFencedCodeBlock);

    ParsedSyntax::Present(m.complete(p, ASTRO_EMBEDDED_CONTENT))
}
