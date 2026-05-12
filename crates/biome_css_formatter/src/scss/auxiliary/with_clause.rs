use crate::prelude::*;
use crate::utils::scss_module_configuration::is_source_separated_with_configuration;
use biome_css_syntax::{ScssWithClause, ScssWithClauseFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssWithClause;

impl FormatNodeRule<ScssWithClause> for FormatScssWithClause {
    fn fmt_fields(&self, node: &ScssWithClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssWithClauseFields {
            with_token,
            configurations,
        } = node.as_fields();

        let configurations = configurations?;

        write!(f, [with_token.format()])?;

        if is_source_separated_with_configuration(&configurations) {
            write!(f, [space()])?;
        }

        write!(f, [configurations.format()])
    }
}
