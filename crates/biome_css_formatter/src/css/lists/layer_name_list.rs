use crate::prelude::*;
use crate::separated::FormatAstSeparatedListWithScopedOptionsExtension;
use crate::utils::case::CssCase;
use biome_css_syntax::CssLayerNameList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLayerNameList;
impl FormatRule<CssLayerNameList> for FormatCssLayerNameList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssLayerNameList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join()
            .entries(node.format_separated_with_scoped_options(".", CssCase::Preserve))
            .finish()
    }
}
