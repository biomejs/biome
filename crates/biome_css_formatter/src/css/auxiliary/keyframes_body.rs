use crate::prelude::*;
use biome_css_syntax::CssKeyframesBody;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesBody;
impl FormatNodeRule<CssKeyframesBody> for FormatCssKeyframesBody {
    fn fmt_fields(&self, node: &CssKeyframesBody, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
