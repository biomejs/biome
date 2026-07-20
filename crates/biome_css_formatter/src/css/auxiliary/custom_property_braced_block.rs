use crate::prelude::*;
use crate::utils::custom_property::CustomPropertyContainer;
use biome_css_syntax::CssCustomPropertyBracedBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyBracedBlock;
impl FormatNodeRule<CssCustomPropertyBracedBlock> for FormatCssCustomPropertyBracedBlock {
    fn fmt_fields(
        &self,
        node: &CssCustomPropertyBracedBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        CustomPropertyContainer::from(node.clone()).fmt(f)
    }
}
