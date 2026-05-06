use crate::comments::FormatCssLeadingComment;
use crate::prelude::*;
use crate::utils::comment_trivia::has_same_group_leading_block_comment;
use crate::utils::scss_expression::is_self_breaking_value;
use biome_css_syntax::{
    AnyScssExpression, CssLanguage, CssSyntaxNode, ScssListExpressionElement,
    is_in_scss_include_arguments,
};
use biome_formatter::comments::CommentKind;
use biome_formatter::{FormatRefWithRule, format_args, write};
use biome_rowan::AstNode;

type SeparatorCommentBody<'a> = dyn Fn(&mut CssFormatter) -> FormatResult<()> + 'a;

/// Shared opt-in for nodes that receive include separator comments.
pub(crate) trait FormatScssSeparatorComments<N>: FormatNodeRule<N>
where
    N: AstNode<Language = CssLanguage>,
{
    fn fmt_node_with_scss_separator_comments(
        &self,
        node: &N,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        ScssSeparatorComments::around(node.syntax()).fmt_node(f, |f| self.fmt_fields(node, f))
    }

    fn fmt_leading_scss_separator_comments(
        &self,
        node: &N,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        ScssSeparatorComments::around(node.syntax()).fmt_leading_comments(f)
    }
}

impl<N, T> FormatScssSeparatorComments<N> for T
where
    N: AstNode<Language = CssLanguage>,
    T: FormatNodeRule<N>,
{
}

/// Wrapper for nodes that receive SCSS separator-path comments.
struct ScssSeparatorComments<'a> {
    node: &'a CssSyntaxNode,
}

impl<'a> ScssSeparatorComments<'a> {
    pub(crate) fn around(node: &'a CssSyntaxNode) -> Self {
        Self { node }
    }

    /// Formats a node with attached separator comments when needed.
    pub(crate) fn fmt_node(
        self,
        f: &mut CssFormatter,
        fmt_node: impl Fn(&mut CssFormatter) -> FormatResult<()>,
    ) -> FormatResult<()> {
        if !is_in_scss_include_arguments(self.node) {
            return fmt_node(f);
        }

        write!(
            f,
            [group(&format_with(|f| {
                format_node_with_separator_comments(self.node, f, &fmt_node)
            }))]
        )
    }

    /// Prints or suppresses normal leading comments for this node.
    pub(crate) fn fmt_leading_comments(self, f: &mut CssFormatter) -> FormatResult<()> {
        if is_in_scss_include_arguments(self.node) {
            Ok(())
        } else {
            write!(f, [format_leading_comments(self.node)])
        }
    }
}

/// Writes a node after its attached separator comments.
fn format_node_with_separator_comments(
    node: &CssSyntaxNode,
    f: &mut CssFormatter,
    fmt_node: &impl Fn(&mut CssFormatter) -> FormatResult<()>,
) -> FormatResult<()> {
    let should_indent_after_leading_block = has_same_group_leading_block_comment(node, f);

    if should_indent_after_leading_block && node_has_self_breaking_value(node) {
        write_separator_leading_comments(node, f, Some(fmt_node))?;
        return Ok(());
    }

    write_separator_leading_comments(node, f, None)?;
    fmt_node(f)
}

/// Writes leading comments that stay grouped with one separated item.
fn write_separator_leading_comments(
    node: &CssSyntaxNode,
    f: &mut CssFormatter,
    indented_body: Option<&SeparatorCommentBody<'_>>,
) -> FormatResult<()> {
    let comments = f.comments().clone();
    let leading_comments = comments.leading_comments(node);
    let comment_count = leading_comments.len();

    for (index, comment) in leading_comments.iter().enumerate() {
        let lines_after = comment.lines_after();
        let kind = comment.kind();
        let is_last_comment = index + 1 == comment_count;

        write!(
            f,
            [FormatRefWithRule::new(comment, FormatCssLeadingComment)]
        )?;

        match kind {
            CommentKind::Block | CommentKind::InlineBlock => match lines_after {
                0 | 1 if is_last_comment => {
                    if let Some(fmt_body) = indented_body {
                        write!(
                            f,
                            [indent(&format_args![
                                soft_line_break_or_space(),
                                format_with(|f| fmt_body(f))
                            ])]
                        )?;
                    } else {
                        write!(f, [soft_line_break_or_space()])?;
                    }
                }
                0 | 1 => write!(f, [soft_line_break_or_space()])?,
                _ => write!(f, [empty_line()])?,
            },
            CommentKind::Line => {
                // A leading `//` comment on the next include item must expand
                // the parent group so the first format pass stays idempotent.
                write!(f, [expand_parent()])?;

                match lines_after {
                    0 | 1 => write!(f, [hard_line_break()])?,
                    _ => write!(f, [empty_line()])?,
                }
            }
        }

        comment.mark_formatted();
    }

    Ok(())
}

/// Returns `true` for nodes whose value owns multiline layout.
fn node_has_self_breaking_value(node: &CssSyntaxNode) -> bool {
    AnyScssExpression::cast(node.clone())
        .is_some_and(|expression| is_self_breaking_value(&expression))
        || ScssListExpressionElement::cast(node.clone())
            .and_then(|element| element.value().ok())
            .is_some_and(|value| is_self_breaking_value(&value))
}
