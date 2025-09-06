---
"@biomejs/biome": patch
---

Fixed [#7212](https://github.com/biomejs/biome/issues/7212), now the [`useOptionalChain`](https://biomejs.dev/ja/linter/rules/use-optional-chain/) rule recognizes optional chaining using `typeof` (e.g., `typeof foo !== 'undefined' && foo.bar`).
