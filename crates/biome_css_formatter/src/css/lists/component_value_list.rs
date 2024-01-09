use crate::prelude::*;
use crate::utils::component_value_list::write_component_value_list;
use biome_css_syntax::CssComponentValueList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComponentValueList;
impl FormatRule<CssComponentValueList> for FormatCssComponentValueList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &CssComponentValueList, f: &mut CssFormatter) -> FormatResult<()> {
        write_component_value_list(node, f)
    }
}
