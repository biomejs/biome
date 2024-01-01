use crate::prelude::*;
use biome_css_syntax::{CssQueryFeatureRange, CssQueryFeatureRangeFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureRange;
impl FormatNodeRule<CssQueryFeatureRange> for FormatCssQueryFeatureRange {
    fn fmt_fields(&self, node: &CssQueryFeatureRange, f: &mut CssFormatter) -> FormatResult<()> {
        let CssQueryFeatureRangeFields {
            left,
            comparison,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                comparison.format(),
                space(),
                right.format()
            ]
        )
    }
}
