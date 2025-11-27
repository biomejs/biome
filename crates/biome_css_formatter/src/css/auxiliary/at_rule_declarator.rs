use crate::prelude::*;
use biome_css_syntax::{CssAtRuleDeclarator, CssAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAtRuleDeclarator;
impl FormatNodeRule<CssAtRuleDeclarator> for FormatCssAtRuleDeclarator {
    fn fmt_fields(&self, node: &CssAtRuleDeclarator, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAtRuleDeclaratorFields {
            at_token,
            declarator,
        } = node.as_fields();

        write!(f, [at_token.format(), declarator.format()])
    }
}
