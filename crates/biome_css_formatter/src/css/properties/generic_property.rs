use crate::prelude::*;
use crate::utils::comment_trivia::has_source_gap_before_token;
use crate::utils::component_value_list::{ValueListLayout, get_value_list_layout};
use biome_css_syntax::{
    AnyCssGenericPropertyValueOrExpression, CssDeclaration, CssGenericProperty,
    CssGenericPropertyFields, CssLanguage, CssSupportsFeatureDeclaration,
};
use biome_formatter::comments::SourceComment;
use biome_formatter::trivia::format_dangling_comment;
use biome_formatter::{format_args, write};
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
        // No-op: `fmt_colon_boundary` prints `a { color/* a */: red; }`.
        Ok(())
    }

    fn fmt_trailing_comments(
        &self,
        _node: &CssGenericProperty,
        _f: &mut CssFormatter,
    ) -> FormatResult<()> {
        // No-op: `fmt_value_boundary` prints `a { color:/* a */ red; }`.
        Ok(())
    }
}

/// Formats declaration-colon comments like `a { color/* a */:/* b */ red; }`.
struct CssPropertyColonComments<'a> {
    node: &'a CssGenericProperty,
}

impl<'a> CssPropertyColonComments<'a> {
    fn new(node: &'a CssGenericProperty) -> Self {
        Self { node }
    }

    /// Writes name/colon comments and the colon, as in `a { color/* c */: red; }`.
    fn fmt_colon_boundary(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let colon = self.node.colon_token();
        // Clone the Rc-backed store so comment slices don't borrow `f` during writes.
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
                    maybe_space(
                        self.should_preserve_source_gap_before_colon()
                            && has_source_gap_before_token(colon_boundary_comments, &colon)
                    ),
                    colon.format()
                ]
            )
        }
    }

    /// Writes one name/colon comment, as in `a { color/* c */ : red; }`.
    ///
    /// `@supports (display /* c */: flex) {}` keeps a space before the comment.
    fn fmt_name_boundary_comment(
        &self,
        comment: &SourceComment<CssLanguage>,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let formatted = format_dangling_comment(comment);

        if comment.kind().is_line() || self.is_supports_feature_declaration() {
            write!(f, [space(), formatted])
        } else {
            write!(f, [formatted])
        }
    }

    /// Returns `true` when `a { color/* c */ : red; }` preserves the gap.
    ///
    /// Custom properties and `@supports` normalize the gap.
    fn should_preserve_source_gap_before_colon(&self) -> bool {
        if self
            .node
            .name()
            .is_ok_and(|name| name.syntax().text_trimmed().starts_with("--"))
        {
            return false;
        }

        if self.node.parent::<CssDeclaration>().is_none() {
            return false;
        }

        !self.is_supports_feature_declaration()
    }

    fn is_supports_feature_declaration(&self) -> bool {
        let Some(declaration) = self.node.parent::<CssDeclaration>() else {
            return false;
        };

        declaration
            .parent::<CssSupportsFeatureDeclaration>()
            .is_some()
    }

    /// Writes colon/value comments and the value, as in `a { color:/* c */ red; }`.
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
        let formatted = format_dangling_comment(comment);

        if comment.lines_before() > 0 {
            if comment.kind().is_line() {
                return write!(f, [hard_line_break(), formatted]);
            }

            // Keep the block comment at its raw column:
            // a { color:
            // /* note */ red; }
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

    /// Returns `true` for the gap in `a { color: /* note */ red; }`.
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

/// Returns `true` when `a { font-family: /* note */ Hiragino Sans, sans-serif; }` breaks.
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
