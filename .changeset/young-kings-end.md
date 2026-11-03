---
"@biomejs/biome": patch
---

Fixed `.editorconfig` section globs with unbalanced braces. As in the EditorConfig reference implementation, all braces in such a glob are now matched literally, so `[{a,b}}.json]` only matches a file named `{a,b}}.json`.
