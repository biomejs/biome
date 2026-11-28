//! Shared logic of the `useBannerComment` rule, implemented by every language
//! that supports block comments.

use biome_console::markup;
use biome_diagnostics::Category;
use biome_rowan::{
    BatchMutation, Language, SyntaxToken, SyntaxTriviaPiece, TriviaPieceKind, chain_trivia_pieces,
};

use crate::RuleDiagnostic;

/// Returns whether the file starts with one of the `accepted` banners.
///
/// `first_token` is the first token of the file that can carry the banner in
/// its leading trivia. The banner must be the first comment of the file, and
/// it must be a block comment (`/* ... */`).
pub fn has_banner<L: Language>(first_token: &SyntaxToken<L>, accepted: &[Box<str>]) -> bool {
    first_comment(first_token)
        .as_ref()
        .and_then(|comment| block_comment_body(comment.text()))
        .is_some_and(|body| {
            accepted
                .iter()
                .any(|expected| banner_matches(body, expected))
        })
}

/// Creates the diagnostic of a file that doesn't start with the required banner.
pub fn missing_banner_diagnostic<L: Language>(
    category: &'static Category,
    first_token: &SyntaxToken<L>,
) -> RuleDiagnostic {
    let range = first_comment(first_token)
        .map_or_else(|| first_token.text_range(), |comment| comment.text_range());

    RuleDiagnostic::new(
        category,
        range,
        markup! {
            "The file does not start with the required banner comment."
        },
    )
    .note(markup! {
        "A banner comment communicates licensing, copyright, or ownership information to anyone reading or auditing the file."
    })
}

/// Inserts `content` as a block comment at the top of the file, before any
/// existing comment.
///
/// When a token precedes `first_token` on the first line (e.g. a shebang),
/// the banner is inserted after the line break that ends that line.
pub fn insert_banner<L: Language>(
    mutation: &mut BatchMutation<L>,
    first_token: &SyntaxToken<L>,
    content: &str,
) -> Option<()> {
    let banner = if content.contains('\n') {
        format!("/*\n{content}\n*/")
    } else {
        format!("/* {content} */")
    };
    // Detached trivia pieces holding the banner and its line break.
    let banner_carrier = first_token.with_leading_trivia([
        (TriviaPieceKind::MultiLineComment, banner.as_str()),
        (TriviaPieceKind::Newline, "\n"),
    ]);

    let leading_trivia = first_token.leading_trivia();
    let skip = usize::from(
        first_token.prev_token().is_some()
            && leading_trivia
                .pieces()
                .next()
                .is_some_and(|piece| piece.is_newline()),
    );
    let new_first_token = first_token.with_leading_trivia_pieces(chain_trivia_pieces(
        chain_trivia_pieces(
            leading_trivia.pieces().take(skip),
            banner_carrier.leading_trivia().pieces(),
        ),
        leading_trivia.pieces().skip(skip),
    ));

    // The text edit of a replaced token doesn't cover its leading trivia, so
    // the parent node is replaced instead.
    let parent = first_token.parent()?;
    let new_parent = parent
        .clone()
        .replace_child(first_token.clone().into(), new_first_token.into())?;
    mutation.replace_element_discard_trivia(parent.into(), new_parent.into());
    Some(())
}

fn first_comment<L: Language>(token: &SyntaxToken<L>) -> Option<SyntaxTriviaPiece<L>> {
    token
        .leading_trivia()
        .pieces()
        .find(|piece| piece.is_comments())
}

/// Returns the text between `/*` and `*/`, or `None` if `comment` isn't a block comment.
fn block_comment_body(comment: &str) -> Option<&str> {
    comment.strip_prefix("/*")?.strip_suffix("*/")
}

/// Compares two banners line by line. Each line is trimmed, a leading `*` is
/// ignored, and blank lines are skipped.
fn banner_matches(actual: &str, expected: &str) -> bool {
    banner_lines(actual).eq(banner_lines(expected))
}

fn banner_lines(text: &str) -> impl Iterator<Item = &str> {
    text.lines().filter_map(|line| {
        let line = line.trim();
        let line = line.strip_prefix('*').unwrap_or(line).trim();
        (!line.is_empty()).then_some(line)
    })
}
