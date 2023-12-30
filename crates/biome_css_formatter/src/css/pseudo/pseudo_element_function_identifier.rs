use crate::prelude::*;
use biome_css_syntax::{
    CssPseudoElementFunctionIdentifier, CssPseudoElementFunctionIdentifierFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementFunctionIdentifier;
impl FormatNodeRule<CssPseudoElementFunctionIdentifier>
    for FormatCssPseudoElementFunctionIdentifier
{
    fn fmt_fields(
        &self,
        node: &CssPseudoElementFunctionIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoElementFunctionIdentifierFields {
            name,
            l_paren_token,
            ident,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&ident.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
