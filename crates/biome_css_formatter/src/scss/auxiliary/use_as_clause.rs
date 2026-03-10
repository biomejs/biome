use crate::prelude::*;
use biome_css_syntax::{ScssUseAsClause, ScssUseAsClauseFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssUseAsClause;

impl FormatNodeRule<ScssUseAsClause> for FormatScssUseAsClause {
    fn fmt_fields(&self, node: &ScssUseAsClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssUseAsClauseFields {
            as_token,
            namespace,
        } = node.as_fields();

        write!(f, [as_token.format(), space(), namespace.format()])
    }
}
