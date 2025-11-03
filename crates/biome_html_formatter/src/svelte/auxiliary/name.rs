use crate::prelude::*;
use biome_html_syntax::SvelteName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteName;
impl FormatNodeRule<SvelteName> for FormatSvelteName {
    fn fmt_fields(&self, node: &SvelteName, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
