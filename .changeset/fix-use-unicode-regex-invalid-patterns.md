---
"@biomejs/biome": patch
---

Fixed [#11933](https://github.com/biomejs/biome/issues/11933): [`useUnicodeRegex`](https://biomejs.dev/linter/rules/use-unicode-regex/) no longer offers the `u`-flag fix for patterns that are invalid in Unicode mode.

Patterns using non-Unicode (Annex B) syntax, such as `/{/`, `/\a/` or `/[\d-z]/`, throw a `SyntaxError` when the `u` flag is added. The rule still reports the diagnostic for these patterns, but the unsafe fix is withheld.
