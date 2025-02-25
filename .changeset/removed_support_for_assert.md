---
"@biomejs/biome": major
---

Removed support for `assert` syntax.

Biome now longer supports the `assert` syntax, use the new `with` syntax instead

```diff
-import {test} from "foo.json" assert { for: "for" }
-export * from "mod" assert { type: "json" }
+import {test} from "foo.json" with { for: "for" }
+export * from "mod" with { type: "json" }
```

