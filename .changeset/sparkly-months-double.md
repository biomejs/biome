---
"@biomejs/biome": patch
---

Fixed [#12187](https://github.com/biomejs/biome/issues/12187): [`noIrregularWhitespace`](https://biomejs.dev/linter/rules/no-irregular-whitespace) no longer reports irregular whitespace inside CSS and SCSS strings. It avoids duplicate diagnostics in nested rules and checks embedded CSS snippets.


