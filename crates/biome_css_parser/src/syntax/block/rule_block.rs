use crate::parser::CssParser;
use crate::syntax::{RuleList, is_at_rule_list_element};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::{CompletedMarker, Parser};

use crate::syntax::block::ParseBlockBody;

#[inline]
pub(crate) fn parse_rule_block(p: &mut CssParser) -> CompletedMarker {
    RuleBlock.parse_block_body(p)
}

struct RuleBlock;

impl ParseBlockBody for RuleBlock {
    const BLOCK_KIND: CssSyntaxKind = CSS_RULE_BLOCK;

    fn is_at_element(&self, p: &mut CssParser) -> bool {
        // A semicolon cannot supply the body when the opening brace is missing.
        !p.at(T![;]) && is_at_rule_list_element(p)
    }

    fn parse_list(&mut self, p: &mut CssParser) {
        RuleList::new(T!['}']).parse_list(p);
    }
}
