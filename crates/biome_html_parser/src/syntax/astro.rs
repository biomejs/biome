use crate::parser::HtmlParser;
use crate::syntax::parse_error::expected_closed_fence;
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind::{FENCE, HTML_ASTRO_FRONTMATTER_ELEMENT, HTML_LITERAL};
use biome_html_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::Absent;

pub(crate) fn parse_astro_fence(p: &mut HtmlParser) -> ParsedSyntax {
    if p.at(HTML_LITERAL) && p.cur_text() == "---" {
        let m = p.start();
        p.bump_remap_with_context(FENCE, HtmlLexContext::AstroFencedCodeBlock);
        if p.at(T![<]) {
            p.error(expected_closed_fence(p, p.cur_range()));
            let c = m.complete(p, HTML_ASTRO_FRONTMATTER_ELEMENT);
            return ParsedSyntax::Present(c);
        }
        p.bump_with_context(HTML_LITERAL, HtmlLexContext::AstroFencedCodeBlock);
        p.expect_with_context(T![---], HtmlLexContext::AstroFencedCodeBlock);

        let c = m.complete(p, HTML_ASTRO_FRONTMATTER_ELEMENT);
        return ParsedSyntax::Present(c);
    }
    Absent
}

#[inline]
pub(crate) fn is_at_fence(p: &mut HtmlParser) -> bool {
    p.file_source().is_astro() && p.at(HTML_LITERAL) && p.cur_text() == "---"
}
