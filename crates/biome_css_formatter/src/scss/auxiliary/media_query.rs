use crate::prelude::*;
use biome_css_syntax::{ScssMediaQuery, ScssMediaQueryFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMediaQuery;
impl FormatNodeRule<ScssMediaQuery> for FormatScssMediaQuery {
    fn fmt_fields(&self, node: &ScssMediaQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMediaQueryFields { query } = node.as_fields();

        query.format().fmt(f)
    }
}
