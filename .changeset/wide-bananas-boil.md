---
"@biomejs/biome": patch
---

Added the nursery rule [`noExcessiveClassesPerFile`](https://biomejs.dev/linter/rules/no-excessive-classes-per-file). Enforce a maximum number of classes per file.

**Invalid:**

```js
class Foo { }
class Bar { }
```
