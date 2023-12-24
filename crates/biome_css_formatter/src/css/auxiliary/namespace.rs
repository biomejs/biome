use crate::prelude::*;
use biome_css_syntax::{CssNamespace, CssNamespaceFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNamespace;
impl FormatNodeRule<CssNamespace> for FormatCssNamespace {
    fn fmt_fields(&self, node: &CssNamespace, f: &mut CssFormatter) -> FormatResult<()> {
        let CssNamespaceFields {
            prefix,
            bitwise_or_token,
        } = node.as_fields();

        write!(f, [prefix.format(), bitwise_or_token.format()])
    }
}
