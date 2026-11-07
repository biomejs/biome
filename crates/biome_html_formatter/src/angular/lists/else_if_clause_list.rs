use crate::prelude::*;
use biome_html_syntax::AngularElseIfClauseList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularElseIfClauseList;
impl FormatRule<AngularElseIfClauseList> for FormatAngularElseIfClauseList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AngularElseIfClauseList, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
