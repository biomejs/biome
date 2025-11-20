use crate::prelude::*;
use biome_html_syntax::SvelteElseIfClauseList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteElseIfClauseList;
impl FormatRule<SvelteElseIfClauseList> for FormatSvelteElseIfClauseList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &SvelteElseIfClauseList, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
