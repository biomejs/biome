use crate::prelude::*;
use biome_css_syntax::{CssBooleanMediaQuery, CssBooleanMediaQueryFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBooleanMediaQuery;

impl FormatNodeRule<CssBooleanMediaQuery> for FormatCssBooleanMediaQuery {
    fn fmt_fields(&self, node: &CssBooleanMediaQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssBooleanMediaQueryFields { boolean } = node.as_fields();

        boolean.format()?.with_text_case(CssCase::Lowercase).fmt(f)
    }
}
