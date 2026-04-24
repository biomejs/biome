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

        let maybe_space = format_with(|f: &mut CssFormatter| {
            if f.options().delimiter_spacing().value() {
                write!(f, [space()])?;
            }
            Ok(())
        });

        write!(
            f,
            [
                name.format(),
                l_paren_token.format(),
                maybe_space,
                value.format(),
                maybe_space,
                r_paren_token.format()
            ]
        )
    }
}
