---
"@biomejs/biome": patch
---

Fixed [#10892](https://github.com/biomejs/biome/issues/10892): [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) no longer reports a false positive when checking a member of a discriminated union that is accessed through a default type-only namespace import. The following code is no longer flagged:

```ts
import type Types from "./types";

declare function parse(): Types.Result<string>;
const result = parse();
if (!result.success) {
}
```
