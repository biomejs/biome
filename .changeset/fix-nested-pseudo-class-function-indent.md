---
"@biomejs/biome": patch
---

Fixed [#13165](https://github.com/biomejs/biome/issues/13165): the CSS formatter no longer double-indents the arguments of functional pseudo-classes in nested selectors, such as `&:has(...)` or `.a:not(...)`. The output now matches Prettier.
