---
"@biomejs/biome": patch
---

Fixed [#12187](https://github.com/biomejs/biome/issues/12187): [`noIrregularWhitespace`](https://biomejs.dev/linter/rules/no-irregular-whitespace/) no longer reports irregular whitespace inside CSS strings, such as `content: "　"`.

The rule also no longer reports the same irregular whitespace more than once when it appears inside nested rules.
