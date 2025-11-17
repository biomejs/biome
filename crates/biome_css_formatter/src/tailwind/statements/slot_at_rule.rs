use crate::prelude::*;
use biome_css_syntax::{TwSlotAtRule, TwSlotAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwSlotAtRule;
impl FormatNodeRule<TwSlotAtRule> for FormatTwSlotAtRule {
    fn fmt_fields(&self, node: &TwSlotAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwSlotAtRuleFields {
            slot_token,
            semicolon_token,
        } = node.as_fields();

        write!(f, [slot_token.format(), semicolon_token.format()])
    }
}
