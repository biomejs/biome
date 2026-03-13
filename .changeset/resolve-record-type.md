---
"@biomejs/biome": patch
---

Fixed [#6606](https://github.com/biomejs/biome/issues/6606): The type inference engine now resolves `Record<K, V>` types, synthesizing them as object types with index signatures. This improves accuracy for type-aware lint rules such as `noFloatingPromises`, `noMisusedPromises`, `useAwaitThenable`, and `useArraySortCompare` when operating on Record-typed values.
