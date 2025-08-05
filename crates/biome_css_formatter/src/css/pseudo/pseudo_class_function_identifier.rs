use crate::css::value::identifier::FormatCssIdentifierOptions;
use crate::prelude::*;
use biome_css_syntax::{CssPseudoClassFunctionIdentifier, CssPseudoClassFunctionIdentifierFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionIdentifier;
impl FormatNodeRule<CssPseudoClassFunctionIdentifier> for FormatCssPseudoClassFunctionIdentifier {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionIdentifierFields {
            name,
            l_paren_token,
            ident,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format()?
                    .with_options(FormatCssIdentifierOptions::default().with_lowercasing()),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&ident.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
