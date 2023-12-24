use crate::prelude::*;
use biome_css_syntax::{CssMediaTypeQuery, CssMediaTypeQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaTypeQuery;
impl FormatNodeRule<CssMediaTypeQuery> for FormatCssMediaTypeQuery {
    fn fmt_fields(&self, node: &CssMediaTypeQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaTypeQueryFields { modifier, ty } = node.as_fields();

        if modifier.is_some() {
            write!(f, [modifier.format(), space()])?;
        }

        write!(f, [ty.format()])
    }
}
