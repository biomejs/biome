use crate::prelude::*;
use biome_formatter::comments::{
    CommentKind, CommentPlacement, CommentStyle, Comments, DecoratedComment, SourceComment,
};
use biome_formatter::formatter::Formatter;
use biome_formatter::{FormatResult, FormatRule, normalize_newlines, write};
use biome_rowan::SyntaxTriviaPieceComments;
use biome_toml_syntax::{TomlArray, TomlInlineTable, TomlLanguage, TomlSyntaxKind};

pub type TomlComments = Comments<TomlLanguage>;

#[derive(Default)]
pub struct FormatTomlLeadingComment;

impl FormatRule<SourceComment<TomlLanguage>> for FormatTomlLeadingComment {
    type Context = TomlFormatContext;

    /// Prints the comment without its trailing whitespace.
    ///
    /// Unlike the generic trivia formatting, this doesn't treat U+2028 and
    /// U+2029 as line breaks: TOML only recognizes LF and CRLF, so printing
    /// them as line breaks would end the comment early.
    fn fmt(
        &self,
        comment: &SourceComment<TomlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        let piece = comment.piece();
        let text = piece.text().trim_end_matches([' ', '\t']);
        write!(
            f,
            [syntax_token_cow_slice(
                // A comment only contains a `\r` in an invalid document.
                normalize_newlines(text, ['\r']),
                &piece.as_piece().token(),
                piece.text_range().start()
            )]
        )
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
        handle_empty_value_comment(comment)
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
            TomlInlineTable::cast_ref(enclosing).is_some_and(|table| table.members().is_empty())
        }
        _ => false,
    };

    if is_empty {
        CommentPlacement::dangling(enclosing.clone(), comment)
    } else {
        CommentPlacement::Default(comment)
    }
}
