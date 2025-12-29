---
"@biomejs/biome": patch
---

Fixed [#7020](https://github.com/biomejs/biome/issues/7020): The type limit check during flattening now correctly uses the actual type count instead of the loop index. Previously, the check would only trigger after 200,000 iterations rather than when the type store actually exceeded 200,000 types, allowing unbounded type growth and causing 100% CPU usage.
