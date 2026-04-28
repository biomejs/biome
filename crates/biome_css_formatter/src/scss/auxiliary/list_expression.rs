use crate::prelude::*;
use crate::utils::scss_list_layout::ScssListLayout;
use crate::utils::scss_separator_comments::ScssSeparatorComments;
use biome_css_syntax::ScssListExpression;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssListExpression;

impl FormatNodeRule<ScssListExpression> for FormatScssListExpression {
    fn fmt_node(&self, node: &ScssListExpression, f: &mut CssFormatter) -> FormatResult<()> {
        ScssSeparatorComments::around(node.syntax()).fmt_node(f, |f| self.fmt_fields(node, f))
    }

    fn fmt_fields(&self, node: &ScssListExpression, f: &mut CssFormatter) -> FormatResult<()> {
        ScssListLayout::new(node).fmt(f)
    }

    fn fmt_leading_comments(
        &self,
        node: &ScssListExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        ScssSeparatorComments::around(node.syntax()).fmt_leading_comments(f)
    }

    fn fmt_dangling_comments(
        &self,
        node: &ScssListExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Include-owned closing comments are printed inside the list layout so
        // `@include mix((a, b, /* end */))` stays parse-safe.
        if ScssListLayout::new(node).owns_dangling_comments(f) {
            Ok(())
        } else {
            write!(
                f,
                [format_dangling_comments(node.syntax()).with_soft_block_indent()]
            )
        }
    }
}
