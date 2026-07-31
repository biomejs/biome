---
"@biomejs/biome": patch
---

Fixed false positives in `noUnusedImports` and `useImportType` for Svelte components that reference types only inside the `<script generics="...">` attribute.
