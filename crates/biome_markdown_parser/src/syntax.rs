use biome_markdown_syntax::kind::MarkdownSyntaxKind::*;
use biome_parser::Parser;

use crate::MarkdownParser;

pub(crate) fn parse_root(p: &mut MarkdownParser) {
    let m = p.start();

    m.complete(p, MARKDOWN_DOCUMENT);
}
