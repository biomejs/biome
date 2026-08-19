use crate::prelude::*;
use crate::utils::custom_property::CustomPropertyContainer;
use biome_css_syntax::CssCustomPropertyParenthesizedBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyParenthesizedBlock;
impl FormatNodeRule<CssCustomPropertyParenthesizedBlock>
    for FormatCssCustomPropertyParenthesizedBlock
{
    fn fmt_fields(
        &self,
        node: &CssCustomPropertyParenthesizedBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        CustomPropertyContainer::from(node.clone()).fmt(f)
    }
}
