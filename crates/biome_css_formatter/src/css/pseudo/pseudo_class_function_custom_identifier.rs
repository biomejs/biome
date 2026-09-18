use crate::css::value::identifier::FormatCssIdentifierOptions;
use crate::prelude::*;
use biome_css_syntax::{
    CssPseudoClassFunctionCustomIdentifier, CssPseudoClassFunctionCustomIdentifierFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionCustomIdentifier;
impl FormatNodeRule<CssPseudoClassFunctionCustomIdentifier>
    for FormatCssPseudoClassFunctionCustomIdentifier
{
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionCustomIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionCustomIdentifierFields {
            name,
            l_paren_token,
            ident,
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
                    soft_block_indent_with_maybe_space(&ident.format(), should_insert_space),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
