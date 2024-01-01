use crate::prelude::*;
use biome_css_syntax::{CssPseudoClassIdentifier, CssPseudoClassIdentifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassIdentifier;
impl FormatNodeRule<CssPseudoClassIdentifier> for FormatCssPseudoClassIdentifier {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassIdentifierFields { name } = node.as_fields();

        write!(f, [name.format()])
    }
}
