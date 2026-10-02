use crate::prelude::*;
use crate::utils::comment_trivia::FormatCommentGap;
use crate::utils::scss_statement_at_rule::{
    FormatScssStatementAtRuleSemicolon, format_scss_statement_at_rule_semicolon,
};
use biome_css_syntax::{CssSyntaxNode, CssSyntaxToken};
use biome_formatter::trivia::{format_dangling_comment, format_trailing_comments_from_slice};
use biome_formatter::{Format, write};

/// Keeps block comments before the semicolon and final line comments after it.
pub(crate) struct FormatStatementAtRuleEnding<'a> {
    node: &'a CssSyntaxNode,
    semicolon: FormatScssStatementAtRuleSemicolon,
}

impl<'a> FormatStatementAtRuleEnding<'a> {
    pub(crate) fn new(node: &'a CssSyntaxNode, semicolon: Option<CssSyntaxToken>) -> Self {
        Self {
            node,
            semicolon: format_scss_statement_at_rule_semicolon(semicolon),
        }
    }
}

impl Format<CssFormatContext> for FormatStatementAtRuleEnding<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let comments = f.comments().clone();
        let boundary_comments = comments.dangling_comments(self.node);
        for comment in boundary_comments {
            if comment.kind().is_line() {
                format_trailing_comments_from_slice(std::slice::from_ref(comment)).fmt(f)?;
            } else {
                write!(
                    f,
                    [
                        FormatCommentGap::new(comment.lines_before()),
                        format_dangling_comment(comment)
                    ]
                )?;
            }
        }

        self.semicolon.fmt(f)?;

        // Keep queued boundary comments ahead of comments after the statement.
        if boundary_comments
            .last()
            .is_some_and(|comment| comment.kind().is_line())
        {
            write!(f, [hard_line_break()])?;
        }

        Ok(())
    }
}
