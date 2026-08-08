use crate::prelude::*;
use biome_html_syntax::AngularDeferClauseList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDeferClauseList;
impl FormatRule<AngularDeferClauseList> for FormatAngularDeferClauseList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AngularDeferClauseList, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
