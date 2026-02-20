---
"@biomejs/biome": patch
---

Fixed [#9161](https://github.com/biomejs/biome/issues/9161): The Vue parser now correctly handles colon attributes like `xlink:href` and `xmlns:xlink` by parsing them as single attributes instead of splitting them into separate tokens.
