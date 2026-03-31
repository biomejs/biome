---
"@biomejs/biome": patch
---

Fixed [#2245](https://github.com/biomejs/biome/issues/2245): Svelte `<script>` tag language detection when the `generics` attribute contains `>` characters (e.g., `<script lang="ts" generics="T extends Record<string, unknown>">`). Biome now correctly recognizes TypeScript in such script blocks.
