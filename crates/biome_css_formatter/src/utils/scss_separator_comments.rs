use crate::prelude::*;
use crate::utils::comment_trivia::has_same_group_leading_block_comment;
use crate::utils::scss_expression::is_self_breaking_value;
use biome_css_syntax::{
    AnyScssExpression, CssLanguage, CssSyntaxNode, ScssListExpressionElement,
    is_in_scss_include_arguments,
};
use biome_formatter::write;
use biome_rowan::AstNode;

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
    expand_if_has_leading_line_comment(node, f)?;

    if has_same_group_leading_block_comment(node, f) {
        if node_has_self_breaking_value(node) {
            write!(
                f,
                [format_leading_comments(node).with_indented_following_content(fmt_node)]
            )?;
        } else {
            write!(
                f,
                [format_leading_comments(node).with_following_content(fmt_node)]
            )?;
        }

        return Ok(());
    }

    write!(f, [format_leading_comments(node)])?;
    fmt_node(f)
}

fn expand_if_has_leading_line_comment(
    node: &CssSyntaxNode,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    let has_line_comment = f
        .comments()
        .leading_comments(node)
        .iter()
        .any(|comment| comment.kind().is_line());

    if has_line_comment {
        // A leading `//` comment on the next include item must expand
        // the parent group so the first format pass stays idempotent.
        write!(f, [expand_parent()])?;
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
