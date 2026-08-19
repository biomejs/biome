---
"@biomejs/biome": patch
---

Fixed Astro expressions containing a comment failing to parse.

```astro
<div>{/* block comment */ x}</div>
<div>{/* only a comment */}</div>
```

`{#`, `{/`, `{:` and `{@` are now only read as Svelte block openings in Svelte files. In HTML, Vue and Angular files they are ordinary text instead of a parse error.
