use crate::prelude::*;
use biome_css_syntax::{CssQueryFeatureRangeComparison, CssQueryFeatureRangeComparisonFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureRangeComparison;
impl FormatNodeRule<CssQueryFeatureRangeComparison> for FormatCssQueryFeatureRangeComparison {
    fn fmt_fields(
        &self,
        node: &CssQueryFeatureRangeComparison,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssQueryFeatureRangeComparisonFields { operator } = node.as_fields();

        write!(f, [operator.format()])
    }
}
