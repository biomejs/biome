use crate::parser::CssParser;
use biome_parser::CompletedMarker;

use crate::syntax::block::{parse_declaration_or_rule_list_block, parse_rule_block};

///
/// Parses a conditional block from the current position in the CSS parser.
///
/// If the current state of the parser indicates that it's inside a nesting block (e.g., inside a style rule),
/// this function parses the content as a declaration or rule list block. Otherwise, it parses the content
/// as a rule block.
///
/// In addition to nested style rules, this specification allows nested group rules inside of style rules:
/// any at-rule whose body contains style rules can be nested inside of a style rule as well.
///
/// For more detailed information refer to the
/// [CSS Nesting Module](https://drafts.csswg.org/css-nesting-1/#conditionals)
///
#[inline]
pub(crate) fn parse_conditional_block(p: &mut CssParser) -> CompletedMarker {
    if p.state_mut().is_nesting_block {
        parse_declaration_or_rule_list_block(p)
    } else {
        parse_rule_block(p)
    }
}
