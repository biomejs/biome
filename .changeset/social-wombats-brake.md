---
"@biomejs/biome": patch
---

Support setting `indent_size` to `tab` in `.editorconfig`, the following config will not cause error:

```editorconfig
root = true
[*]
indent_size = tab
```
