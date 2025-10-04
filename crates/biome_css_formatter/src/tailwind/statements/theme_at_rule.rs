use crate::prelude::*;
use biome_css_syntax::{TwThemeAtRule, TwThemeAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwThemeAtRule;
impl FormatNodeRule<TwThemeAtRule> for FormatTwThemeAtRule {
    fn fmt_fields(&self, node: &TwThemeAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwThemeAtRuleFields {
            theme_token,
            name,
            block,
        } = node.as_fields();

        write!(f, [theme_token.format()])?;
        if let Some(name) = name {
            write!(f, [space(), name.format()])?;
        }
        write!(f, [space(), block.format()])
    }
}
