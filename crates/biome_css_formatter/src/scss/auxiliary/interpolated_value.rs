use crate::prelude::*;
use biome_css_syntax::{ScssInterpolatedValue, ScssInterpolatedValueFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedValue;

impl FormatNodeRule<ScssInterpolatedValue> for FormatScssInterpolatedValue {
    fn fmt_fields(&self, node: &ScssInterpolatedValue, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssInterpolatedValueFields { items } = node.as_fields();

        write!(f, [items.format()])
    }
}
