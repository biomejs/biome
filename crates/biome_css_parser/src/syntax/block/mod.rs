mod conditional_block;
mod declaration_block;
mod declaration_or_at_rule_list_block;
mod declaration_or_rule_list_block;
mod rule_block;

use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::diagnostic::{expected_node, ParseDiagnostic};
use biome_parser::{CompletedMarker, Parser};
use biome_rowan::TextRange;

pub(crate) use conditional_block::parse_conditional_block;
pub(crate) use declaration_block::parse_declaration_block;
pub(crate) use declaration_or_at_rule_list_block::parse_declaration_or_at_rule_list_block;
pub(crate) use declaration_or_rule_list_block::parse_declaration_or_rule_list_block;
pub(crate) use rule_block::parse_rule_block;

pub(crate) trait ParseBlockBody {
    const BLOCK_KIND: CssSyntaxKind;

    /// If the '{' is missing try we need to recover:
    ///  - Try to check if the next item is an item of a list. If it is, we can parse the list.
    ///  - If the next item is not a list item, we return bogus block and skip the list parsing.
    fn is_at_element(&self, p: &mut CssParser) -> bool;

    fn parse_list(&mut self, p: &mut CssParser);

    /// Parses the body of a block in CSS.
    ///
    /// This function handles the parsing of a block's content, delimited by curly braces `{}`.
    /// It temporarily sets the parser's state to indicate it is within a nesting block and then
    /// processes the content of the block using the provided callback function.
    fn parse_block_body(&mut self, p: &mut CssParser) -> CompletedMarker {
        let m = p.start();

        let is_open_brace_missing = !p.expect(T!['{']);

        if is_open_brace_missing && (!self.is_at_element(p) || p.state().speculative_parsing) {
            p.error(expected_block(p, p.cur_range()));
            return m.complete(p, CSS_BOGUS_BLOCK);
        }

        let old_nesting_block = std::mem::replace(&mut p.state_mut().is_nesting_block, true);

        self.parse_list(p);

        let is_close_brace_missing = !p.expect(T!['}']);

        p.state_mut().is_nesting_block = old_nesting_block;

        let kind = if is_open_brace_missing || is_close_brace_missing {
            CSS_BOGUS_BLOCK
        } else {
            Self::BLOCK_KIND
        };

        m.complete(p, kind)
    }
}

pub(crate) fn expected_block(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("body", range, p)
}
