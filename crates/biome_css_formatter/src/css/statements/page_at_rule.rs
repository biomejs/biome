use crate::prelude::*;
use biome_css_syntax::{CssPageAtRule, CssPageAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageAtRule;
impl FormatNodeRule<CssPageAtRule> for FormatCssPageAtRule {
    fn fmt_fields(&self, node: &CssPageAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssPageAtRuleFields {
            page_token,
            selectors,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                page_token.format(),
                space(),
                group(&indent(&selectors.format())),
                space(),
                block.format()
            ]
        )
    }
}
