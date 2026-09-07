use biome_rowan::{Direction, Language, SyntaxNode, SyntaxToken, TextSize};
use indexmap::IndexSet;

/// Tracks the ranges of the formatted (including replaced or tokens formatted as verbatim) tokens.
///
/// This implementation uses the fact that no two tokens can have an overlapping range to avoid the need for an interval tree.
/// Thus, testing if a token has already been formatted only requires testing if a token starting at the same offset has been formatted.
#[derive(Debug, Clone, Default)]
pub struct PrintedTokens {
    /// Key: Start of a token's range
    offsets: IndexSet<TextSize>,
    disabled: bool,
}

#[derive(Copy, Clone)]
pub struct PrintedTokensSnapshot {
    len: usize,
    disabled: bool,
}

impl PrintedTokens {
    /// Tracks a formatted token
    ///
    /// ## Panics
    /// If this token has been formatted before.
    pub fn track_token<L: Language>(&mut self, token: &SyntaxToken<L>) {
        if self.disabled {
            return;
        }

        let range = token.text_trimmed_range();
        if range.is_empty() {
            // Ignore zero-width tokens
            return;
        }

        if !self.offsets.insert(range.start()) {
            panic!(
                "You tried to print the token '{token:?}' twice, and this is not valid.\
                \nYou may need to memoize the token if you are writing it to multiple buffers at the same time."
            );
        }
    }

    /// Enables or disables the assertion tracking
    pub(crate) fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    pub(crate) fn is_disabled(&self) -> bool {
        self.disabled
    }

    pub(crate) fn snapshot(&self) -> PrintedTokensSnapshot {
        PrintedTokensSnapshot {
            len: self.offsets.len(),
            disabled: self.disabled,
        }
    }

    pub(crate) fn restore(&mut self, snapshot: PrintedTokensSnapshot) {
        let PrintedTokensSnapshot { len, disabled } = snapshot;

        self.offsets.truncate(len);
        self.disabled = disabled
    }

    /// Asserts that all tokens of the passed in node have been tracked
    ///
    /// ## Panics
    /// If any descendant token of `root` hasn't been tracked
    pub fn assert_all_tracked<L: Language>(&self, root: &SyntaxNode<L>) {
        let mut offsets = self.offsets.clone();

        for token in root.descendants_tokens(Direction::Next) {
            let range = token.text_trimmed_range();
            if range.is_empty() {
                // Ignore zero-width tokens
                continue;
            }

            if !offsets.swap_remove(&range.start()) {
                panic!(
                    "token has not been seen by the formatter: {token:#?}.\
                        \nUse `format_replaced` if you want to replace a token from the formatted output.\
                        \nUse `format_removed` if you want to remove a token from the formatted output.\n\
                        parent: {:#?}",
                    token.parent()
                )
            }
        }

        if let Some(offset) = offsets.into_iter().next() {
            panic!(
                "tracked offset {offset:?} doesn't match any token of {root:#?}. Have you passed a token from another tree?"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, parse_module};

    #[test]
    fn assertion_preserves_tracking_order_and_snapshots() {
        let root = parse_module("first; second; third;", JsParserOptions::default()).syntax();
        let tokens: Vec<_> = root.descendants_tokens(Direction::Prev).collect();
        let mut tracked = PrintedTokens::default();

        for token in &tokens[..3] {
            tracked.track_token(token);
        }
        let snapshot = tracked.snapshot();
        let saved_offsets: Vec<_> = tracked.offsets.iter().copied().collect();

        for token in &tokens[3..] {
            tracked.track_token(token);
        }
        let all_offsets: Vec<_> = tracked.offsets.iter().copied().collect();

        tracked.assert_all_tracked(&root);
        tracked.assert_all_tracked(&root);
        assert_eq!(
            tracked.offsets.iter().copied().collect::<Vec<_>>(),
            all_offsets
        );

        tracked.set_disabled(true);
        tracked.restore(snapshot);
        assert!(!tracked.is_disabled());
        assert_eq!(
            tracked.offsets.iter().copied().collect::<Vec<_>>(),
            saved_offsets
        );

        for token in &tokens[3..] {
            tracked.track_token(token);
        }
        tracked.assert_all_tracked(&root);
    }

    #[test]
    #[should_panic(expected = "token has not been seen by the formatter")]
    fn assertion_rejects_missing_token() {
        let root = parse_module("first; second;", JsParserOptions::default()).syntax();
        let mut tracked = PrintedTokens::default();
        for token in root.descendants_tokens(Direction::Next).skip(1) {
            tracked.track_token(&token);
        }
        tracked.assert_all_tracked(&root);
    }

    #[test]
    #[should_panic(expected = "doesn't match any token")]
    fn assertion_rejects_extra_offset() {
        let root = parse_module("first;", JsParserOptions::default()).syntax();
        let mut tracked = PrintedTokens::default();
        for token in root.descendants_tokens(Direction::Next) {
            tracked.track_token(&token);
        }
        tracked
            .offsets
            .insert(root.text_trimmed_range().end() + TextSize::from(1));
        tracked.assert_all_tracked(&root);
    }
}
