use crate::prelude::*;
use biome_css_syntax::{CssSupportsAtRuleDeclarator, CssSupportsAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsAtRuleDeclarator;
impl FormatNodeRule<CssSupportsAtRuleDeclarator> for FormatCssSupportsAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssSupportsAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssSupportsAtRuleDeclaratorFields {
            supports_token,
            condition,
        } = node.as_fields();

        write!(
            f,
            [
                supports_token.format(),
                space(),
                group(&indent(&condition.format())),
            ]
        )
    }
}
