use crate::prelude::*;
use biome_css_syntax::CssMediaQueryList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaQueryList;
impl FormatRule<CssMediaQueryList> for FormatCssMediaQueryList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssMediaQueryList, f: &mut CssFormatter) -> FormatResult<()> {
        f.fill()
            .entries(&soft_line_break_or_space(), node.format_separated(","))
            .finish()
    }
}
