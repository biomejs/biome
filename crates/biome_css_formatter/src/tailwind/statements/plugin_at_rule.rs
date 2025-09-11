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
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                plugin_token.format(),
                space(),
                name.format(),
                semicolon_token.format()
            ]
        )
    }
}
