---
"@biomejs/biome": patch
---

Fixed [#10131](https://github.com/biomejs/biome/issues/10131): Biome now correctly parses curried arrow functions in ternary consequents when the inner arrow's parameters use a destructuring pattern, e.g. `cond ? (x) => ({ a, b }) => body : alt`.
