use biome_diagnostics::category;
use biome_formatter::comments::{
    CommentKind, CommentPlacement, CommentStyle, CommentTextPosition, Comments, DecoratedComment,
    SourceComment,
};
use biome_formatter::formatter::Formatter;
use biome_formatter::{FormatResult, FormatRule, write};
use biome_rowan::AstNode;
use biome_rowan::{SyntaxTriviaPieceComments, TextSize};
use biome_suppression::{SuppressionKind, parse_suppression_comment};
use biome_yaml_syntax::{YamlDocument, YamlLanguage, YamlRoot};

use crate::prelude::*;

pub type YamlComments = Comments<YamlLanguage>;

#[derive(Default)]
pub struct FormatYamlLeadingComment;

impl FormatRule<SourceComment<YamlLanguage>> for FormatYamlLeadingComment {
    type Context = YamlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<YamlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        write!(f, [comment.piece().as_piece()])
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct YamlCommentStyle;

impl CommentStyle for YamlCommentStyle {
    type Language = YamlLanguage;

    fn is_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .filter(|suppression| suppression.kind == SuppressionKind::Classic)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, ..)| key == category!("format"))
    }

    fn is_global_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .filter(|suppression| suppression.kind == SuppressionKind::All)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, ..)| key == category!("format"))
    }

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        CommentKind::Line
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        handle_global_suppression(comment)
            .or_else(handle_document_comment)
            .or_else(handle_end_of_line_comment)
    }
}

/// Handles comments that are attached to the marker tokens (`---`, `...`) or
/// directives of a [YamlDocument].
fn handle_document_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    let Some(document) = YamlDocument::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };
    let comment_start = comment.piece().text_range().start();

    // Comments following the `...` document end marker belong to the document,
    // so they are printed after the marker (which is then kept).
    if let Some(dotdotdot) = document.dotdotdot_token()
        && comment_start > dotdotdot.text_trimmed_range().start()
    {
        return CommentPlacement::trailing(document.syntax().clone(), comment);
    }

    if let Some(dashdashdash) = document.dashdashdash_token() {
        if comment_start < dashdashdash.text_trimmed_range().start() {
            // Comments between the last directive and the `---` marker stay
            // with the directive so they aren't moved after the marker.
            if let Some(directive) = document.directives().iter().last()
                && directive.range().end() <= comment_start
            {
                return CommentPlacement::trailing(directive.syntax().clone(), comment);
            }

            // Comments preceding the `---` marker of a document without
            // directives lead the whole document.
            return CommentPlacement::leading(document.syntax().clone(), comment);
        }

        // Comments between the `---` marker and the document content are
        // printed right after the marker.
        let before_content = document
            .node()
            .is_none_or(|content| comment_start < content.range().start());
        if before_content {
            return CommentPlacement::dangling(document.syntax().clone(), comment);
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_end_of_line_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    if comment.text_position() != CommentTextPosition::EndOfLine {
        return CommentPlacement::Default(comment);
    }

    if let Some(preceding_node) = comment.preceding_node() {
        return CommentPlacement::trailing(preceding_node.clone(), comment);
    }

    CommentPlacement::Default(comment)
}

fn handle_global_suppression(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    let node = comment.enclosing_node();

    if node.text_range_with_trivia().start() == TextSize::from(0) {
        let has_global_suppression = node.first_leading_trivia().is_some_and(|trivia| {
            trivia
                .pieces()
                .filter(|piece| piece.is_comments())
                .any(|piece| YamlCommentStyle::is_global_suppression(piece.text()))
        });
        let root = node.ancestors().find_map(YamlRoot::cast);
        if let Some(root) = root
            && has_global_suppression
        {
            return CommentPlacement::leading(root.syntax().clone(), comment);
        }
    }

    CommentPlacement::Default(comment)
}
