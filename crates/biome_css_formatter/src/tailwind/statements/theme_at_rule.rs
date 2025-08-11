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

        write!(
            f,
            [
                theme_token.format(),
                name.format().map(|name| format_args![space(), name]),
                space(),
                block.format()
            ]
        )
    }
}
