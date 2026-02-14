---
"@biomejs/biome": patch
---

Fixed [#9068](https://github.com/biomejs/biome/issues/9068): The `noVueDuplicateKeys` rule now correctly handles `toRefs(props)` patterns and no longer produces false positives when destructuring props, particularly in `<script setup>` blocks.
