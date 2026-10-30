---
"@biomejs/biome": patch
---

Fixed [#10896](https://github.com/biomejs/biome/issues/10896): Biome now parses quoted attribute values on top-level Svelte `<script>` and `<style>` tags as plain strings, like Svelte does, so `<script lang="ts" generics="T extends { name: string }">` no longer causes a parse error.
