use crate::prelude::*;
use biome_css_syntax::{CssContainerStyleOrQuery, CssContainerStyleOrQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerStyleOrQuery;
impl FormatNodeRule<CssContainerStyleOrQuery> for FormatCssContainerStyleOrQuery {
    fn fmt_fields(
        &self,
        node: &CssContainerStyleOrQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerStyleOrQueryFields {
            left,
            or_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                or_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
