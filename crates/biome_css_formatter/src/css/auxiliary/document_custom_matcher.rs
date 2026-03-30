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

        let should_insert_space = f.options().delimiter_spacing().value();

        if should_insert_space {
            write!(
                f,
                [
                    name.format(),
                    l_paren_token.format(),
                    space(),
                    value.format(),
                    space(),
                    r_paren_token.format()
                ]
            )
        } else {
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
}
