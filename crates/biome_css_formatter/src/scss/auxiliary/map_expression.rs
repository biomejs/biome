use crate::prelude::*;
use crate::utils::scss_map_layout::ScssMapLayout;
use biome_css_syntax::ScssMapExpression;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMapExpression;
impl FormatNodeRule<ScssMapExpression> for FormatScssMapExpression {
    fn fmt_fields(&self, node: &ScssMapExpression, f: &mut CssFormatter) -> FormatResult<()> {
        ScssMapLayout::new(node).fmt(f)
    }

    fn fmt_dangling_comments(
        &self,
        node: &ScssMapExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        if ScssMapLayout::new(node).owns_dangling_comments(f) {
            Ok(())
        } else {
            format_dangling_comments(node.syntax())
                .with_soft_block_indent()
                .fmt(f)
        }
    }
}
