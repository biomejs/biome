use crate::prelude::*;
use biome_html_syntax::AngularForTrackClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularForTrackClause;
impl FormatNodeRule<AngularForTrackClause> for FormatAngularForTrackClause {
    fn fmt_fields(&self, node: &AngularForTrackClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
