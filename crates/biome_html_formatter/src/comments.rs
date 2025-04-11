use biome_diagnostics_categories::category;
use biome_formatter::{
    FormatRule,
    comments::{
        CommentKind, CommentPlacement, CommentStyle, Comments, DecoratedComment, SourceComment,
        is_alignable_comment,
    },
    prelude::*,
    write,
};
use biome_html_syntax::{HtmlLanguage, HtmlSyntaxKind};
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
            .any(|(key, ..)| key == category!("format"))
    }

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<HtmlLanguage>) -> CommentKind {
        CommentKind::Block
    }

    /// This allows us to override which comments are associated with which nodes.
    ///
    /// While every comment is directly attached to a **syntax token**, Biome actually builds a map of comments to **syntax nodes** separately. This map lives in [`HtmlComments`]. This is so that we can easily look up comments that are associated with a specific node. It's part of how suppression comments are handled.
    ///
    /// This method specifically, however, lets us fine tune which comments are associated with which nodes. This is useful when the default heuristic fails.
    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        // Fix trailing comments that are right before EOF being assigned to the wrong node.
        //
        // The issue is demonstrated in the example below.
        // ```html
        // Foo
        //
        // <!-- This comment gets assigned to the text node, despite it being actually attached to the EOF token. -->
        // ```
        if let Some(token) = comment.following_token() {
            if token.kind() == HtmlSyntaxKind::EOF {
                return CommentPlacement::trailing(comment.enclosing_node().clone(), comment);
            }
        }

        CommentPlacement::Default(comment)
    }
}
