---
"@biomejs/biome": patch
---

Fixed [#10011](https://github.com/biomejs/biome/issues/10011): The `noThisInStatic` rule no longer reports `this` when it is used as the constructor target in `new this(...)`, which is required for inherited static factory methods.
