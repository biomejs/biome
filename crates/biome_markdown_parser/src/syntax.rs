use biome_demo_syntax::kind::MarkdownSyntaxKind::ROOT;
use biome_parser::Parser;

use crate::DemoParser;

pub(crate) fn parse_root(p: &mut DemoParser) {
    let m = p.start();

    m.complete(p, ROOT);
}
