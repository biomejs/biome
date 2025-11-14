use crate::prelude::*;
use biome_css_syntax::{CssCounterStyleAtRuleDeclarator, CssCounterStyleAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCounterStyleAtRuleDeclarator;

impl FormatNodeRule<CssCounterStyleAtRuleDeclarator> for FormatCssCounterStyleAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssCounterStyleAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssCounterStyleAtRuleDeclaratorFields {
            counter_style_token,
            name,
        } = node.as_fields();

        write!(f, [counter_style_token.format(), space(), name.format()])
    }
}
