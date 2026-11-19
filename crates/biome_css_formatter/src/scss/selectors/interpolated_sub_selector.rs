use crate::prelude::*;
use biome_css_syntax::{ScssInterpolatedSubSelector, ScssInterpolatedSubSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedSubSelector;
impl FormatNodeRule<ScssInterpolatedSubSelector> for FormatScssInterpolatedSubSelector {
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedSubSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedSubSelectorFields { name } = node.as_fields();

        write!(f, [name.format()])
    }
}
