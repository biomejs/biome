---
"@biomejs/biome": patch
---

Fixed [#12683](https://github.com/biomejs/biome/issues/12683): `.editorconfig` section globs with nested braces, such as `[{*.{js,json},biome.jsonc}]`, are now supported instead of being ignored.
