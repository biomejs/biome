use crate::prelude::*;
use biome_css_syntax::{CssSupportsFeatureDeclaration, CssSupportsFeatureDeclarationFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsFeatureDeclaration;
impl FormatNodeRule<CssSupportsFeatureDeclaration> for FormatCssSupportsFeatureDeclaration {
    fn fmt_fields(
        &self,
        node: &CssSupportsFeatureDeclaration,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssSupportsFeatureDeclarationFields {
            l_paren_token,
            declaration,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&declaration.format()),
                r_paren_token.format()
            ])]
        )
    }
}
