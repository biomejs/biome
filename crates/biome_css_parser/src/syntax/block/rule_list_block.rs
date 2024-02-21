use crate::parser::CssParser;
use crate::syntax::{is_at_rule_list_element, RuleList};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::CompletedMarker;

use crate::syntax::block::ParseBlockBody;

struct RuleListBlock;

impl ParseBlockBody for RuleListBlock {
    const BLOCK_KIND: CssSyntaxKind = CSS_RULE_LIST_BLOCK;

    fn is_at_element(&self, p: &mut CssParser) -> bool {
        is_at_rule_list_element(p)
    }

    fn parse_list(&mut self, p: &mut CssParser) {
        RuleList::new(T!['}']).parse_list(p);
    }
}

#[inline]
pub(crate) fn parse_rule_list_block(p: &mut CssParser) -> CompletedMarker {
    RuleListBlock.parse_block_body(p)
}
