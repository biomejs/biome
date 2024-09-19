use biome_diagnostics_categories::category;
use biome_formatter::{
    comments::{
        is_alignable_comment, CommentKind, CommentPlacement, CommentStyle, Comments,
        DecoratedComment, SourceComment,
    },
    prelude::*,
    write, FormatRule,
};
use biome_html_syntax::HtmlLanguage;
use biome_rowan::{SyntaxTriviaPieceComments, TextLen};
use biome_suppression::parse_suppression_comment;

use crate::context::HtmlFormatContext;

pub type HtmlComments = Comments<HtmlLanguage>;

#[derive(Default)]
pub struct FormatHtmlLeadingComment;

impl FormatRule<SourceComment<HtmlLanguage>> for FormatHtmlLeadingComment {
    type Context = HtmlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<HtmlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        if is_alignable_comment(comment.piece()) {
            let mut source_offset = comment.piece().text_range().start();

            let mut lines = comment.piece().text().lines();

            // SAFETY: Safe, `is_alignable_comment` only returns `true` for multiline comments
            let first_line = lines.next().unwrap();
            write!(f, [dynamic_text(first_line.trim_end(), source_offset)])?;

            source_offset += first_line.text_len();

            // Indent the remaining lines by one space so that all `*` are aligned.
            write!(
                f,
                [&format_once(|f| {
                    for line in lines {
                        write!(
                            f,
                            [
                                hard_line_break(),
                                text(" "),
                                dynamic_text(line.trim(), source_offset)
                            ]
                        )?;

                        source_offset += line.text_len();
                    }

                    Ok(())
                })]
            )
        } else {
            write!(f, [comment.piece().as_piece()])
        }
    }
}

#[derive(Default)]
pub struct FormatHtmlTrailingComment;

impl FormatRule<SourceComment<HtmlLanguage>> for FormatHtmlTrailingComment {
    type Context = HtmlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<HtmlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        write!(f, [comment.piece().as_piece()])
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct HtmlCommentStyle;

impl CommentStyle for HtmlCommentStyle {
    type Language = HtmlLanguage;

    fn is_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, _)| key == category!("format"))
    }

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<HtmlLanguage>) -> CommentKind {
        CommentKind::Block
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        CommentPlacement::Default(comment)
    }
}
