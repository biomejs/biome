use crate::prelude::*;
use biome_css_syntax::{TwReferenceAtRule, TwReferenceAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwReferenceAtRule;
impl FormatNodeRule<TwReferenceAtRule> for FormatTwReferenceAtRule {
    fn fmt_fields(&self, node: &TwReferenceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwReferenceAtRuleFields {
            reference_token,
            path,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                reference_token.format(),
                space(),
                path.format(),
                semicolon_token.format()
            ]
        )
    }
}
