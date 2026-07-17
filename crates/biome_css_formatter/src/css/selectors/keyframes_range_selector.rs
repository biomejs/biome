use crate::prelude::*;
use biome_css_syntax::{CssKeyframesRangeSelector, CssKeyframesRangeSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesRangeSelector;
impl FormatNodeRule<CssKeyframesRangeSelector> for FormatCssKeyframesRangeSelector {
    fn fmt_fields(
        &self,
        node: &CssKeyframesRangeSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssKeyframesRangeSelectorFields { name, percentage } = node.as_fields();

        write!(f, [name.format(), space(), percentage.format()])
    }
}
