---
"@biomejs/biome": patch
---

Add `Temporal` to the browser and worker JavaScript globals.

`Temporal` (TC39 Stage 4) has shipped in Chrome 127 and Firefox 139. It was
previously reported as an undeclared variable by `noUndeclaredVariables` when
used in browser or web-worker environments. The identifier is now recognised in
the `BROWSER`, `SERVICE_WORKER`, and `WEB_WORKER` global lists.
