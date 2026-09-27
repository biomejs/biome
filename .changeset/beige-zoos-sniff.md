---
"@biomejs/biome": patch
---

Fixed [#11767](https://github.com/biomejs/biome/issues/11767): when the same file is open in several editors connected to the Biome daemon with different unsaved contents, formatting, code actions and diagnostics are now computed against the text of the editor that made the request. Previously, they could be computed against the text of another editor and the returned edits corrupted the document.
