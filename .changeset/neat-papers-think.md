---
"@biomejs/biome": patch
---

Fix [#7583](https://github.com/biomejs/biome/issues/7583).
[`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) now
sorts named specifiers inside bare exports and merges bare exports.

```diff
- export { b, a };
- export { c };
+ export { a, b, c };
```

Also, `organizeImports` now correctly adds a blank line between an import chunk
and an export chunk.

```diff
  import { A } from "package";
+
  export { A };
```
