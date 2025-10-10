use crate::prelude::*;
use biome_css_syntax::{TwSourceAtRule, TwSourceAtRuleFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwSourceAtRule;
impl FormatNodeRule<TwSourceAtRule> for FormatTwSourceAtRule {
    fn fmt_fields(&self, node: &TwSourceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwSourceAtRuleFields {
            source_token,
            path,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                source_token.format(),
                space(),
                path.format(),
                semicolon_token.format()
            ]
        )
    }
}
