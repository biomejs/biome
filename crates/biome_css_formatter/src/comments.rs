use crate::prelude::*;
use biome_css_syntax::{AnyCssDeclarationName, CssLanguage, TextLen};
use biome_diagnostics::category;
use biome_formatter::comments::{
    is_doc_comment, CommentKind, CommentPlacement, CommentStyle, CommentTextPosition, Comments,
    DecoratedComment, SourceComment,
};
use biome_formatter::formatter::Formatter;
use biome_formatter::{write, FormatResult, FormatRule};
use biome_rowan::SyntaxTriviaPieceComments;
use biome_suppression::parse_suppression_comment;

pub type CssComments = Comments<CssLanguage>;

#[derive(Default)]
pub struct FormatCssLeadingComment;

impl FormatRule<SourceComment<CssLanguage>> for FormatCssLeadingComment {
    type Context = CssFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<CssLanguage>,
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
pub struct CssCommentStyle;

impl CommentStyle for CssCommentStyle {
    type Language = CssLanguage;

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

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        match comment.text_position() {
            CommentTextPosition::EndOfLine => handle_declaration_name_comment(comment),
            CommentTextPosition::OwnLine => handle_declaration_name_comment(comment),
            CommentTextPosition::SameLine => handle_declaration_name_comment(comment),
        }
    }
}

fn handle_declaration_name_comment(
    comment: DecoratedComment<CssLanguage>,
) -> CommentPlacement<CssLanguage> {
    match comment.preceding_node() {
        Some(following_node) if AnyCssDeclarationName::can_cast(following_node.kind()) => {
            CommentPlacement::leading(following_node.clone(), comment)
        }
        _ => CommentPlacement::Default(comment),
    }
}
