use crate::comments::FormatCssLeadingComment;
use crate::prelude::*;
use crate::utils::comment_trivia::{
    has_block_comment_gap_before_token, has_source_gap_before_token,
};
use biome_css_syntax::{
    CssLanguage, CssSyntaxToken, ScssVariableDeclaration, ScssVariableDeclarationFields,
};
use biome_formatter::comments::SourceComment;
use biome_formatter::trivia::format_dangling_comment;
use biome_formatter::{FormatRefWithRule, format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssVariableDeclaration;

impl FormatNodeRule<ScssVariableDeclaration> for FormatScssVariableDeclaration {
    fn fmt_fields(&self, node: &ScssVariableDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssVariableDeclarationFields {
            name,
            value,
            modifiers,
            semicolon_token,
            ..
        } = node.as_fields();
        let variable_comments = ScssVariableDeclarationComments::new(node);
        // Only modifier-bearing variables preserve `$color: red !default /* c */ ;`.
        // Plain variables normalize the pre-`;` gap.
        let has_modifiers = !modifiers.is_empty();

        write!(f, [name.format()])?;
        variable_comments.fmt_colon_boundary(f)?;
        write!(f, [space(), value.format()])?;

        if !modifiers.is_empty() {
            write!(f, [modifiers.format()])?;
        }

        variable_comments.fmt_semicolon_boundary(semicolon_token, has_modifiers, f)
    }

    fn fmt_leading_comments(
        &self,
        node: &ScssVariableDeclaration,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        ScssVariableDeclarationComments::new(node).fmt_leading_comments(f)
    }

    fn fmt_dangling_comments(
        &self,
        _node: &ScssVariableDeclaration,
        _f: &mut CssFormatter,
    ) -> FormatResult<()> {
        // No-op: `fmt_colon_boundary` prints `$color/* c */ : red;` before `:`,
        // not at the declaration end.
        Ok(())
    }
}

/// Formats SCSS variable comment boundaries.
struct ScssVariableDeclarationComments<'a> {
    node: &'a ScssVariableDeclaration,
}

impl<'a> ScssVariableDeclarationComments<'a> {
    fn new(node: &'a ScssVariableDeclaration) -> Self {
        Self { node }
    }

    /// Prints `/* note */ $color: red;` as a comment, then the declaration.
    fn fmt_leading_comments(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let comments = f.comments().clone();
        let leading_comments = comments.leading_comments(self.node.syntax());

        for comment in leading_comments {
            let format_comment = FormatRefWithRule::new(comment, FormatCssLeadingComment);
            write!(f, [format_comment])?;

            if comment.lines_after() > 1 {
                write!(f, [empty_line()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }

            // This override fully prints these leading comments.
            comment.mark_formatted();
        }

        Ok(())
    }

    /// Writes `$color/* c */ : red;` name/colon comments.
    fn fmt_colon_boundary(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let colon = self.node.colon_token();
        let comments = f.comments().clone();
        let colon_boundary_comments = comments.dangling_comments(self.node.syntax());

        if colon_boundary_comments.is_empty() {
            return write!(f, [colon.format()]);
        }

        for comment in colon_boundary_comments {
            self.fmt_name_boundary_comment(comment, f)?;
        }

        let should_break_before_colon = colon_boundary_comments
            .iter()
            .any(|comment| comment.kind().is_line() || comment.lines_after() > 0);

        if should_break_before_colon {
            write!(
                f,
                [dedent_to_root(&format_args![
                    hard_line_break(),
                    colon.format()
                ])]
            )
        } else {
            write!(
                f,
                [
                    maybe_space(has_source_gap_before_token(colon_boundary_comments, &colon)),
                    colon.format()
                ]
            )
        }
    }

    /// Writes one name/colon comment, as in `$color/* c */ : red;`.
    fn fmt_name_boundary_comment(
        &self,
        comment: &SourceComment<CssLanguage>,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let formatted = format_dangling_comment(comment);

        if comment.kind().is_line() {
            write!(f, [space(), formatted])
        } else {
            write!(f, [formatted])
        }
    }

    /// Writes `;`, preserving `$x: red !default /* c */ ;`.
    fn fmt_semicolon_boundary(
        &self,
        semicolon_token: Option<CssSyntaxToken>,
        preserve_source_gap_before_semicolon: bool,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        if let Some(semicolon) = semicolon_token.as_ref() {
            if preserve_source_gap_before_semicolon && has_block_comment_gap_before_token(semicolon)
            {
                write!(f, [space()])?;
            }

            write!(f, [semicolon.format()])
        } else {
            Ok(())
        }
    }
}
