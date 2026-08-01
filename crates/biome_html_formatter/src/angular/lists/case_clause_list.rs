use crate::prelude::*;
use biome_html_syntax::AngularCaseClauseList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularCaseClauseList;
impl FormatRule<AngularCaseClauseList> for FormatAngularCaseClauseList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AngularCaseClauseList, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
