use crate::prelude::*;
use crate::verbatim::format_css_verbatim_node;
use biome_css_syntax::ScssInterpolatedIdentifierHyphen;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedIdentifierHyphen;
impl FormatNodeRule<ScssInterpolatedIdentifierHyphen> for FormatScssInterpolatedIdentifierHyphen {
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedIdentifierHyphen,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
