use crate::prelude::*;
use biome_css_syntax::{CssKeyframesPercentageSelector, CssKeyframesPercentageSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesPercentageSelector;
impl FormatNodeRule<CssKeyframesPercentageSelector> for FormatCssKeyframesPercentageSelector {
    fn fmt_fields(
        &self,
        node: &CssKeyframesPercentageSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssKeyframesPercentageSelectorFields { selector } = node.as_fields();

        write!(f, [selector.format()])
    }
}
