---
"@biomejs/biome": patch
---

Changed the `useSimplifiedLogicExpression` fix to unsafe because removing boolean fallbacks can change TypeScript assignability for values such as `boolean | undefined`.
