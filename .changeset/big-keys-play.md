---
"@biomejs/biome": patch
---

Fixed [#7381](https://github.com/biomejs/biome/issues/7381), now the [`useOptionalChain`](https://biomejs.dev/ja/linter/rules/use-optional-chain/) rule recognizes optional chaining using Yoda expressions (e.g., `undefined !== foo && foo.bar`).
