use crate::prelude::*;
use biome_html_syntax::AngularDeferOnClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDeferOnClause;
impl FormatNodeRule<AngularDeferOnClause> for FormatAngularDeferOnClause {
    fn fmt_fields(&self, node: &AngularDeferOnClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
