---
"@biomejs/biome": patch
---

Fixed [#8605](https://github.com/biomejs/biome/issues/8605): Text expressions in some template languages (`{{ expr }}` or `{ expr }`) at the top level of a HTML document no longer causes panicking.
