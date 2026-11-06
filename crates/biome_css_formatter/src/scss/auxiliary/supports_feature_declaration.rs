use crate::prelude::*;
use biome_css_syntax::{ScssSupportsFeatureDeclaration, ScssSupportsFeatureDeclarationFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssSupportsFeatureDeclaration;

impl FormatNodeRule<ScssSupportsFeatureDeclaration> for FormatScssSupportsFeatureDeclaration {
    fn fmt_fields(
        &self,
        node: &ScssSupportsFeatureDeclaration,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssSupportsFeatureDeclarationFields {
            l_paren_token,
            declaration,
            r_paren_token,
        } = node.as_fields();
        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent_with_maybe_space(&declaration.format(), should_insert_space),
                r_paren_token.format(),
            ])]
        )
    }
}
