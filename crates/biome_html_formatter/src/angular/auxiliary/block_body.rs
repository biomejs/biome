use crate::prelude::*;
use biome_html_syntax::AngularBlockBody;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularBlockBody;
impl FormatNodeRule<AngularBlockBody> for FormatAngularBlockBody {
    fn fmt_fields(&self, node: &AngularBlockBody, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
