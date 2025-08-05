use crate::css::value::identifier::FormatCssIdentifierOptions;
use crate::prelude::*;
use biome_css_syntax::{
    CssPseudoClassFunctionCompoundSelector, CssPseudoClassFunctionCompoundSelectorFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionCompoundSelector;
impl FormatNodeRule<CssPseudoClassFunctionCompoundSelector>
    for FormatCssPseudoClassFunctionCompoundSelector
{
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionCompoundSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionCompoundSelectorFields {
            name,
            l_paren_token,
            selector,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format()?
                    .with_options(FormatCssIdentifierOptions::default().with_lowercasing()),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&selector.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
