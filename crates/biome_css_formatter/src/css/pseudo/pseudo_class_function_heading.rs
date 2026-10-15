use crate::prelude::*;
use crate::utils::case::pseudo_identifier_case;
use biome_css_syntax::{CssPseudoClassFunctionHeading, CssPseudoClassFunctionHeadingFields};
use biome_formatter::{format_args, write};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionHeading;
impl FormatNodeRule<CssPseudoClassFunctionHeading> for FormatCssPseudoClassFunctionHeading {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionHeading,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionHeadingFields {
            name,
            l_paren_token,
            levels,
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
                    soft_block_indent_with_maybe_space(&levels.format(), should_insert_space),
                    r_paren_token.format(),
                ])
            ]
        )
    }
}
