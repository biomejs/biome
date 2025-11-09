use crate::prelude::*;
use biome_css_syntax::{CssPropertyAtRuleDeclarator, CssPropertyAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPropertyAtRuleDeclarator;

impl FormatNodeRule<CssPropertyAtRuleDeclarator> for FormatCssPropertyAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssPropertyAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPropertyAtRuleDeclaratorFields {
            property_token,
            name,
        } = node.as_fields();

        write!(f, [property_token.format(), space(), name.format()])
    }
}
