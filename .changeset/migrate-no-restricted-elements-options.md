---
"@biomejs/biome": patch
---

`biome migrate eslint` now preserves the element lists of `react/forbid-elements`, `svelte/no-restricted-html-elements`, and `vue/no-restricted-html-elements` when migrating them to [`noRestrictedElements`](https://biomejs.dev/linter/rules/no-restricted-elements/). When several of these rules are configured, their lists are merged.
