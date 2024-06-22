use crate::prelude::*;
use biome_css_syntax::{CssIdentifier, CssIdentifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIdentifier;

impl FormatNodeRule<CssIdentifier> for FormatCssIdentifier {
    fn fmt_fields(&self, node: &CssIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIdentifierFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
