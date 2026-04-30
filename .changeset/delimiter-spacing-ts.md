---
"@biomejs/biome": minor
---

Implemented `delimiterSpacing` for TypeScript. When enabled, Biome inserts spaces inside TypeScript angle brackets (e.g., `foo< T >()`), indexed access types (e.g., `T[ K ]`), mapped types, tuple types, type parameters, and index signatures. Only applies when the content fits on a single line. Empty delimiters are not affected.

```diff
- type Result = Map<string, number>;
+ type Result = Map< string, number >;
```
