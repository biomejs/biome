use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteEachBindingClause, SvelteEachBindingClauseFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteEachBindingClause;

impl FormatNodeRule<SvelteEachBindingClause> for FormatSvelteEachBindingClause {
    fn fmt_fields(
        &self,
        node: &SvelteEachBindingClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteEachBindingClauseFields { as_token, clause } = node.as_fields();

        write!(f, [space(), as_token.format(), space(), clause.format()])
    }
}
