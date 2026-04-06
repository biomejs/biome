---
"@biomejs/biome": "patch"
---

Fixed [#9690](https://github.com/biomejs/biome/issues/9690): `biome check --write` no longer falsely reports "Fixed 1 file" on every run for HTML files with embedded `<style>` blocks. The fix skips applying `fix_file` formatting output when no lint/assist actions were produced, preventing the embedded CSS indentation from being mangled by the non-delegated formatting pass.
