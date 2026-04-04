use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteAwaitThenClause, SvelteAwaitThenClauseFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteAwaitThenClause;
impl FormatNodeRule<SvelteAwaitThenClause> for FormatSvelteAwaitThenClause {
    fn fmt_fields(&self, node: &SvelteAwaitThenClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteAwaitThenClauseFields { name, then_token } = node.as_fields();

        write!(f, [then_token.format(), space(), name.format()])
    }
}
