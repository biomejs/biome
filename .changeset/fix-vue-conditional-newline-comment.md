---
"@biomejs/biome": patch
---

Fixed a bug in `useVueValidVElse` and `useVueValidVElseIf` rules where conditional directives were incorrectly flagged as invalid when separated from their preceding `v-if`/`v-else-if` element by whitespace, newlines, or comments.
