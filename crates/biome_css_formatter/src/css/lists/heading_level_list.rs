use crate::prelude::*;
use biome_css_syntax::CssHeadingLevelList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssHeadingLevelList;
impl FormatRule<CssHeadingLevelList> for FormatCssHeadingLevelList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssHeadingLevelList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
        f.join_with(&separator)
            .entries(node.format_separated(","))
            .finish()
    }
}
