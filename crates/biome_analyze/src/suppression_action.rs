use crate::SuppressionCommentEmitterPayload;
use crate::rule::SuppressAction;
use biome_console::markup;
use biome_rowan::{
    AstNode, BatchMutation, BatchMutationExt, Language, SyntaxToken, TextLen, TextRange,
    TokenAtOffset, TriviaPiece, TriviaPieceKind,
};

pub trait SuppressionAction {
    type Language: Language;

    fn inline_suppression(&self, payload: SuppressionCommentEmitterPayload<Self::Language>) {
        let SuppressionCommentEmitterPayload {
            token_offset,
            mutation,
            suppression_text,
            diagnostic_text_range,
            suppression_reason,
        } = payload;

        // retrieve the most suited, leftest token where the diagnostics was emitted
        let original_token = self.get_token_from_offset(token_offset, diagnostic_text_range);

        // considering that our suppression system works via lines, we need to look for the first newline,
        // so we can place the comment there
        let apply_suppression = original_token.as_ref().and_then(|original_token| {
            self.find_token_for_inline_suppression(original_token.clone())
        });

        if let Some(apply_suppression) = apply_suppression {
            self.apply_inline_suppression(
                mutation,
                apply_suppression,
                suppression_text,
                suppression_reason,
            );
        }
    }

    /// Finds the first token, starting with the current token and traversing backwards,
    /// until it find one that has a leading newline trivia.
    ///
    /// Sometimes, the offset is between tokens, we need to decide which one to take.
    ///
    /// For example:
    /// ```jsx
    /// function f() {
    ///     return <div
    ///     ><img /> {/* <--- diagnostic emitted in this line */}
    ///     </div>
    /// }
    /// ```
    ///
    /// In these case it's best to peek the right token, because it belongs to the node where error actually occurred,
    /// and becomes easier to add the suppression comment.
    fn get_token_from_offset(
        &self,
        token_at_offset: TokenAtOffset<SyntaxToken<Self::Language>>,
        diagnostic_text_range: &TextRange,
    ) -> Option<SyntaxToken<Self::Language>> {
        match token_at_offset {
            TokenAtOffset::None => None,
            TokenAtOffset::Single(token) => Some(token),
            TokenAtOffset::Between(left_token, right_token) => {
                let chosen_token =
                    if right_token.text_range().start() == diagnostic_text_range.start() {
                        right_token
                    } else {
                        left_token
                    };
                Some(chosen_token)
            }
        }
    }

    fn find_token_for_inline_suppression(
        &self,
        original_token: SyntaxToken<Self::Language>,
    ) -> Option<ApplySuppression<Self::Language>>;

    fn apply_inline_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        apply_suppression: ApplySuppression<Self::Language>,
        suppression_text: &str,
        suppression_reason: &str,
    );

    fn apply_top_level_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        token: SyntaxToken<Self::Language>,
        suppression_text: &str,
    ) {
        let has_comments = token
            .leading_trivia()
            .pieces()
            .any(|trivia| trivia.is_comments());

        let mut text = String::new();
        let new_trivia = if has_comments {
            new_trivia_for_top_suppression_with_comments(&token, &mut text, suppression_text)
        } else {
            new_trivia_for_top_suppression(&token, &mut text, suppression_text)
        };

        let new_token = SyntaxToken::new_detached(token.kind(), text.as_str(), new_trivia, [])
            .with_trailing_trivia_pieces(token.trailing_trivia().pieces());
        mutation.replace_token_discard_trivia(token, new_token);
    }

    /// Returns the whole top level comment, based on the language
    fn suppression_top_level_comment(&self, _suppression_text: &str) -> String;
}

/// Convenient type to store useful information
pub struct ApplySuppression<L: Language> {
    /// If the token is followed by trailing comments
    pub token_has_trailing_comments: bool,
    /// The token to attach the suppression
    pub token_to_apply_suppression: SyntaxToken<L>,
    /// If the suppression should have a leading newline
    pub should_insert_leading_newline: bool,
}

/// Generates new trivia from a syntax token that contains leading comments
fn new_trivia_for_top_suppression_with_comments<L: Language>(
    token: &SyntaxToken<L>,
    text: &mut String,
    suppression_text: &str,
) -> Vec<TriviaPiece> {
    let mut new_trivia = vec![];
    let mut after_comment = false;
    let mut trivia_applied = false;
    let pieces = token.leading_trivia().pieces();
    for trivia in pieces {
        if trivia.is_comments() {
            after_comment = true
        }

        if !trivia.is_comments() && after_comment && !trivia_applied {
            new_trivia.push(TriviaPiece::newline(1));
            text.push('\n');
            new_trivia.push(TriviaPiece::multi_line_comment(suppression_text.text_len()));
            text.push_str(suppression_text);
            after_comment = false;
            trivia_applied = true
        }

        new_trivia.push(TriviaPiece::new(trivia.kind(), trivia.text_len()));
        text.push_str(trivia.text());
    }
    text.push_str(token.text_trimmed());
    new_trivia
}

/// Generates new trivia from a syntax token that doesn't have any leading comments
fn new_trivia_for_top_suppression<L: Language>(
    token: &SyntaxToken<L>,
    text: &mut String,
    suppression_text: &str,
) -> Vec<TriviaPiece> {
    let mut new_trivia = vec![
        TriviaPiece::new(
            TriviaPieceKind::SingleLineComment,
            suppression_text.text_len(),
        ),
        TriviaPiece::newline(1),
    ];
    text.push_str(suppression_text);
    text.push('\n');
    for trivia in token.leading_trivia().pieces() {
        new_trivia.push(TriviaPiece::new(trivia.kind(), trivia.text_len()));
        text.push_str(trivia.text());
    }
    text.push_str(token.text_trimmed());
    new_trivia
}

/// Build an inline suppression (`biome-ignore`) action for the given rule category.
///
/// This is a shared helper used by both the native `Rule` trait and the plugin
/// signal path so the suppression text/message format is defined in one place.
pub(crate) fn make_inline_suppression<L: Language>(
    rule_category: &str,
    kind_label: &str,
    root: &<L as Language>::Root,
    text_range: &TextRange,
    suppression_action: &dyn SuppressionAction<Language = L>,
    suppression_reason: &str,
) -> SuppressAction<L> {
    let suppression_text = format!("biome-ignore {rule_category}");
    let root = root.clone();
    let token = root.syntax().token_at_offset(text_range.start());
    let mut mutation = root.begin();
    suppression_action.inline_suppression(SuppressionCommentEmitterPayload {
        suppression_text: suppression_text.as_str(),
        mutation: &mut mutation,
        token_offset: token,
        diagnostic_text_range: text_range,
        suppression_reason,
    });
    SuppressAction {
        mutation,
        message: markup! { "Suppress " {kind_label} " " {rule_category} " for this line." }
            .to_owned(),
    }
}

/// Build a top-level suppression (`biome-ignore-all`) action for the given rule category.
///
/// Returns `None` when the root has no tokens (empty file).
pub(crate) fn make_top_level_suppression<L: Language>(
    rule_category: &str,
    kind_label: &str,
    root: &<L as Language>::Root,
    suppression_action: &dyn SuppressionAction<Language = L>,
) -> Option<SuppressAction<L>> {
    let suppression_text = format!("biome-ignore-all {rule_category}");
    let root = root.clone();
    let first_token = root.syntax().first_token()?;
    let mut mutation = root.begin();
    let comment = suppression_action.suppression_top_level_comment(suppression_text.as_str());
    suppression_action.apply_top_level_suppression(&mut mutation, first_token, comment.as_str());
    Some(SuppressAction {
        mutation,
        message: markup! { "Suppress " {kind_label} " " {rule_category} " for the whole file." }
            .to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_console::MarkupBuf;
    use biome_rowan::TriviaPiece;
    use biome_rowan::raw_language::{
        RawLanguage, RawLanguageKind, RawLanguageRoot, RawSyntaxTreeBuilder,
    };
    use std::cell::Cell;

    /// Extract the plain text content from a `MarkupBuf` (ignoring markup tags).
    fn markup_to_string(markup: &MarkupBuf) -> String {
        markup.0.iter().map(|node| node.content.as_str()).collect()
    }

    /// A stub `SuppressionAction` for `RawLanguage` that records whether its
    /// methods were called but otherwise performs no real mutations.
    struct RecordingAction {
        top_level_comment_called: Cell<bool>,
        top_level_comment_arg: Cell<Option<String>>,
    }

    impl RecordingAction {
        fn new() -> Self {
            Self {
                top_level_comment_called: Cell::new(false),
                top_level_comment_arg: Cell::new(None),
            }
        }
    }

    impl SuppressionAction for RecordingAction {
        type Language = RawLanguage;

        fn find_token_for_inline_suppression(
            &self,
            _: SyntaxToken<Self::Language>,
        ) -> Option<ApplySuppression<Self::Language>> {
            // Return None — inline suppression is a no-op at the mutation level
            // but the helper still exercises the code path.
            None
        }

        fn apply_inline_suppression(
            &self,
            _: &mut BatchMutation<Self::Language>,
            _: ApplySuppression<Self::Language>,
            _: &str,
            _: &str,
        ) {
            unreachable!("apply_inline_suppression should not be called when find returns None")
        }

        fn apply_top_level_suppression(
            &self,
            _: &mut BatchMutation<Self::Language>,
            _: SyntaxToken<Self::Language>,
            _: &str,
        ) {
            // no-op — we only care that the method was reached
        }

        fn suppression_top_level_comment(&self, suppression_text: &str) -> String {
            self.top_level_comment_called.set(true);
            self.top_level_comment_arg
                .set(Some(suppression_text.to_string()));
            format!("// {suppression_text}")
        }
    }

    /// Build a minimal `RawLanguageRoot` with a single token.
    fn make_root_with_token() -> RawLanguageRoot {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::EXPRESSION_LIST);
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token_with_trivia(
            RawLanguageKind::STRING_TOKEN,
            "\n\"hello\"",
            &[TriviaPiece::newline(1)],
            &[],
        );
        builder.finish_node();
        builder.finish_node();
        builder.finish_node();
        RawLanguageRoot::unwrap_cast(builder.finish())
    }

    /// Build an empty `RawLanguageRoot` with no tokens.
    fn make_empty_root() -> RawLanguageRoot {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::EXPRESSION_LIST);
        builder.finish_node();
        builder.finish_node();
        RawLanguageRoot::unwrap_cast(builder.finish())
    }

    #[test]
    fn inline_suppression_message_contains_rule_category() {
        let root = make_root_with_token();
        let action = RecordingAction::new();
        let range = TextRange::new(1.into(), 8.into());

        let result = make_inline_suppression::<RawLanguage>(
            "lint/complexity/useWhile",
            "rule",
            &root,
            &range,
            &action,
            "<explanation>",
        );

        let msg = markup_to_string(&result.message);
        assert!(
            msg.contains("lint/complexity/useWhile"),
            "expected category in message, got: {msg}"
        );
        assert!(
            msg.contains("for this line"),
            "expected 'for this line' in message, got: {msg}"
        );
    }

    #[test]
    fn inline_suppression_message_uses_action_label() {
        let root = make_root_with_token();
        let action = RecordingAction::new();
        let range = TextRange::new(1.into(), 8.into());

        let result = make_inline_suppression::<RawLanguage>(
            "action/source/organizeImports",
            "action",
            &root,
            &range,
            &action,
            "<explanation>",
        );

        let msg = markup_to_string(&result.message);
        assert!(
            msg.contains("Suppress action"),
            "expected 'Suppress action' in message, got: {msg}"
        );
    }

    #[test]
    fn inline_suppression_plugin_category() {
        let root = make_root_with_token();
        let action = RecordingAction::new();
        let range = TextRange::new(1.into(), 8.into());

        let result = make_inline_suppression::<RawLanguage>(
            "lint/plugin/myRule",
            "rule",
            &root,
            &range,
            &action,
            "<explanation>",
        );

        let msg = markup_to_string(&result.message);
        assert!(
            msg.contains("lint/plugin/myRule"),
            "expected plugin category in message, got: {msg}"
        );
    }

    #[test]
    fn top_level_suppression_message_format() {
        let root = make_root_with_token();
        let action = RecordingAction::new();

        let result = make_top_level_suppression::<RawLanguage>(
            "lint/complexity/useWhile",
            "rule",
            &root,
            &action,
        );

        let result = result.expect("should return Some for non-empty root");
        let msg = markup_to_string(&result.message);
        assert!(
            msg.contains("for the whole file"),
            "expected 'for the whole file' in message, got: {msg}"
        );
        assert!(
            msg.contains("lint/complexity/useWhile"),
            "expected category in message, got: {msg}"
        );
    }

    #[test]
    fn top_level_suppression_returns_none_for_empty_root() {
        let root = make_empty_root();
        let action = RecordingAction::new();

        let result = make_top_level_suppression::<RawLanguage>(
            "lint/complexity/useWhile",
            "rule",
            &root,
            &action,
        );

        assert!(result.is_none(), "expected None for empty root");
    }

    #[test]
    fn top_level_suppression_calls_suppression_action() {
        let root = make_root_with_token();
        let action = RecordingAction::new();
        let category = "lint/style/useConst";

        let _ = make_top_level_suppression::<RawLanguage>(category, "rule", &root, &action);

        assert!(
            action.top_level_comment_called.get(),
            "suppression_top_level_comment should have been called"
        );
        let arg = action.top_level_comment_arg.take().unwrap();
        assert_eq!(
            arg,
            format!("biome-ignore-all {category}"),
            "suppression_top_level_comment was called with wrong text"
        );
    }
}
