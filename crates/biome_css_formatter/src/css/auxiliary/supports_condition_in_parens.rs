use crate::prelude::*;
use biome_css_syntax::{CssSupportsConditionInParens, CssSupportsConditionInParensFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsConditionInParens;
impl FormatNodeRule<CssSupportsConditionInParens> for FormatCssSupportsConditionInParens {
    fn fmt_fields(
        &self,
        node: &CssSupportsConditionInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssSupportsConditionInParensFields {
            l_paren_token,
            condition,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent_with_maybe_space(&condition.format(), should_insert_space),
                r_paren_token.format()
            ])]
        )
    }
}
