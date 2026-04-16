---
"@biomejs/biome": patch
---

Type-aware lint rules now resolve members through `Partial<T>`, `Required<T>`, and `Readonly<T>` utility types, preserving optional, readonly, and nullable member flags.
