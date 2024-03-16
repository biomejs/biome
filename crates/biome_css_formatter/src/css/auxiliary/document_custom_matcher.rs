use crate::prelude::*;
use biome_css_syntax::{CssDocumentCustomMatcher, CssDocumentCustomMatcherFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDocumentCustomMatcher;
impl FormatNodeRule<CssDocumentCustomMatcher> for FormatCssDocumentCustomMatcher {
    fn fmt_fields(
        &self,
        node: &CssDocumentCustomMatcher,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssDocumentCustomMatcherFields {
            name,
            l_paren_token,
            value,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                l_paren_token.format(),
                value.format(),
                r_paren_token.format()
            ]
        )
    }
}
