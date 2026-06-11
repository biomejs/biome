---
"@biomejs/biome": patch
---

Fixed false positives in `noUnusedImports`, `noUnusedVariables`, and `useImportType` for Svelte components that use both a `<script module>` and a `<script>` block. The two blocks compile to a single module and share a top-level scope, so a binding (import, function, or variable) declared in one block and used only in the other is no longer reported as unused.
