use crate::css::lists::custom_identifier_comma_separated_list::FormatCssCustomIdentifierCommaSeparatedListOptions;
use crate::css::value::identifier::FormatCssIdentifierOptions;
use crate::prelude::*;
use biome_css_syntax::{
    CssPseudoClassFunctionCustomIdentifierCommaSeparatedList,
    CssPseudoClassFunctionCustomIdentifierCommaSeparatedListFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionCustomIdentifierCommaSeparatedList;
impl FormatNodeRule<CssPseudoClassFunctionCustomIdentifierCommaSeparatedList>
    for FormatCssPseudoClassFunctionCustomIdentifierCommaSeparatedList
{
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionCustomIdentifierCommaSeparatedList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionCustomIdentifierCommaSeparatedListFields {
            name,
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format()?
                    .with_options(FormatCssIdentifierOptions::default().with_lowercasing()),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(
                        &items.format().with_options(
                            FormatCssCustomIdentifierCommaSeparatedListOptions::default()
                                .with_fluid_layout()
                        )
                    ),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
