use crate::prelude::*;
use crate::utils::case::query_feature_name_case;
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
        let left = left?;
        let case = query_feature_name_case(&left);

        write!(
            f,
            [
                left.format().with_text_case(case),
                space(),
                comparison.format(),
                space(),
                right.format()
            ]
        )
    }
}
