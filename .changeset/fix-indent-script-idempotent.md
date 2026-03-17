---
"@biomejs/biome": patch
---

Fixed [#9117](https://github.com/biomejs/biome/issues/9117): `biome check --write` no longer falsely reports Svelte and Vue files as changed when `html.formatter.indentScriptAndStyle` is enabled and the files are already correctly formatted.
