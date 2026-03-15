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

        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [
                layer_token.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(&name.format(), should_insert_space),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
