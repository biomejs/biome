use crate::prelude::*;
use biome_html_syntax::SvelteAwaitClausesList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteAwaitClausesList;
impl FormatRule<SvelteAwaitClausesList> for FormatSvelteAwaitClausesList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &SvelteAwaitClausesList, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
