use crate::prelude::*;
use biome_css_syntax::{CssRule, CssRuleFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRule;
impl FormatNodeRule<CssRule> for FormatCssRule {
    fn fmt_fields(&self, node: &CssRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRuleFields { prelude, block } = node.as_fields();

        write!(
            f,
            [
                // The selector list gets expanded so that every selector
                // appears on its own line, no matter how long they are.
                group(&prelude.format()).should_expand(true),
                space(),
                &block?.format()
            ]
        )
    }
}
