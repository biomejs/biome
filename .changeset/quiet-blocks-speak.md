---
"@biomejs/biome": patch
---

Added the nursery rule [`useConsistentBlockLang`](https://biomejs.dev/linter/rules/use-consistent-block-lang/), which enforces the languages that the top-level blocks of Vue and Svelte components use. The `blocks` option configures the allowed languages per block, and the rule doesn't check anything until it's configured.

For example, with `{ "blocks": { "script": { "lang": ["ts"] } } }`, the following is reported:

```vue
<script lang="js"></script>
```
