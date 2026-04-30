use crate::prelude::*;
use biome_css_syntax::ScssListExpressionElementList;
use biome_formatter::separated::TrailingSeparator;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssListExpressionElementList;

impl FormatRule<ScssListExpressionElementList> for FormatScssListExpressionElementList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssListExpressionElementList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);
        let separated = node
            .format_separated(",")
            .with_trailing_separator(TrailingSeparator::Omit);

        for formatted in separated {
            joiner.entry(&formatted);
        }
        joiner.finish()
    }
}
