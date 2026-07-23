use crate::prelude::*;
use crate::utils::case::pseudo_identifier_case;
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

        let should_insert_space = f.options().delimiter_spacing().value();
        let name = name?;

        write!(
            f,
            [
                name.format().with_text_case(pseudo_identifier_case(&name)),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(&selector.format(), should_insert_space),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
