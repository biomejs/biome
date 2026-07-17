use crate::prelude::*;
use crate::utils::scss_keyword_argument_layout::ScssKeywordArgumentLayout;
use crate::utils::scss_separator_comments::FormatScssSeparatorComments;
use biome_css_syntax::ScssKeywordArgument;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssKeywordArgument;
impl FormatNodeRule<ScssKeywordArgument> for FormatScssKeywordArgument {
    fn fmt_node(&self, node: &ScssKeywordArgument, f: &mut CssFormatter) -> FormatResult<()> {
        self.fmt_node_with_scss_separator_comments(node, f)
    }

    fn fmt_fields(&self, node: &ScssKeywordArgument, f: &mut CssFormatter) -> FormatResult<()> {
        ScssKeywordArgumentLayout::new(node).fmt(f)
    }

    fn fmt_leading_comments(
        &self,
        node: &ScssKeywordArgument,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        self.fmt_leading_scss_separator_comments(node, f)
    }

    fn fmt_dangling_comments(
        &self,
        node: &ScssKeywordArgument,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        if f.comments().has_dangling_comments(node.syntax()) {
            Ok(())
        } else {
            format_dangling_comments(node.syntax()).fmt(f)
        }
    }
}
