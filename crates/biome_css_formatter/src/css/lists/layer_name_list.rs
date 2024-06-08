use crate::css::value::identifier::FormatCssIdentifierOptions;
use crate::prelude::*;
use crate::separated::FormatAstSeparatedListWithOptionsExtension;
use biome_css_syntax::CssLayerNameList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLayerNameList;
impl FormatRule<CssLayerNameList> for FormatCssLayerNameList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssLayerNameList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join()
            .entries(node.format_separated_with_options(
                ".",
                FormatCssIdentifierOptions::default().prevent_lowercase(true),
            ))
            .finish()
    }
}
