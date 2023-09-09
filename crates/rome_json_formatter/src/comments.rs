use crate::prelude::*;
use biome_diagnostics::category;
use biome_suppression::parse_suppression_comment;
use rome_formatter::comments::{
    is_doc_comment, CommentKind, CommentStyle, Comments, SourceComment,
};
use rome_formatter::formatter::Formatter;
use rome_formatter::{write, FormatResult, FormatRule};
use rome_json_syntax::{JsonLanguage, TextLen};
use rome_rowan::SyntaxTriviaPieceComments;

pub type JsonComments = Comments<JsonLanguage>;

#[derive(Default)]
pub struct FormatJsonLeadingComment;

impl FormatRule<SourceComment<JsonLanguage>> for FormatJsonLeadingComment {
    type Context = JsonFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<JsonLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        if is_doc_comment(comment.piece()) {
            let mut source_offset = comment.piece().text_range().start();

            let mut lines = comment.piece().text().lines();

            // SAFETY: Safe, `is_doc_comment` only returns `true` for multiline comments
            let first_line = lines.next().unwrap();
            write!(f, [dynamic_text(first_line.trim_end(), source_offset)])?;

            source_offset += first_line.text_len();

            // Indent the remaining lines by one space so that all `*` are aligned.
            write!(
                f,
                [align(
                    1,
                    &format_once(|f| {
                        for line in lines {
                            write!(
                                f,
                                [hard_line_break(), dynamic_text(line.trim(), source_offset)]
                            )?;

                            source_offset += line.text_len();
                        }

                        Ok(())
                    })
                )]
            )
        } else {
            write!(f, [comment.piece().as_piece()])
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct JsonCommentStyle;

impl CommentStyle for JsonCommentStyle {
    type Language = JsonLanguage;

    fn is_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, _)| key == category!("format"))
    }

    fn get_comment_kind(comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        if comment.text().starts_with("/*") {
            if comment.has_newline() {
                CommentKind::Block
            } else {
                CommentKind::InlineBlock
            }
        } else {
            CommentKind::Line
        }
    }
}
