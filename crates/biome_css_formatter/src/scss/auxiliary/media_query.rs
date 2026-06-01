use crate::prelude::*;
use biome_css_syntax::{ScssMediaQuery, ScssMediaQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMediaQuery;
impl FormatNodeRule<ScssMediaQuery> for FormatScssMediaQuery {
    fn fmt_fields(&self, node: &ScssMediaQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMediaQueryFields { query } = node.as_fields();

        write!(f, [query.format()])
    }
}
