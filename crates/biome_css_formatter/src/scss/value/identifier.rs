use crate::prelude::*;
use biome_css_syntax::{ScssIdentifier, ScssIdentifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssIdentifier;

impl FormatNodeRule<ScssIdentifier> for FormatScssIdentifier {
    fn fmt_fields(&self, node: &ScssIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssIdentifierFields { dollar_token, name } = node.as_fields();

        write!(f, [dollar_token.format(), name.format()])
    }
}
