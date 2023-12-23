use crate::prelude::*;
use biome_css_syntax::{CssCustomProperty, CssCustomPropertyFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomProperty;
impl FormatNodeRule<CssCustomProperty> for FormatCssCustomProperty {
    fn fmt_fields(&self, node: &CssCustomProperty, f: &mut CssFormatter) -> FormatResult<()> {
        let CssCustomPropertyFields { value } = node.as_fields();

        write!(f, [value.format()])
    }
}
