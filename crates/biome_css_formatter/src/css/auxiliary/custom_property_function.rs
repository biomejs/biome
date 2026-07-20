use crate::prelude::*;
use crate::utils::custom_property::CustomPropertyContainer;
use biome_css_syntax::CssCustomPropertyFunction;
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyFunction;
impl FormatNodeRule<CssCustomPropertyFunction> for FormatCssCustomPropertyFunction {
    fn fmt_fields(
        &self,
        node: &CssCustomPropertyFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        write!(
            f,
            [
                node.name()?.format(),
                CustomPropertyContainer::from(node.clone())
            ]
        )
    }
}
