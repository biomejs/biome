use crate::prelude::*;
use biome_css_syntax::{CssColorProfileAtRuleDeclarator, CssColorProfileAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssColorProfileAtRuleDeclarator;

impl FormatNodeRule<CssColorProfileAtRuleDeclarator> for FormatCssColorProfileAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssColorProfileAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssColorProfileAtRuleDeclaratorFields {
            color_profile_token,
            name,
        } = node.as_fields();

        write!(f, [color_profile_token.format(), space(), name.format()])
    }
}
