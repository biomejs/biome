use crate::prelude::*;
use biome_css_syntax::{CssContainerAtRuleDeclarator, CssContainerAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerAtRuleDeclarator;

impl FormatNodeRule<CssContainerAtRuleDeclarator> for FormatCssContainerAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssContainerAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerAtRuleDeclaratorFields {
            container_token,
            name,
            query,
        } = node.as_fields();

        write!(f, [container_token.format(), space()])?;

        if name.is_some() {
            write!(f, [name.format(), space()])?;
        }

        write!(f, [query.format()])
    }
}
