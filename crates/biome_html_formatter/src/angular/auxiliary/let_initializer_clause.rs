use crate::prelude::*;
use biome_html_syntax::AngularLetInitializerClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularLetInitializerClause;
impl FormatNodeRule<AngularLetInitializerClause> for FormatAngularLetInitializerClause {
    fn fmt_fields(
        &self,
        node: &AngularLetInitializerClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
