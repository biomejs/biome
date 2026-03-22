---
"@biomejs/biome": patch
---

Fixed [#9385](https://github.com/biomejs/biome/issues/9385): [`noUselessEscapeInString`](https://biomejs.dev/linter/rules/no-useless-escape-in-string/) no longer incorrectly flags valid CSS hex escapes (e.g. `\e7bb`) as useless. The rule now recognizes all hex digits (`0-9`, `a-f`, `A-F`) as valid escape characters in CSS strings.
