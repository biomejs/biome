---
"@biomejs/biome": patch
---

Fixed [#10247](https://github.com/biomejs/biome/issues/10247): `biome check --write`/`biome lint --write` now correctly writes fixes for code inside an HTML attribute expression (for example a Svelte `onclick={...}` handler, or a mustache expression like `{count}`), instead of silently reporting the diagnostic as fixable and applying nothing.

For example, running `biome lint --write --unsafe` for [`useBlockStatements`](https://biomejs.dev/linter/rules/use-block-statements/) (an unsafe fix) on this Svelte component used to leave the file unchanged:

```svelte
<button onclick={() => { if (open) close(); }}>Close</button>
```
