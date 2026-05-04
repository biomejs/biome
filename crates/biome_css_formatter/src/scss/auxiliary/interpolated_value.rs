use crate::prelude::*;
use biome_css_syntax::ScssInterpolatedValue;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedValue;

impl FormatNodeRule<ScssInterpolatedValue> for FormatScssInterpolatedValue {
    fn fmt_fields(&self, node: &ScssInterpolatedValue, f: &mut CssFormatter) -> FormatResult<()> {
        node.items().format().fmt(f)
    }
}
