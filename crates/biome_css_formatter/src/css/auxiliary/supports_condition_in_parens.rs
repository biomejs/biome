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
        let condition = condition?;

        if f.options().file_source().is_scss()
            && condition.as_css_supports_not_condition().is_some()
        {
            // Sass requires `not (...)` to touch the enclosing `)`.
            let separator = if should_insert_space {
                soft_line_break_or_space()
            } else {
                soft_line_break()
            };
            return write!(
                f,
                [group(&format_args![
                    l_paren_token.format(),
                    indent(&format_args![
                        separator,
                        condition.format().with_text_case(CssCase::Preserve)
                    ]),
                    r_paren_token.format()
                ])]
            );
        }

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent_with_maybe_space(
                    &condition.format().with_text_case(CssCase::Preserve),
                    should_insert_space
                ),
                r_paren_token.format()
            ])]
        )
    }
}
