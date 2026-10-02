use biome_analyze::{ApplySuppression, SuppressionAction};
use biome_markdown_syntax::{MarkdownLanguage, MarkdownSyntaxToken, MdRoot};
use biome_rowan::{AstNode, BatchMutation, TriviaPieceKind};

pub(crate) struct MarkdownSuppressionAction;

impl SuppressionAction for MarkdownSuppressionAction {
    type Language = MarkdownLanguage;

    fn suppression_top_level_comment(&self, suppression_text: &str) -> String {
        format!("<!-- {suppression_text}: <explanation> -->")
    }

    fn find_token_for_inline_suppression(
        &self,
        token: MarkdownSyntaxToken,
    ) -> Option<ApplySuppression<Self::Language>> {
        Some(ApplySuppression {
            token_has_trailing_comments: false,
            token_to_apply_suppression: token,
            should_insert_leading_newline: false,
        })
    }

    fn apply_inline_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        apply_suppression: ApplySuppression<Self::Language>,
        suppression_text: &str,
        suppression_reason: &str,
        _diagnostic_text_range: &biome_rowan::TextRange,
    ) {
        let ApplySuppression {
            token_to_apply_suppression,
            ..
        } = apply_suppression;

        if let Some(line_prefix) = suppression_line_prefix(&token_to_apply_suppression) {
            let comment = format!("<!-- {suppression_text}: {suppression_reason} -->");
            insert_comment_trivia(mutation, token_to_apply_suppression, &line_prefix, &comment);
        }
    }

    fn apply_top_level_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        token: MarkdownSyntaxToken,
        suppression_text: &str,
    ) {
        let Some(root) = token.parent().and_then(|parent| {
            parent
                .ancestors()
                .find_map(MdRoot::cast)
                .or_else(|| MdRoot::cast(parent))
        }) else {
            return;
        };
        let token = root
            .value()
            .syntax()
            .first_token()
            .or_else(|| root.eof_token().ok());
        if let Some(token) = token {
            insert_comment_trivia(mutation, token, &[], suppression_text);
        }
    }
}

fn suppression_line_prefix(token: &MarkdownSyntaxToken) -> Option<Vec<MarkdownSyntaxToken>> {
    let mut prefix = Vec::new();
    let mut previous = token.prev_token();
    while let Some(token) = previous {
        let text = token.text_trimmed();
        if text.contains(['\n', '\r']) {
            break;
        }
        if !text
            .chars()
            .all(|character| matches!(character, ' ' | '\t' | '>'))
        {
            return None;
        }
        previous = token.prev_token();
        prefix.push(token);
    }
    prefix.reverse();
    Some(prefix)
}

fn insert_comment_trivia(
    mutation: &mut BatchMutation<MarkdownLanguage>,
    token: MarkdownSyntaxToken,
    line_prefix: &[MarkdownSyntaxToken],
    comment: &str,
) {
    let mut trivia = vec![
        (TriviaPieceKind::MultiLineComment, comment),
        (TriviaPieceKind::Newline, "\n"),
    ];
    trivia.extend(
        line_prefix
            .iter()
            .map(|token| (TriviaPieceKind::Whitespace, token.text_trimmed())),
    );
    let leading_trivia: Vec<_> = token.leading_trivia().pieces().collect();
    trivia.extend(
        leading_trivia
            .iter()
            .map(|piece| (piece.kind(), piece.text())),
    );
    let new_token = token.with_leading_trivia(trivia);
    mutation.replace_token_discard_trivia(token, new_token);
}
