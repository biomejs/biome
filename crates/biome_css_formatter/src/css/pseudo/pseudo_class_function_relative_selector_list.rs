use crate::css::value::identifier::FormatCssIdentifierOptions;
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
            name,
            l_paren_token,
            relative_selectors,
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
                        &relative_selectors.format(),
                        should_insert_space
                    ),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
