---
"@biomejs/biome": patch
---

Fixed [#6211](https://github.com/biomejs/biome/issues/6211): previously the
import organizer emitted broken code when it merged an import at the start of
the file with another import and placed the merged result after a third import.

The following code is now correctly organized:

```diff
- import { B } from "bc";
- import { C } from "bc";
  import { A } from "a";
+ import { B, C } from "bc";
```
