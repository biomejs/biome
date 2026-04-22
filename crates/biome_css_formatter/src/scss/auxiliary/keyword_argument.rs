use crate::prelude::*;
use crate::utils::scss_expression::value_manages_its_own_breaking;
use biome_css_syntax::{ScssKeywordArgument, ScssKeywordArgumentFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssKeywordArgument;
impl FormatNodeRule<ScssKeywordArgument> for FormatScssKeywordArgument {
    fn fmt_fields(&self, node: &ScssKeywordArgument, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssKeywordArgumentFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        let child_value_manages_its_own_breaking =
            value.as_ref().is_ok_and(value_manages_its_own_breaking);

        let formatted_value = format_with(|f| {
            if child_value_manages_its_own_breaking {
                // Keep `$arg: (` inline and let the child formatter break inside the value.
                write!(f, [space(), value.format()])
            } else {
                write!(
                    f,
                    [indent(&format_args![
                        soft_line_break_or_space(),
                        value.format()
                    ])]
                )
            }
        });

        write!(
            f,
            [group(&format_args![
                name.format(),
                group(&format_args![colon_token.format(), formatted_value])
            ])]
        )
    }
}
