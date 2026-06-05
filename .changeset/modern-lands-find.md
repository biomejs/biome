---
"@biomejs/biome": patch
---

Added a new nursery rule [`noRestrictedDependencies`](https://biomejs.dev/linter/rules/no-restricted-dependencies/), which flags imports and `package.json` dependency entries that have better alternatives in e18e's module replacement data.

For example, the package `globby` is reported because there's a better alternative:

```js
import glob from "globby"
```

```json
{
  "dependencies": {
    "globby": "x.x.x"
  }
}
```
