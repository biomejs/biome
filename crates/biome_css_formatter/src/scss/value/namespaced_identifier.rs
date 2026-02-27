use crate::prelude::*;
use biome_css_syntax::{ScssNamespacedIdentifier, ScssNamespacedIdentifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssNamespacedIdentifier;

impl FormatNodeRule<ScssNamespacedIdentifier> for FormatScssNamespacedIdentifier {
    fn fmt_fields(
        &self,
        node: &ScssNamespacedIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssNamespacedIdentifierFields {
            namespace,
            dot_token,
            name,
        } = node.as_fields();

        write!(f, [namespace.format(), dot_token.format(), name.format()])
    }
}
