use crate::prelude::*;
use crate::shared::FormatSvelteAwaitBinding;
use biome_html_syntax::{SvelteAwaitCatchClause, SvelteAwaitCatchClauseFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteAwaitCatchClause;
impl FormatNodeRule<SvelteAwaitCatchClause> for FormatSvelteAwaitCatchClause {
    fn fmt_fields(&self, node: &SvelteAwaitCatchClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteAwaitCatchClauseFields { name, catch_token } = node.as_fields();

        FormatSvelteAwaitBinding {
            keyword: catch_token?,
            name,
        }
        .fmt(f)
    }
}
