---
"@biomejs/biome": patch
---

Fixed [#9196](https://github.com/biomejs/biome/issues/9196): `biome check --write --unsafe` no longer hangs forever when applying the [`noCommentText`](https://biomejs.dev/linter/rules/no-comment-text/) code fix.

The rule's fix now wraps the comment in a real JSX expression container (`{/* comment */}`) instead of re-inserting the braces as plain JSX text, so the fixed code is no longer reported again by the same rule.
