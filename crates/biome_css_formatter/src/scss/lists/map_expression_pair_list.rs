use crate::prelude::*;
use biome_css_syntax::ScssMapExpressionPairList;
use biome_formatter::separated::TrailingSeparator;
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMapExpressionPairList;

impl FormatRule<ScssMapExpressionPairList> for FormatScssMapExpressionPairList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssMapExpressionPairList, f: &mut CssFormatter) -> FormatResult<()> {
        // `FormatScssMapExpression` prints the final comma so inline comments
        // like `a: b, /* end */)` stay after the comma.
        let separated = node
            .format_separated(",")
            .with_trailing_separator(TrailingSeparator::Omit);
        // Preserve source blank lines between pairs, e.g. comment-heavy maps.
        let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node.elements().zip(separated) {
            join.entry(element.node()?.syntax(), &formatted);
        }

        join.finish()
    }
}
