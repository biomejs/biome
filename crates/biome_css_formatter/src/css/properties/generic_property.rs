use crate::comments::FormatCssLeadingComment;
use crate::prelude::*;
use crate::utils::component_value_list::{ValueListLayout, get_value_list_layout};
use biome_css_syntax::{
    AnyCssGenericPropertyValueOrExpression, CssGenericProperty, CssGenericPropertyFields,
    CssLanguage,
};
use biome_formatter::comments::SourceComment;
use biome_formatter::{FormatRefWithRule, format_args, write};
use biome_rowan::SyntaxResult;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssGenericProperty;
impl FormatNodeRule<CssGenericProperty> for FormatCssGenericProperty {
    fn fmt_fields(&self, node: &CssGenericProperty, f: &mut CssFormatter) -> FormatResult<()> {
        let CssGenericPropertyFields { name, .. } = node.as_fields();
        let colon_comments = CssPropertyColonComments::new(node);

        write!(f, [name.format()])?;
        colon_comments.fmt_colon_boundary(f)?;
        colon_comments.fmt_value_boundary(f)
    }

    fn fmt_dangling_comments(
        &self,
        _node: &CssGenericProperty,
        _f: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Printed by `CssPropertyColonComments` as `color/* a */: red`.
        Ok(())
    }

    fn fmt_trailing_comments(
        &self,
        _node: &CssGenericProperty,
        _f: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Printed by `CssPropertyColonComments` as `color:/* a */ red`.
        Ok(())
    }
}

/// Formats comments around a declaration colon.
///
/// Prettier preserves this boundary closely: `color/* a */:/* b */ red`.
struct CssPropertyColonComments<'a> {
    node: &'a CssGenericProperty,
}

impl<'a> CssPropertyColonComments<'a> {
    fn new(node: &'a CssGenericProperty) -> Self {
        Self { node }
    }

    /// Writes name/colon-boundary comments and the colon.
    ///
    /// Example: `color/* before */: red`.
    fn fmt_colon_boundary(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let colon = self.node.colon_token();
        // Clone the Rc-backed store so comment slices don't borrow `f` during writes.
        let comments = f.comments().clone();
        let colon_boundary_comments = comments.dangling_comments(self.node.syntax());

        if colon_boundary_comments.is_empty() {
            return write!(f, [colon.format()]);
        }

        for comment in colon_boundary_comments {
            let formatted = FormatRefWithRule::new(comment, FormatCssLeadingComment);

            if comment.kind().is_line() {
                write!(f, [space(), formatted])?;
            } else {
                write!(f, [formatted])?;
            }

            comment.mark_formatted();
        }

        // Prettier keeps `color // note\n: red` as a raw name/colon boundary.
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
            write!(f, [colon.format()])
        }
    }

    /// Writes colon/value-boundary comments and the value.
    ///
    /// Example: `color:/* after */ red`.
    fn fmt_value_boundary(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let value = self.node.value();
        // Clone the Rc-backed store so comment slices don't borrow `f` during writes.
        let comments = f.comments().clone();
        let value_boundary_comments = comments.trailing_comments(self.node.syntax());

        if value_boundary_comments.is_empty() {
            return write!(f, [space(), value.format()]);
        }

        for (index, comment) in value_boundary_comments.iter().enumerate() {
            self.fmt_value_boundary_comment(index, comment, f)?;
            comment.mark_formatted();
        }

        let should_break_before_value = value_boundary_comments
            .iter()
            .any(|comment| comment.kind().is_line())
            || value_list_needs_multiline_layout(&value, f);

        if should_break_before_value {
            write!(
                f,
                [indent(&format_args![hard_line_break(), &value.format()])]
            )
        } else {
            write!(f, [space(), value.format()])
        }
    }

    fn fmt_value_boundary_comment(
        &self,
        index: usize,
        comment: &SourceComment<CssLanguage>,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let formatted = FormatRefWithRule::new(comment, FormatCssLeadingComment);

        if comment.lines_before() > 0 {
            if comment.kind().is_line() {
                return write!(f, [hard_line_break(), formatted]);
            }

            // `color:\n/* note */ red` keeps the block comment at the raw column.
            return write!(
                f,
                [dedent_to_root(&format_args![hard_line_break(), formatted])]
            );
        }

        if index == 0 {
            write!(f, [maybe_space(self.has_source_gap_after_colon(comment))])?;
        } else {
            write!(f, [space()])?;
        }

        write!(f, [formatted])
    }

    /// Returns `true` for a source gap in `color: /* note */ red`.
    fn has_source_gap_after_colon(&self, comment: &SourceComment<CssLanguage>) -> bool {
        let Ok(colon_token) = self.node.colon_token() else {
            return false;
        };

        let comment_start = comment.piece().text_range().start();

        colon_token
            .trailing_trivia()
            .pieces()
            .take_while(|piece| piece.text_range().end() <= comment_start)
            .any(|piece| piece.is_whitespace() || piece.is_newline())
    }
}

/// Returns `true` when the value list needs multiline layout.
///
/// Example: `font-family: /* note */ Hiragino Sans, sans-serif`.
fn value_list_needs_multiline_layout(
    value: &SyntaxResult<AnyCssGenericPropertyValueOrExpression>,
    f: &CssFormatter,
) -> bool {
    let Some(list) = value
        .as_ref()
        .ok()
        .and_then(|value| value.as_css_generic_component_value_list())
    else {
        return false;
    };

    matches!(
        get_value_list_layout(list, f.comments(), f),
        ValueListLayout::OnePerLine
            | ValueListLayout::OneGroupPerLine
            | ValueListLayout::OneGroupPerLineWithDanglingComments
    )
}
