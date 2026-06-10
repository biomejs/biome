use crate::prelude::*;
use biome_css_syntax::CssComposesPropertyValueList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComposesPropertyValueList;
impl FormatRule<CssComposesPropertyValueList> for FormatCssComposesPropertyValueList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssComposesPropertyValueList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = hard_line_break();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
