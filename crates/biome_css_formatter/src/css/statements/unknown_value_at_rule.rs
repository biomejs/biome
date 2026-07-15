use crate::prelude::*;
use crate::utils::case::unknown_at_rule_name_case;
use biome_css_syntax::{CssUnknownValueAtRule, CssUnknownValueAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownValueAtRule;
impl FormatNodeRule<CssUnknownValueAtRule> for FormatCssUnknownValueAtRule {
    fn fmt_fields(&self, node: &CssUnknownValueAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUnknownValueAtRuleFields {
            name,
            components,
            semicolon_token,
        } = node.as_fields();

        let name = name?;
        let name_case = unknown_at_rule_name_case(&name, components.as_ref().ok());

        write!(f, [name.format().with_text_case(name_case)])?;

        if let Ok(components) = components {
            if components.items().next().is_some() {
                write!(f, [space()])?;
            }
            write!(f, [components.format()])?;
        }

        write!(f, [semicolon_token.format()])
    }
}
