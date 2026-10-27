use crate::prelude::*;
use biome_formatter::comments::{
    CommentKind, CommentPlacement, CommentStyle, Comments, DecoratedComment, SourceComment,
};
use biome_formatter::formatter::Formatter;
use biome_formatter::{FormatResult, FormatRule, write};
use biome_rowan::SyntaxTriviaPieceComments;
use biome_toml_syntax::{
    TomlArray, TomlArrayTable, TomlInlineTable, TomlLanguage, TomlSyntaxKind, TomlTable,
};

pub type TomlComments = Comments<TomlLanguage>;

#[derive(Default)]
pub struct FormatTomlLeadingComment;

impl FormatRule<SourceComment<TomlLanguage>> for FormatTomlLeadingComment {
    type Context = TomlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<TomlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        write!(f, [comment.piece().as_piece()])
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct TomlCommentStyle;

impl CommentStyle for TomlCommentStyle {
    type Language = TomlLanguage;

    /// TOML only has `#` comments, which run to the end of the line.
    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        CommentKind::Line
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        handle_table_header_comment(comment).or_else(handle_empty_value_comment)
    }
}

/// Makes a comment at the end of a table header, such as `[table] # comment`,
/// a dangling comment of the table, which prints it after the closing bracket.
///
/// By default, the comment would trail the header's key and be printed before
/// the closing bracket.
fn handle_table_header_comment(
    comment: DecoratedComment<TomlLanguage>,
) -> CommentPlacement<TomlLanguage> {
    let enclosing = comment.enclosing_node();
    let header = match enclosing.kind() {
        TomlSyntaxKind::TOML_TABLE => TomlTable::cast_ref(enclosing)
            .and_then(|table| Some((table.r_brack_token().ok()?, table.entries()))),
        TomlSyntaxKind::TOML_ARRAY_TABLE => TomlArrayTable::cast_ref(enclosing)
            .and_then(|table| Some((table.r_double_brack_token().ok()?, table.entries()))),
        _ => None,
    };

    let Some((close, entries)) = header else {
        return CommentPlacement::Default(comment);
    };

    // An end-of-line comment between the header and the first entry is on
    // the header line.
    let start = comment.piece().text_range().start();
    let is_on_header_line = comment.text_position().is_end_of_line()
        && start >= close.text_trimmed_range().end()
        && entries
            .first()
            .is_none_or(|entry| start < entry.syntax().text_trimmed_range().start());

    if is_on_header_line {
        CommentPlacement::dangling(enclosing.clone(), comment)
    } else {
        CommentPlacement::Default(comment)
    }
}

/// Makes the comments of an empty array or inline table dangling comments of
/// that value.
fn handle_empty_value_comment(
    comment: DecoratedComment<TomlLanguage>,
) -> CommentPlacement<TomlLanguage> {
    let enclosing = comment.enclosing_node();
    let is_empty = match enclosing.kind() {
        TomlSyntaxKind::TOML_ARRAY => {
            TomlArray::cast_ref(enclosing).is_some_and(|array| array.elements().is_empty())
        }
        TomlSyntaxKind::TOML_INLINE_TABLE => {
            TomlInlineTable::cast_ref(enclosing).is_some_and(|table| table.entries().is_empty())
        }
        _ => false,
    };

    if is_empty {
        CommentPlacement::dangling(enclosing.clone(), comment)
    } else {
        CommentPlacement::Default(comment)
    }
}
