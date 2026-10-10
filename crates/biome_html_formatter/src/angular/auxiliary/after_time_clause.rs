use crate::prelude::*;
use biome_html_syntax::AngularAfterTimeClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularAfterTimeClause;
impl FormatNodeRule<AngularAfterTimeClause> for FormatAngularAfterTimeClause {
    fn fmt_fields(&self, node: &AngularAfterTimeClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
