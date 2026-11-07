use crate::prelude::*;
use biome_html_syntax::AngularLoadingParameterClauseList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularLoadingParameterClauseList;
impl FormatRule<AngularLoadingParameterClauseList> for FormatAngularLoadingParameterClauseList {
    type Context = HtmlFormatContext;
    fn fmt(
        &self,
        node: &AngularLoadingParameterClauseList,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
