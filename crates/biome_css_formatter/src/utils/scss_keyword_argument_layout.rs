use crate::prelude::*;
use crate::utils::comment_trivia::{
    has_inline_trailing_comment, has_same_group_leading_block_comment,
};
use crate::utils::scss_expression::is_self_breaking_value;
use biome_css_syntax::{
    ScssKeywordArgument, ScssKeywordArgumentFields, is_in_scss_include_arguments,
};
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
        let indent_self_breaking_value = child_value_is_self_breaking
            && is_in_scss_include_arguments(node.syntax())
            && has_same_group_leading_block_comment(node.syntax(), f);
        let has_trailing_comments = f.comments().has_dangling_comments(node.syntax());
        let has_raw_trailing_comment = value
            .as_ref()
            .is_ok_and(|value| has_inline_trailing_comment(value.syntax()));
        let should_expand_value =
            child_value_is_self_breaking && (has_trailing_comments || has_raw_trailing_comment);

        let formatted_child_value = format_with(|f| {
            if should_expand_value {
                let value = value.format();
                let expanded_value = group(&value).should_expand(true);

                if indent_self_breaking_value {
                    write!(f, [indent(&expanded_value)])
                } else {
                    write!(f, [expanded_value])
                }
            } else if indent_self_breaking_value {
                write!(f, [indent(&value.format())])
            } else {
                write!(f, [value.format()])
            }
        });

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
        let trailing_comments = format_with(|f| {
            if has_trailing_comments {
                write!(
                    f,
                    [
                        soft_line_break_or_space(),
                        format_dangling_comments(node.syntax())
                    ]
                )
            } else {
                Ok(())
            }
        });

        write!(
            f,
            [group(&format_args![
                name.format(),
                group(&format_args![colon_token.format(), formatted_value]),
                trailing_comments
            ])]
        )
    }
}
