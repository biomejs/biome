---
"@biomejs/biome": patch
---

Added the nursery rule [`noUnsafeSqliteInterpolation`](https://biomejs.dev/linter/rules/no-unsafe-sqlite-interpolation/), which reports SQL strings that insert values with `${}` or use a tagged template when passed to the `exec()` and `prepare()` methods of a `node:sqlite` database.

```js
import { DatabaseSync } from "node:sqlite";

const database = new DatabaseSync(":memory:");
database.exec(`SELECT * FROM users WHERE id = ${id}`);
```
