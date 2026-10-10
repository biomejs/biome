use crate::WorkspaceError;
use crate::db::WorkspaceDb;
use crate::file_handlers::{ParsedOrigin, UpdateSnippetsNodes};
use biome_markdown_syntax::{
    MarkdownLanguage, MarkdownSyntaxNode, MarkdownSyntaxToken, MdFencedCodeBlock, MdFrontmatter,
    MdHtmlBlock, MdRoot,
};
use biome_rowan::syntax::SyntaxTrivia;
use biome_rowan::{
    AstNode, AstNodeList, BatchMutation, NodeOrToken, SendNode, TextRange, TriviaPiece,
};

/// Writes the fixed code of snippets back into the Markdown document.
///
/// The code of a snippet is the whole literal token of its block, trivia
/// included, so the fixed code replaces that token. A snippet keeps its code
/// when the fixed code would change the lines that delimit the block, such as
/// the line break after the opening fence.
pub(crate) fn update_snippets(
    root: ParsedOrigin,
    workspace_db: WorkspaceDb,
    new_snippets: Vec<UpdateSnippetsNodes>,
) -> Result<SendNode, WorkspaceError> {
    let tree: MdRoot = root.tree(&workspace_db);
    let mut mutation = BatchMutation::new(tree.syntax().clone());

    for snippet in new_snippets {
        let Some(token) = snippet_literal(tree.syntax(), snippet.range) else {
            continue;
        };
        let Some(new_token) = fixed_literal(&token, &snippet.new_code) else {
            continue;
        };
        mutation.replace_token(token, new_token);
    }

    let root = mutation.commit();
    Ok(root
        .as_send()
        .expect("the root of a committed mutation is a root node"))
}

/// Returns the literal token holding the code of the snippet whose block spans
/// `range`: a fenced code block, the frontmatter, or an HTML block.
fn snippet_literal(root: &MarkdownSyntaxNode, range: TextRange) -> Option<MarkdownSyntaxToken> {
    let node = match root.covering_element(range) {
        NodeOrToken::Node(node) => node,
        NodeOrToken::Token(token) => token.parent()?,
    };
    node.ancestors().find_map(|node| {
        if let Some(block) = MdFencedCodeBlock::cast_ref(&node) {
            return (block.range() == range)
                .then(|| {
                    block
                        .content()
                        .first()?
                        .as_md_code_content()?
                        .value_token()
                        .ok()
                })
                .flatten();
        }
        if let Some(frontmatter) = MdFrontmatter::cast_ref(&node) {
            return (frontmatter.range() == range)
                .then(|| frontmatter.content().ok()?.value_token().ok())
                .flatten();
        }
        let block = MdHtmlBlock::cast_ref(&node)?;
        (block.range() == range)
            .then(|| block.content().ok()?.value_token().ok())
            .flatten()
    })
}

/// Returns a copy of `token` holding `code`, or `None` when `code` changes the
/// trivia of the token or the line breaks at the edges of its text.
///
/// The code of fenced code blocks and frontmatter starts with the line break
/// that ends the opening line and ends with the line break before the closing
/// line. Without them, the code would join the delimiter lines.
fn fixed_literal(token: &MarkdownSyntaxToken, code: &str) -> Option<MarkdownSyntaxToken> {
    let leading_trivia = token.leading_trivia();
    let trailing_trivia = token.trailing_trivia();
    let text = code
        .strip_prefix(leading_trivia.text())?
        .strip_suffix(trailing_trivia.text())?;
    let original = token.text_trimmed();
    let is_line_break = |character: char| matches!(character, '\n' | '\r');
    if original.starts_with(is_line_break) && !text.starts_with(is_line_break) {
        return None;
    }
    if original.ends_with(is_line_break) && !text.ends_with(is_line_break) {
        return None;
    }

    Some(MarkdownSyntaxToken::new_detached(
        token.kind(),
        code,
        trivia_pieces(&leading_trivia),
        trivia_pieces(&trailing_trivia),
    ))
}

fn trivia_pieces(trivia: &SyntaxTrivia<MarkdownLanguage>) -> Vec<TriviaPiece> {
    trivia
        .pieces()
        .map(|piece| TriviaPiece::new(piece.kind(), piece.text_len()))
        .collect()
}

#[cfg(test)]
#[path = "update_snippets.tests.rs"]
mod tests;
