use crate::prelude::*;
use biome_css_syntax::{CssLayerReference, CssLayerReferenceFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLayerReference;
impl FormatNodeRule<CssLayerReference> for FormatCssLayerReference {
    fn fmt_fields(&self, node: &CssLayerReference, f: &mut CssFormatter) -> FormatResult<()> {
        let CssLayerReferenceFields {
            references,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                group(&indent(&references.format())),
                semicolon_token.format()
            ]
        )
    }
}
