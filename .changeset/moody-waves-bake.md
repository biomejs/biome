---
"@biomejs/biome": patch
---

Fixed [#10038](https://github.com/biomejs/biome/issues/10038): [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) now sorts imports in TypeScript modules and declaration files.

```diff
  declare module "mymodule" {
-  	import type { B } from "b";
  	import type { A } from "a";
+  	import type { B } from "b";
  }
```
