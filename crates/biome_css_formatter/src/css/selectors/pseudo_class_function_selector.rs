use crate::prelude::*;
use biome_css_syntax::{CssPseudoClassFunctionSelector, CssPseudoClassFunctionSelectorFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionSelector;
impl FormatNodeRule<CssPseudoClassFunctionSelector> for FormatCssPseudoClassFunctionSelector {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionSelectorFields {
            name,
            l_paren_token,
            selector,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&selector.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
