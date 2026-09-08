use crate::convert_ast::ConvertCtx;
use biome_rowan::{AstNode, Direction, TextRange, TriviaPieceKind};
use react_compiler_ast::common::{Comment, CommentData};

pub(crate) fn collect_comments(ctx: &ConvertCtx<'_>) -> Vec<Comment> {
    ctx.root
        .syntax()
        .descendants_tokens(Direction::Next)
        .flat_map(|token| {
            token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
        })
        .filter_map(|piece| {
            let range = piece.text_range();
            let text = piece.text();
            match piece.kind() {
                TriviaPieceKind::SingleLineComment => Some(Comment::CommentLine(comment_data(
                    ctx, text, range, "//", "",
                ))),
                TriviaPieceKind::MultiLineComment => Some(Comment::CommentBlock(comment_data(
                    ctx, text, range, "/*", "*/",
                ))),
                _ => None,
            }
        })
        .collect()
}

fn comment_data(
    ctx: &ConvertCtx<'_>,
    text: &str,
    range: TextRange,
    prefix: &str,
    suffix: &str,
) -> CommentData {
    let value = text
        .strip_prefix(prefix)
        .unwrap_or(text)
        .strip_suffix(suffix)
        .unwrap_or(text)
        .trim()
        .to_string();
    CommentData {
        value,
        start: Some(range.start().into()),
        end: Some(range.end().into()),
        loc: Some(ctx.source_location(range)),
    }
}
