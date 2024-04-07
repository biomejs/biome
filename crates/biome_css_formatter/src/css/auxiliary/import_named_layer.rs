use crate::prelude::*;
use biome_css_syntax::{CssImportNamedLayer, CssImportNamedLayerFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssImportNamedLayer;
impl FormatNodeRule<CssImportNamedLayer> for FormatCssImportNamedLayer {
    fn fmt_fields(&self, node: &CssImportNamedLayer, f: &mut CssFormatter) -> FormatResult<()> {
        let CssImportNamedLayerFields {
            layer_token,
            l_paren_token,
            name,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                layer_token.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&name.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
