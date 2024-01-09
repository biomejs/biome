use crate::prelude::*;
use biome_css_syntax::{
    CssPseudoClassFunctionRelativeSelectorList, CssPseudoClassFunctionRelativeSelectorListFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionRelativeSelectorList;
impl FormatNodeRule<CssPseudoClassFunctionRelativeSelectorList>
    for FormatCssPseudoClassFunctionRelativeSelectorList
{
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionRelativeSelectorList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionRelativeSelectorListFields {
            name_token,
            l_paren_token,
            relative_selectors,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name_token.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&relative_selectors.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
