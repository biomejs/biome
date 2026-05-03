use crate::prelude::*;
use crate::utils::component_value_list::write_component_value_list;
use biome_css_syntax::CssBracketedValueList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBracketedValueList;
impl FormatRule<CssBracketedValueList> for FormatCssBracketedValueList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssBracketedValueList, f: &mut CssFormatter) -> FormatResult<()> {
        write_component_value_list(node, f)
    }
}
