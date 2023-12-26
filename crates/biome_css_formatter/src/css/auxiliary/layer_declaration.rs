use crate::prelude::*;
use biome_css_syntax::{CssLayerDeclaration, CssLayerDeclarationFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLayerDeclaration;
impl FormatNodeRule<CssLayerDeclaration> for FormatCssLayerDeclaration {
    fn fmt_fields(&self, node: &CssLayerDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        let CssLayerDeclarationFields { references, block } = node.as_fields();

        write!(
            f,
            [
                group(&indent(&references.format())),
                space(),
                block.format()
            ]
        )
    }
}
