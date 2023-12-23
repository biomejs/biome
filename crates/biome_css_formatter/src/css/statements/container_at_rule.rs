use crate::prelude::*;
use biome_css_syntax::{CssContainerAtRule, CssContainerAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerAtRule;
impl FormatNodeRule<CssContainerAtRule> for FormatCssContainerAtRule {
    fn fmt_fields(&self, node: &CssContainerAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssContainerAtRuleFields {
            container_token,
            name,
            query,
            block,
        } = node.as_fields();

        write!(f, [container_token.format(), space()])?;

        if name.is_some() {
            write!(f, [name.format(), space()])?;
        }

        write!(f, [query.format(), space(), block.format()])
    }
}
