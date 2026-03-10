use crate::prelude::*;
use crate::utils::component_value_list::write_component_value_list;
use biome_css_syntax::ScssExpressionItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssExpressionItemList;
impl FormatRule<ScssExpressionItemList> for FormatScssExpressionItemList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &ScssExpressionItemList, f: &mut CssFormatter) -> FormatResult<()> {
        write_component_value_list(node, f)
    }
}
