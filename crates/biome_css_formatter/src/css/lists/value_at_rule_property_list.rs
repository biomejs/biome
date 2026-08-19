use crate::prelude::*;
use biome_css_syntax::CssValueAtRulePropertyList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRulePropertyList;
impl FormatRule<CssValueAtRulePropertyList> for FormatCssValueAtRulePropertyList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssValueAtRulePropertyList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
