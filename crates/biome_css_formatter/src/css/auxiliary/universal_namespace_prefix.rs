use crate::prelude::*;
use biome_css_syntax::{CssUniversalNamespacePrefix, CssUniversalNamespacePrefixFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUniversalNamespacePrefix;
impl FormatNodeRule<CssUniversalNamespacePrefix> for FormatCssUniversalNamespacePrefix {
    fn fmt_fields(
        &self,
        node: &CssUniversalNamespacePrefix,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssUniversalNamespacePrefixFields { star_token } = node.as_fields();

        write!(f, [star_token.format()])
    }
}
