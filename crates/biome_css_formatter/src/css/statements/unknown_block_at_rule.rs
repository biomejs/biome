use crate::prelude::*;
use biome_css_syntax::{CssUnknownBlockAtRule, CssUnknownBlockAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownBlockAtRule;
impl FormatNodeRule<CssUnknownBlockAtRule> for FormatCssUnknownBlockAtRule {
    fn fmt_fields(&self, node: &CssUnknownBlockAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUnknownBlockAtRuleFields {
            name,
            components,
            block,
        } = node.as_fields();

        write!(f, [name.format(), space(), components.format()])?;

        if components.is_ok_and(|components| components.items().next().is_some()) {
            write!(f, [space()])?;
        }

        write!(f, [block.format()])
    }
}
