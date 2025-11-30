use crate::prelude::*;
use biome_css_syntax::{TwPluginAtRule, TwPluginAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwPluginAtRule;
impl FormatNodeRule<TwPluginAtRule> for FormatTwPluginAtRule {
    fn fmt_fields(&self, node: &TwPluginAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwPluginAtRuleFields {
            plugin_token,
            name,
            block,
            semicolon_token,
        } = node.as_fields();

        write!(f, [plugin_token.format(), space(), name.format()])?;
        if let Some(block) = block {
            write!(f, [space(), block.format()])?;
            if let Some(semicolon_token) = semicolon_token {
                write!(f, [format_removed(&semicolon_token)])?;
            }
        } else {
            write!(f, [semicolon_token.format()])?;
        }

        Ok(())
    }
}
