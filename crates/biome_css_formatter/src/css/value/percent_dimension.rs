use crate::prelude::*;
use biome_css_syntax::{CssPercentDimension, CssPercentDimensionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPercentDimension;
impl FormatNodeRule<CssPercentDimension> for FormatCssPercentDimension {
    fn fmt_fields(&self, node: &CssPercentDimension, f: &mut CssFormatter) -> FormatResult<()> {
        let CssPercentDimensionFields { value, unit_token } = node.as_fields();

        write!(f, [value.format(), unit_token.format()])
    }
}
