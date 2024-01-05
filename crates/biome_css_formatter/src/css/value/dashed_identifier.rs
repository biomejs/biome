use crate::prelude::*;
use biome_css_syntax::{CssDashedIdentifier, CssDashedIdentifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDashedIdentifier;
impl FormatNodeRule<CssDashedIdentifier> for FormatCssDashedIdentifier {
    fn fmt_fields(&self, node: &CssDashedIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        let CssDashedIdentifierFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
