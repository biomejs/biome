use crate::prelude::*;
use biome_css_syntax::{ScssInterpolatedNthValue, ScssInterpolatedNthValueFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedNthValue;
impl FormatNodeRule<ScssInterpolatedNthValue> for FormatScssInterpolatedNthValue {
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedNthValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedNthValueFields { items } = node.as_fields();

        write!(f, [items.format()])
    }
}
