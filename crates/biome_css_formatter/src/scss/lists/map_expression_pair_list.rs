use crate::prelude::*;
use biome_css_syntax::ScssMapExpressionPairList;
use biome_formatter::separated::TrailingSeparator;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMapExpressionPairList;

impl FormatRule<ScssMapExpressionPairList> for FormatScssMapExpressionPairList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssMapExpressionPairList, f: &mut CssFormatter) -> FormatResult<()> {
        // `FormatScssMapExpression` prints the final comma so inline comments
        // like `a: b, /* end */)` stay after the comma.
        let mut separated = node
            .format_separated(",")
            .with_trailing_separator(TrailingSeparator::Omit);
        f.join_with(&soft_line_break_or_space())
            .entries(&mut separated)
            .finish()
    }
}
