---
"@biomejs/biome": patch
---

Fixed part of [#13165](https://github.com/biomejs/biome/issues/13165): the CSS formatter no longer double-indents the arguments of a functional pseudo-class in a nested selector without a leading combinator, such as `&:has(...)` or `.a:not(...)`.
