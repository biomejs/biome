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
        write!(f, [FormatScssMapPairLayout { node }])
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

/// Formats `key: value` with Prettier's map-pair wrapping.
///
/// A broken key can still keep a short scalar value after `:`, e.g.
/// `("long", "key"): value`. Parenthesized values stay after `:`, e.g.
/// `key: (value)`.
struct FormatScssMapPairLayout<'a> {
    node: &'a ScssMapExpressionPair,
}

impl Format<CssFormatContext> for FormatScssMapPairLayout<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMapExpressionPairFields {
            key,
            colon_token,
            value,
        } = self.node.as_fields();
        let key = key?;
        let colon_token = colon_token?;
        let value = value?;

        // `/* c */ key: (nested: value)` keeps the nested map in the pair group.
        if is_self_breaking_value(&value)
            && !is_self_breaking_value(&key)
            && f.comments().leading_comments(self.node.syntax()).is_empty()
        {
            return write!(
                f,
                [group(&format_args![
                    key.format(),
                    colon_token.format(),
                    space(),
                    value.format()
                ])]
            );
        }

        let breakable_pair = format_with(|f| self.write_breakable_pair(f));

        write!(f, [group(&indent(&breakable_pair))])
    }
}

impl FormatScssMapPairLayout<'_> {
    /// Formats a pair whose key or scalar value may break.
    ///
    /// Example: `("long", "key"): value`.
    fn write_breakable_pair(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMapExpressionPairFields {
            key,
            colon_token,
            value,
        } = self.node.as_fields();
        let key = key?;
        let colon_token = colon_token?;
        let value = value?;
        let value_separator = format_with(|f| {
            if is_self_breaking_value(&value) {
                write!(f, [space()])
            } else {
                write!(f, [soft_line_break_or_space()])
            }
        });
        let empty_separator = format_once(|_| Ok(()));

        let mut fill = f.fill();
        fill.entry(&soft_line_break_or_space(), &dedent(&key.format()));
        // `fill` alternates item/separator/item; `:` is an item glued to
        // the key with an empty separator, e.g. `("a", "b"): value`.
        fill.entry(&empty_separator, &colon_token.format());
        fill.entry(&value_separator, &value.format());

        fill.finish()
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
