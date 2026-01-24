---
"@biomejs/biome": patch
---

Fixed `noUnusedVariables` to ignore type parameters declared in ambient contexts such as `declare module` blocks.
