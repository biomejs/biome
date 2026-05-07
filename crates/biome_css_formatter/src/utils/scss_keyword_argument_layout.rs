use crate::prelude::*;
use crate::utils::comment_trivia::{
    has_inline_trailing_comment, has_same_group_leading_block_comment,
};
use crate::utils::scss_expression::is_self_breaking_value;
use crate::utils::scss_include_keyword_value::has_top_level_include_keyword_parenthesized_value;
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

        let is_self_breaking = value.as_ref().is_ok_and(|value| {
            is_self_breaking_value(value)
                || has_top_level_include_keyword_parenthesized_value(node, value)
        });
        let should_indent_self_breaking = is_self_breaking
            && is_in_scss_include_arguments(node.syntax())
            && has_same_group_leading_block_comment(node.syntax(), f);
        let has_trailing_comments = f.comments().has_dangling_comments(node.syntax());
        let has_raw_trailing_comment = value
            .as_ref()
            .is_ok_and(|value| has_inline_trailing_comment(value.syntax()));
        let should_expand = is_self_breaking && (has_trailing_comments || has_raw_trailing_comment);
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
                group(&format_args![
                    colon_token.format(),
                    FormatScssKeywordArgumentValue {
                        value: &value.format(),
                        is_self_breaking,
                        should_expand,
                        should_indent_self_breaking,
                    }
                ]),
                trailing_comments
            ])]
        )
    }
}

/// Formats the value in `$arg: value`.
///
/// Self-breaking values such as `$arg: (a, b)` and `$arg: 2 * (a)`
/// stay after the colon, while scalar values may break after it.
struct FormatScssKeywordArgumentValue<'a> {
    value: &'a dyn Format<CssFormatContext>,
    is_self_breaking: bool,
    should_expand: bool,
    should_indent_self_breaking: bool,
}

impl Format<CssFormatContext> for FormatScssKeywordArgumentValue<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let child = FormatScssKeywordArgumentChildValue {
            value: self.value,
            should_expand: self.should_expand,
            should_indent_self_breaking: self.should_indent_self_breaking,
        };

        if self.is_self_breaking {
            write!(f, [space(), child])
        } else {
            write!(
                f,
                [indent(&format_args![soft_line_break_or_space(), child])]
            )
        }
    }
}

/// Formats the child value before colon spacing is applied.
///
/// Include comments can force `$arg: (a, b)` to expand while keeping the value
/// owned by the keyword argument.
struct FormatScssKeywordArgumentChildValue<'a> {
    value: &'a dyn Format<CssFormatContext>,
    should_expand: bool,
    should_indent_self_breaking: bool,
}

impl Format<CssFormatContext> for FormatScssKeywordArgumentChildValue<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        if self.should_expand {
            let value = format_with(|f| write!(f, [self.value]));
            let expanded_value = group(&value).should_expand(true);

            if self.should_indent_self_breaking {
                write!(f, [indent(&expanded_value)])
            } else {
                write!(f, [expanded_value])
            }
        } else if self.should_indent_self_breaking {
            write!(f, [indent(&format_args![self.value])])
        } else {
            write!(f, [self.value])
        }
    }
}
