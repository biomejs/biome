---
"@biomejs/biome": patch
---

Added an exception to `noUnusedImports` to allow type augmentation imports. Fixes [#6669](https://github.com/biomejs/biome/issues/6669).

```ts
import type {} from "@mui/lab/themeAugmentation";
```
