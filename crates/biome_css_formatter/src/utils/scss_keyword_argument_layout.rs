use crate::prelude::*;
use crate::utils::scss_expression::is_self_breaking_value;
use biome_css_syntax::{ScssKeywordArgument, ScssKeywordArgumentFields};
use biome_formatter::{format_args, write};

/// Layout for `$arg: value` keyword arguments.
pub(crate) struct ScssKeywordArgumentLayout<'a> {
    node: &'a ScssKeywordArgument,
}

impl<'a> ScssKeywordArgumentLayout<'a> {
    pub(crate) fn new(node: &'a ScssKeywordArgument) -> Self {
        Self { node }
    }

    pub(crate) fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let node = self.node;

        let ScssKeywordArgumentFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        let child_value_is_self_breaking = value.as_ref().is_ok_and(is_self_breaking_value);
        let formatted_child_value = format_with(|f| write!(f, [value.format()]));

        let formatted_value = format_with(|f| {
            if child_value_is_self_breaking {
                write!(f, [space(), formatted_child_value])
            } else {
                write!(
                    f,
                    [indent(&format_args![
                        soft_line_break_or_space(),
                        formatted_child_value
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
