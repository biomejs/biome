---
"@biomejs/biome": patch
---

Fixed [#10739](https://github.com/biomejs/biome/issues/10739): [`useValidAutocomplete`](https://biomejs.dev/linter/rules/use-valid-autocomplete/) now recognizes React's camelCase `autoComplete` attribute. Previously the rule only matched the lowercase `autocomplete` spelling, so invalid autocomplete values written in idiomatic JSX/TSX were never reported.
