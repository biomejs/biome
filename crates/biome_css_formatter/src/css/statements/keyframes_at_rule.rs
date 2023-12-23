use crate::prelude::*;
use biome_css_syntax::{CssKeyframesAtRule, CssKeyframesAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesAtRule;
impl FormatNodeRule<CssKeyframesAtRule> for FormatCssKeyframesAtRule {
    fn fmt_fields(&self, node: &CssKeyframesAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssKeyframesAtRuleFields {
            keyframes_token,
            name,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                keyframes_token.format(),
                space(),
                name.format(),
                space(),
                block.format()
            ]
        )
    }
}
