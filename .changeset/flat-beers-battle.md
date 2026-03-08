---
"@biomejs/biome": minor
---

Added the `sortBareImports` option to [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/),
which allows bare imports to be sorted within other imports when set to `true`.

```diff
  /* `sortBareImports` set to `true */
- import "b";
  import "a";
+ import "b";
  import { A } from "a";
+ import "./file";
  import { Local } from "./file";
- import "./file";
```
