use crate::css::value::identifier::FormatCssIdentifierOptions;
use crate::prelude::*;
use biome_css_syntax::{
    CssPseudoClassFunctionCompoundSelectorList, CssPseudoClassFunctionCompoundSelectorListFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionCompoundSelectorList;
impl FormatNodeRule<CssPseudoClassFunctionCompoundSelectorList>
    for FormatCssPseudoClassFunctionCompoundSelectorList
{
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionCompoundSelectorList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionCompoundSelectorListFields {
            name,
            l_paren_token,
            compound_selectors,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [
                name.format()?
                    .with_options(FormatCssIdentifierOptions::default().with_lowercasing()),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(
                        &compound_selectors.format(),
                        should_insert_space
                    ),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
