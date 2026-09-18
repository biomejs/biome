use crate::css::lists::custom_identifier_comma_separated_list::FormatCssCustomIdentifierCommaSeparatedListOptions;
use crate::prelude::*;
use crate::utils::case::pseudo_identifier_case;
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
        let name = name?;

        write!(
            f,
            [
                name.format().with_text_case(pseudo_identifier_case(&name)),
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
