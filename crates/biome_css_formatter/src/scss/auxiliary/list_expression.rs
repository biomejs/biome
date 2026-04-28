use crate::prelude::*;
use crate::utils::scss_list_layout::ScssListLayout;
use biome_css_syntax::ScssListExpression;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssListExpression;

impl FormatNodeRule<ScssListExpression> for FormatScssListExpression {
    fn fmt_fields(&self, node: &ScssListExpression, f: &mut CssFormatter) -> FormatResult<()> {
        ScssListLayout::new(node).fmt(f)
    }
}
