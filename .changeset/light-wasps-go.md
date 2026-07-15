---
"@biomejs/biome": patch
---

Improved Biome's type inference for [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/). Biome now resolves members accessed through namespace imports and namespace re-exports, including cyclic export graphs. For example, `noFloatingPromises` now detects the unhandled Promise in `api.load()`:

```ts
// api.ts
export async function load() {}

// index.ts
import * as api from "./api.ts";
api.load();
```
