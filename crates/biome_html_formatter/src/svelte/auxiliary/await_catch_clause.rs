use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteAwaitCatchClause, SvelteAwaitCatchClauseFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteAwaitCatchClause;
impl FormatNodeRule<SvelteAwaitCatchClause> for FormatSvelteAwaitCatchClause {
    fn fmt_fields(&self, node: &SvelteAwaitCatchClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteAwaitCatchClauseFields { name, catch_token } = node.as_fields();

        write!(f, [catch_token.format(), space(), name.format()])
    }
}
