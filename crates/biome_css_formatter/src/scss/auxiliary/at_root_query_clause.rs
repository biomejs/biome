use crate::comments::CssCommentStyle;
use crate::prelude::*;
use biome_css_syntax::{CssLanguage, ScssAtRootQueryClause, ScssAtRootQueryClauseFields};
use biome_formatter::comments::SourceComment;
use biome_formatter::trivia::format_dangling_comment;
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssAtRootQueryClause;

impl FormatNodeRule<ScssAtRootQueryClause> for FormatScssAtRootQueryClause {
    fn fmt_fields(&self, node: &ScssAtRootQueryClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssAtRootQueryClauseFields {
            modifier,
            colon_token,
            rules,
        } = node.as_fields();
        let comments = f.comments().clone();
        let boundary_comments = comments.dangling_comments(node.syntax());
        let has_suppressed_rules = has_boundary_suppression(boundary_comments);
        let rules = rules?;

        write!(f, [modifier.format(), colon_token.format()])?;

        let mut is_after_line_comment = false;
        for comment in boundary_comments {
            let formatted = format_dangling_comment(comment);
            if is_after_line_comment {
                write!(f, [hard_line_break(), formatted])?;
            } else {
                write!(
                    f,
                    [group(&indent(&format_args![
                        soft_line_break_or_space(),
                        formatted
                    ]))]
                )?;
            }
            is_after_line_comment = comment.kind().is_line();
        }

        let formatted_rules = format_with(|f| {
            if has_suppressed_rules {
                format_suppressed_node(rules.syntax()).fmt(f)
            } else {
                rules.format().fmt(f)
            }
        });

        if is_after_line_comment {
            write!(f, [hard_line_break(), formatted_rules])
        } else {
            write!(
                f,
                [group(&indent(&format_args![
                    soft_line_break_or_space(),
                    formatted_rules
                ]))]
            )
        }
    }

    fn is_suppressed(&self, node: &ScssAtRootQueryClause, f: &CssFormatter) -> bool {
        let boundary_comments = f.comments().dangling_comments(node.syntax());
        !has_boundary_suppression(boundary_comments) && f.comments().is_suppressed(node.syntax())
    }

    fn fmt_dangling_comments(
        &self,
        _node: &ScssAtRootQueryClause,
        _f: &mut CssFormatter,
    ) -> FormatResult<()> {
        // `(without: /* comment */ media)` is handled beside the colon.
        Ok(())
    }
}

fn has_boundary_suppression(comments: &[SourceComment<CssLanguage>]) -> bool {
    comments
        .iter()
        .any(|comment| CssCommentStyle::is_suppression(comment.piece().text()))
}
