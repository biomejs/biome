use crate::prelude::*;
use crate::utils::custom_property::CustomPropertyContainer;
use biome_css_syntax::CssCustomPropertyBracketedBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyBracketedBlock;
impl FormatNodeRule<CssCustomPropertyBracketedBlock> for FormatCssCustomPropertyBracketedBlock {
    fn fmt_fields(
        &self,
        node: &CssCustomPropertyBracketedBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        CustomPropertyContainer::from(node.clone()).fmt(f)
    }
}
