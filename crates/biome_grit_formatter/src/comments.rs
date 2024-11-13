use crate::GritFormatContext;

use biome_formatter::comments::CommentKind;
use biome_formatter::{
    comments::{is_doc_comment, CommentStyle, Comments, SourceComment},
    prelude::*,
    prelude::{align, dynamic_text, format_once, hard_line_break, Formatter},
    write, FormatResult, FormatRule,
};
use biome_grit_syntax::GritLanguage;
use biome_rowan::TextLen;

pub type GritComments = Comments<GritLanguage>;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct GritCommentStyle;

impl CommentStyle for GritCommentStyle {
    type Language = GritLanguage;

    fn is_suppression(_text: &str) -> bool {
        false
    }

    fn get_comment_kind(
        _comment: &biome_rowan::SyntaxTriviaPieceComments<Self::Language>,
    ) -> CommentKind {
        CommentKind::Line
    }

    fn place_comment(
        &self,
        comment: biome_formatter::comments::DecoratedComment<Self::Language>,
    ) -> biome_formatter::comments::CommentPlacement<Self::Language> {
        biome_formatter::comments::CommentPlacement::Default(comment)
    }
}

#[derive(Default)]
pub struct FormatGritLeadingComment;

impl FormatRule<SourceComment<GritLanguage>> for FormatGritLeadingComment {
    type Context = GritFormatContext;
    // Copied and pasted this from the css formatter, not sure how much this needs to change.
    fn fmt(
        &self,
        comment: &SourceComment<GritLanguage>,
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
