use crate::css::value::identifier::FormatCssIdentifierOptions;
use crate::prelude::*;
use biome_css_syntax::{CssPseudoClassFunctionValueList, CssPseudoClassFunctionValueListFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionValueList;
impl FormatNodeRule<CssPseudoClassFunctionValueList> for FormatCssPseudoClassFunctionValueList {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionValueList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionValueListFields {
            name,
            l_paren_token,
            values,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format()?
                    .with_options(FormatCssIdentifierOptions::default().with_lowercasing()),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&values.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
