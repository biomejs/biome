use crate::prelude::*;
use biome_css_syntax::CssCustomPropertyValue;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyValue;
impl FormatNodeRule<CssCustomPropertyValue> for FormatCssCustomPropertyValue {
    fn fmt_fields(&self, node: &CssCustomPropertyValue, f: &mut CssFormatter) -> FormatResult<()> {
        node.components().format().fmt(f)
    }
}
