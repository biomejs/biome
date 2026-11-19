use crate::prelude::*;
use crate::utils::case::query_feature_name_case;
use biome_css_syntax::{CssQueryFeatureReverseRange, CssQueryFeatureReverseRangeFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureReverseRange;
impl FormatNodeRule<CssQueryFeatureReverseRange> for FormatCssQueryFeatureReverseRange {
    fn fmt_fields(
        &self,
        node: &CssQueryFeatureReverseRange,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssQueryFeatureReverseRangeFields {
            left,
            comparison,
            right,
        } = node.as_fields();
        let right = right?;
        let case = query_feature_name_case(&right);

        write!(
            f,
            [
                left.format(),
                space(),
                comparison.format(),
                space(),
                right.format().with_text_case(case)
            ]
        )
    }
}
