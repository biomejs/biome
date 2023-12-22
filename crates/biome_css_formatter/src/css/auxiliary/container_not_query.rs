use crate::prelude::*;
use biome_css_syntax::{CssContainerNotQuery, CssContainerNotQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerNotQuery;
impl FormatNodeRule<CssContainerNotQuery> for FormatCssContainerNotQuery {
    fn fmt_fields(&self, node: &CssContainerNotQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssContainerNotQueryFields { not_token, query } = node.as_fields();

        write!(f, [not_token.format(), space(), query.format()])
    }
}
