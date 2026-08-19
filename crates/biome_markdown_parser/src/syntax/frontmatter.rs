use crate::MarkdownParser;
use crate::lexer::MarkdownLexContext;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_markdown_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};

pub(crate) fn parse_frontmatter(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_frontmatter(p) {
        return Absent;
    }

    let frontmatter = p.start();
    p.bump_remap_with_context(T![---], MarkdownLexContext::Frontmatter);

    let content = p.start();
    p.bump_with_context(MD_FRONTMATTER_LITERAL, MarkdownLexContext::Frontmatter);
    content.complete(p, MD_FRONTMATTER_CONTENT);

    p.bump(T![---]);
    Present(frontmatter.complete(p, MD_FRONTMATTER))
}

fn at_frontmatter(p: &MarkdownParser) -> bool {
    p.at(MD_THEMATIC_BREAK_LITERAL)
        && p.cur_text().trim_end_matches([' ', '\t']) == "---"
        && p.has_frontmatter_closing_fence()
}
