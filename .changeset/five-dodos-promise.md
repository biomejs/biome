---
"@biomejs/biome": patch
---

Fixed [#6669](https://github.com/biomejs/biome/issues/6669): Added an exception to `noUnusedImports` to allow type augmentation imports.

```ts
import type {} from "@mui/lab/themeAugmentation";
```
