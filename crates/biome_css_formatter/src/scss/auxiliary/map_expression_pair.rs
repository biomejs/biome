use crate::prelude::*;
use crate::utils::comment_trivia::format_leading_comments_with_soft_lines;
use crate::utils::scss_expression::is_self_breaking_value;
use crate::utils::scss_separator_comments::FormatScssSeparatorComments;
use biome_css_syntax::{
    AnyScssExpression, CssSyntaxToken, ScssMapExpressionPair, ScssMapExpressionPairFields,
    is_in_scss_include_arguments,
};
use biome_formatter::comments::CommentKind;
use biome_formatter::{format_args, write};
use biome_rowan::SyntaxResult;

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

        write!(
            f,
            [FormatScssMapPairLayout {
                key: &key,
                colon_token: &colon_token,
                value: &value,
            }]
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

/// Formats `key: value` with Prettier's map-pair wrapping.
///
/// A broken key can still keep a short scalar value after `:`, e.g.
/// `("long", "key"): value`. Parenthesized values stay after `:`, e.g.
/// `key: (value)`.
struct FormatScssMapPairLayout<'a> {
    key: &'a SyntaxResult<AnyScssExpression>,
    colon_token: &'a SyntaxResult<CssSyntaxToken>,
    value: &'a SyntaxResult<AnyScssExpression>,
}

impl Format<CssFormatContext> for FormatScssMapPairLayout<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        if self.is_value_self_breaking() && !self.is_key_self_breaking() {
            return write!(
                f,
                [group(&format_args![
                    self.key.format(),
                    self.colon_token.format(),
                    space(),
                    self.value.format()
                ])]
            );
        }

        write!(f, [group(&indent(&self.format_breakable_pair()))])
    }
}

impl FormatScssMapPairLayout<'_> {
    fn is_key_self_breaking(&self) -> bool {
        self.key.as_ref().is_ok_and(is_self_breaking_value)
    }

    fn is_value_self_breaking(&self) -> bool {
        self.value.as_ref().is_ok_and(is_self_breaking_value)
    }

    /// Formats a pair whose key or scalar value may break.
    ///
    /// Example: `("long", "key"): value`.
    fn format_breakable_pair(&self) -> impl Format<CssFormatContext> + '_ {
        format_with(|f| {
            let separator = soft_line_break_or_space();
            let value_separator = format_with(|f| {
                if self.is_value_self_breaking() {
                    write!(f, [space()])
                } else {
                    write!(f, [soft_line_break_or_space()])
                }
            });
            let empty_separator = format_once(|_| Ok(()));
            let key = format_with(|f| write!(f, [dedent(&format_args![self.key.format()])]));
            let colon = format_with(|f| write!(f, [self.colon_token.format()]));
            let value = format_with(|f| write!(f, [self.value.format()]));
            let mut fill = f.fill();

            fill.entry(&separator, &key);
            // `fill` alternates item/separator/item; `:` is an item glued to
            // the key with an empty separator, e.g. `("a", "b"): value`.
            fill.entry(&empty_separator, &colon);
            fill.entry(&value_separator, &value);

            fill.finish()
        })
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
