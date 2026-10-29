---
"@biomejs/biome": patch
---

Fixed [#12683](https://github.com/biomejs/biome/issues/12683): `.editorconfig` section globs with nested braces, such as `[{*.{json,toml},biome.jsonc}]`, are now supported. Previously, Biome ignored these sections. Biome also no longer skips the character that follows a bracket expression such as `[ab]`, so `[ab]/c` now only matches `a/c` and `b/c` relative to the `.editorconfig` file. When a section glob contains an unmatched brace, Biome now treats all its braces as literal characters, as the EditorConfig reference implementation does. For example, `[{a,{b,c}]` now only matches a file named `{a,{b,c}`.
