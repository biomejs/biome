---
"@biomejs/biome": patch
---

Type-aware lint rules now resolve members through `Pick<T, K>` and `Omit<T, K>` utility types.
