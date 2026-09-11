---
"@biomejs/biome": patch
---

Fixed [#7603](https://github.com/biomejs/biome/issues/7603): [`useSingleJsDocAsterisk`](https://biomejs.dev/linter/rules/use-single-js-doc-asterisk/) no longer reports asterisks that are part of JSDoc comment content, such as italic text, as extra line markers.
