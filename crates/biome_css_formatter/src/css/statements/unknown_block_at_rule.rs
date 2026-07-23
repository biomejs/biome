use crate::prelude::*;
use crate::utils::case::unknown_at_rule_name_case;
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

        let name = name?;
        let name_case = unknown_at_rule_name_case(&name, components.as_ref().ok());

        write!(
            f,
            [
                name.format().with_text_case(name_case),
                space(),
                components.format()
            ]
        )?;

        if components.is_ok_and(|components| components.items().next().is_some()) {
            write!(f, [space()])?;
        }

        write!(f, [block.format()])
    }
}
