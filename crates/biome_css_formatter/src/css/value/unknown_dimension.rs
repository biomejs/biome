use crate::prelude::*;
use biome_css_syntax::{CssUnknownDimension, CssUnknownDimensionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownDimension;
impl FormatNodeRule<CssUnknownDimension> for FormatCssUnknownDimension {
    fn fmt_fields(&self, node: &CssUnknownDimension, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUnknownDimensionFields { value, unit } = node.as_fields();

        write!(f, [value.format(), unit.format()])
    }
}
