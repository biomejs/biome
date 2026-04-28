use crate::prelude::*;
use crate::utils::scss_separated_list::trailing_separator_for_node;
use biome_css_syntax::ScssListExpressionElementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssListExpressionElementList;
impl FormatRule<ScssListExpressionElementList> for FormatScssListExpressionElementList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &ScssListExpressionElementList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);
        let separated = node
            .format_separated(",")
            .with_trailing_separator(trailing_separator_for_node(node.syntax()));

        for formatted in separated {
            joiner.entry(&formatted);
        }
        joiner.finish()
    }
}
