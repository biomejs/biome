---
"@biomejs/biome": patch
---

Fixed [#12993](https://github.com/biomejs/biome/issues/12993): Svelte `{@const}` tags with TypeScript type annotations, such as `{@const doubled: number = item * 2}`, now parse and format in components with `<script lang="ts">`.
