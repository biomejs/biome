use crate::prelude::*;
use biome_css_syntax::{
    CssPseudoElementFunctionCustomIdentifier, CssPseudoElementFunctionCustomIdentifierFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementFunctionCustomIdentifier;
impl FormatNodeRule<CssPseudoElementFunctionCustomIdentifier>
    for FormatCssPseudoElementFunctionCustomIdentifier
{
    fn fmt_fields(
        &self,
        node: &CssPseudoElementFunctionCustomIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoElementFunctionCustomIdentifierFields {
            name,
            l_paren_token,
            ident,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [
                name.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(&ident.format(), should_insert_space),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
