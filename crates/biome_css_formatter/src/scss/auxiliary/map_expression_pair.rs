use crate::prelude::*;
use crate::utils::comment_trivia::format_leading_comments_with_soft_lines;
use crate::utils::scss_expression::is_self_breaking_value;
use crate::utils::scss_separator_comments::FormatScssSeparatorComments;
use biome_css_syntax::{
    ScssMapExpressionPair, ScssMapExpressionPairFields, is_in_scss_include_arguments,
};
use biome_formatter::comments::CommentKind;
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMapExpressionPair;
impl FormatNodeRule<ScssMapExpressionPair> for FormatScssMapExpressionPair {
    fn fmt_node(&self, node: &ScssMapExpressionPair, f: &mut CssFormatter) -> FormatResult<()> {
        match ScssMapPairLeadingCommentLayout::for_pair(node, f) {
            ScssMapPairLeadingCommentLayout::Separator => {
                self.fmt_node_with_scss_separator_comments(node, f)
            }
            ScssMapPairLeadingCommentLayout::GroupWithPair => {
                write!(
                    f,
                    [group(&format_args![
                        format_leading_comments_with_soft_lines(node.syntax()),
                        format_with(|f| self.fmt_fields(node, f))
                    ])]
                )
            }
            ScssMapPairLeadingCommentLayout::Default => self.fmt_fields(node, f),
        }
    }

    fn fmt_fields(&self, node: &ScssMapExpressionPair, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMapExpressionPairFields {
            key,
            colon_token,
            value,
        } = node.as_fields();

        let is_self_breaking = value.as_ref().is_ok_and(is_self_breaking_value);

        write!(
            f,
            [group(&format_args![
                key.format(),
                colon_token.format(),
                FormatScssMapExpressionPairValue {
                    value: &value.format(),
                    is_self_breaking,
                }
            ])]
        )
    }

    fn fmt_leading_comments(
        &self,
        node: &ScssMapExpressionPair,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match ScssMapPairLeadingCommentLayout::for_pair(node, f) {
            ScssMapPairLeadingCommentLayout::Separator
            | ScssMapPairLeadingCommentLayout::GroupWithPair => Ok(()),
            ScssMapPairLeadingCommentLayout::Default => {
                write!(f, [format_leading_comments(node.syntax())])
            }
        }
    }
}

/// How leading comments are printed for one SCSS map pair.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ScssMapPairLeadingCommentLayout {
    /// Include arguments use separator comments, e.g. `@include mix($a, /* c */ $b)`.
    Separator,

    /// `/* comment */ key: value` joins the pair group.
    GroupWithPair,

    /// Normal leading comments use the default comment printer.
    Default,
}

impl ScssMapPairLeadingCommentLayout {
    fn for_pair(node: &ScssMapExpressionPair, f: &CssFormatter) -> Self {
        if is_in_scss_include_arguments(node.syntax()) {
            Self::Separator
        } else if should_group_leading_block_comments_with_pair(node, f) {
            Self::GroupWithPair
        } else {
            Self::Default
        }
    }
}

/// Formats the value in `key: value`.
///
/// Self-breaking values such as `key: (a, b)` stay after the colon; scalar
/// values may break as `key:\n  value`.
struct FormatScssMapExpressionPairValue<'a> {
    value: &'a dyn Format<CssFormatContext>,
    is_self_breaking: bool,
}

impl Format<CssFormatContext> for FormatScssMapExpressionPairValue<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        if self.is_self_breaking {
            write!(f, [space(), self.value])
        } else {
            write!(
                f,
                [indent(&format_args![
                    soft_line_break_or_space(),
                    self.value
                ])]
            )
        }
    }
}

/// Returns `true` for `/* comment */ key: value`.
///
/// The comment joins the pair group, so long comments break before `key`.
fn should_group_leading_block_comments_with_pair(
    node: &ScssMapExpressionPair,
    f: &CssFormatter,
) -> bool {
    let leading_comments = f.comments().leading_comments(node.syntax());

    if leading_comments.is_empty()
        || !leading_comments.iter().all(|comment| {
            matches!(
                comment.kind(),
                CommentKind::Block | CommentKind::InlineBlock
            ) && comment.lines_after() <= 1
        })
    {
        return false;
    }

    let Ok(value) = node.value() else {
        return false;
    };

    if is_self_breaking_value(&value) {
        return false;
    }

    true
}
