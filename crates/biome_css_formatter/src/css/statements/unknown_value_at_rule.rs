use crate::prelude::*;
use biome_css_syntax::{CssUnknownValueAtRule, CssUnknownValueAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownValueAtRule;
impl FormatNodeRule<CssUnknownValueAtRule> for FormatCssUnknownValueAtRule {
    fn fmt_fields(&self, node: &CssUnknownValueAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUnknownValueAtRuleFields {
            name,
            parameters,
            semicolon_token,
        } = node.as_fields();

        write!(f, [name.format()])?;

        if let Ok(parameters) = parameters {
            if parameters.items().next().is_some() {
                write!(f, [space()])?;
            }
            write!(f, [parameters.format()])?;
        }

        write!(f, [semicolon_token.format()])
    }
}
