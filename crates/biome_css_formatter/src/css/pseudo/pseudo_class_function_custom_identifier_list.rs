use crate::css::lists::custom_identifier_comma_separated_list::FormatCssCustomIdentifierCommaSeparatedListOptions;
use crate::css::value::identifier::FormatCssIdentifierOptions;
use crate::prelude::*;
use biome_css_syntax::{
    CssPseudoClassFunctionCustomIdentifierList, CssPseudoClassFunctionCustomIdentifierListFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionCustomIdentifierList;
impl FormatNodeRule<CssPseudoClassFunctionCustomIdentifierList>
    for FormatCssPseudoClassFunctionCustomIdentifierList
{
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionCustomIdentifierList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionCustomIdentifierListFields {
            name,
            l_paren_token,
            items,
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
                        &items.format().with_options(
                            FormatCssCustomIdentifierCommaSeparatedListOptions::default()
                                .with_fluid_layout()
                        ),
                        should_insert_space
                    ),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
