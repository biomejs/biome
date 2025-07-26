use crate::css::value::identifier::FormatCssIdentifierOptions;
use crate::prelude::*;
use biome_css_syntax::{
    CssPseudoClassFunctionSelectorList, CssPseudoClassFunctionSelectorListFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionSelectorList;
impl FormatNodeRule<CssPseudoClassFunctionSelectorList>
    for FormatCssPseudoClassFunctionSelectorList
{
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionSelectorList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionSelectorListFields {
            name,
            l_paren_token,
            selectors,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format()?
                    .with_options(FormatCssIdentifierOptions::default().with_lowercasing()),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&selectors.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
