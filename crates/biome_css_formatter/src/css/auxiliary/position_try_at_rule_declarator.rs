use crate::prelude::*;
use biome_css_syntax::{CssPositionTryAtRuleDeclarator, CssPositionTryAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPositionTryAtRuleDeclarator;

impl FormatNodeRule<CssPositionTryAtRuleDeclarator> for FormatCssPositionTryAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssPositionTryAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPositionTryAtRuleDeclaratorFields {
            position_try_token,
            name,
        } = node.as_fields();

        write!(f, [position_try_token.format(), space(), name.format()])
    }
}
