use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::*;
use rome_parser::Parser;

pub(crate) fn parse_root(p: &mut CssParser) {
    let m = p.start();

    let rules = p.start();

    rules.complete(p, CSS_RULE_LIST);

    m.complete(p, CSS_ROOT);
}
