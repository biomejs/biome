use crate::prelude::*;
use biome_css_syntax::{CssCustomMediaAtRuleDeclarator, CssCustomMediaAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomMediaAtRuleDeclarator;

impl FormatNodeRule<CssCustomMediaAtRuleDeclarator> for FormatCssCustomMediaAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssCustomMediaAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssCustomMediaAtRuleDeclaratorFields {
            custom_media_token,
            name,
            queries,
        } = node.as_fields();

        write!(
            f,
            [
                custom_media_token
                    .format()?
                    .with_text_case(CssCase::Lowercase),
                space(),
                name.format(),
                space(),
                group(&indent(&queries.format())),
            ]
        )
    }
}
