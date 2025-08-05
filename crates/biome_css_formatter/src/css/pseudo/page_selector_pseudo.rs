use crate::css::value::identifier::FormatCssIdentifierOptions;
use crate::prelude::*;
use biome_css_syntax::{CssPageSelectorPseudo, CssPageSelectorPseudoFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageSelectorPseudo;
impl FormatNodeRule<CssPageSelectorPseudo> for FormatCssPageSelectorPseudo {
    fn fmt_fields(&self, node: &CssPageSelectorPseudo, f: &mut CssFormatter) -> FormatResult<()> {
        let CssPageSelectorPseudoFields {
            colon_token,
            selector,
        } = node.as_fields();

        write!(
            f,
            [
                colon_token.format(),
                selector
                    .format()?
                    .with_options(FormatCssIdentifierOptions::default().with_lowercasing())
            ]
        )
    }
}
