---
"@biomejs/biome": patch
---

Fixed grit pattern matching for different kinds of import statements.

The grit pattern `import $imports from "foo"` will match the following code:

```ts
import bar from "foo"
import { bar } from "foo"
import { bar, baz } from "foo"
```


