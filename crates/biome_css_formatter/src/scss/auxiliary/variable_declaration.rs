use crate::prelude::*;
use crate::utils::comment_trivia::{
    has_block_comment_gap_before_token, has_source_gap_before_token,
};
use biome_css_syntax::{
    CssLanguage, CssSyntaxToken, ScssVariableDeclaration, ScssVariableDeclarationFields,
};
use biome_formatter::comments::SourceComment;
use biome_formatter::trivia::format_dangling_comment;
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssVariableDeclaration;

impl FormatNodeRule<ScssVariableDeclaration> for FormatScssVariableDeclaration {
    fn fmt_fields(&self, node: &ScssVariableDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssVariableDeclarationFields {
            name,
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
        variable_comments.fmt_value_boundary(f)?;

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
        // No-op: boundary helpers print `$color/* c */: red;` and
        // `$font: /* c */ Arial;` around `:`, not at the declaration end.
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

    /// Writes declaration-leading comments on their own lines.
    ///
    /// ```scss
    /// /* note */
    /// $color: red;
    /// ```
    fn fmt_leading_comments(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let comments = f.comments().clone();
        let leading_comments = comments.leading_comments(self.node.syntax());

        for comment in leading_comments {
            write!(f, [format_dangling_comment(comment)])?;

            if comment.lines_after() > 1 {
                write!(f, [empty_line()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }
        }

        Ok(())
    }

    /// Writes comments between the variable name and `:`.
    ///
    /// ```scss
    /// $color/* c */: red;
    /// ```
    fn fmt_colon_boundary(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let colon = self.node.colon_token();
        let comments = f.comments().clone();
        let dangling_comments = comments.dangling_comments(self.node.syntax());
        let value_boundary_start = self.value_boundary_comment_start(dangling_comments);
        let (colon_boundary_comments, _) = dangling_comments.split_at(value_boundary_start);

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

    /// Writes one comment between the variable name and `:`.
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

    /// Writes comments between `:` and the variable value.
    ///
    /// ```scss
    /// $font:
    ///   // c
    ///   Arial;
    /// ```
    fn fmt_value_boundary(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let value = self.node.value();
        let comments = f.comments().clone();
        let dangling_comments = comments.dangling_comments(self.node.syntax());
        let value_boundary_start = self.value_boundary_comment_start(dangling_comments);
        let (_, value_boundary_comments) = dangling_comments.split_at(value_boundary_start);

        if value_boundary_comments.is_empty() {
            return write!(f, [space(), value.format()]);
        }

        let should_break_before_value = value_boundary_comments
            .iter()
            .any(|comment| comment.kind().is_line());

        for comment in value_boundary_comments {
            self.fmt_value_boundary_comment(comment, f)?;
        }

        if should_break_before_value {
            write!(f, [hard_line_break(), value.format()])
        } else {
            write!(f, [space(), value.format()])
        }
    }

    /// Returns `true` for comments between `:` and the value:
    ///
    /// ```scss
    /// $font:
    ///   // c
    ///   Arial;
    /// ```
    ///
    /// `$font: Arial; // c` stays a trailing declaration comment.
    fn is_value_boundary_comment(&self, comment: &SourceComment<CssLanguage>) -> bool {
        let Ok(colon) = self.node.colon_token() else {
            return false;
        };

        let Some(value_start) = self
            .node
            .value()
            .ok()
            .and_then(|value| value.syntax().first_token())
            .map(|token| token.text_trimmed_range().start())
        else {
            return false;
        };

        let comment_start = comment.piece().text_range().start();

        comment_start >= colon.text_trimmed_range().end() && comment_start < value_start
    }

    /// Writes one comment between `:` and the variable value.
    fn fmt_value_boundary_comment(
        &self,
        comment: &SourceComment<CssLanguage>,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let formatted = format_dangling_comment(comment);

        if comment.lines_before() > 0 {
            if comment.kind().is_line() {
                return write!(f, [indent(&format_args![hard_line_break(), formatted])]);
            }

            return write!(
                f,
                [dedent_to_root(&format_args![hard_line_break(), formatted])]
            );
        }

        write!(f, [space(), formatted])
    }

    /// Finds the first dangling comment between `:` and the value.
    fn value_boundary_comment_start(&self, comments: &[SourceComment<CssLanguage>]) -> usize {
        comments
            .iter()
            .position(|comment| self.is_value_boundary_comment(comment))
            .unwrap_or(comments.len())
    }

    /// Writes the declaration `;`, preserving `$x: red !default /* c */ ;`.
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
            write!(f, [token(";")])
        }
    }
}
