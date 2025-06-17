---
"@biomejs/biome": minor
---

Suppression of syntax rules

Added support for suppressing syntax rules. Syntax rules are particular rules meant **to complement the parser**, hence they can't be configured.

Biome now allows to suppress those rules. This can, for example, be useful in case the rule is affected by a bug. However, this is more an escape hatch, so if a syntax rule requires a suppression, please file an issue.

Example:

```typescript
// biome-ignore syntax/correctness/noTypeOnlyImportAttributes: bug
import type { MyType } from "my-esm-pkg" with { "resolution-mode": "import" };
```

Biome now requires all `biome-ignore-start` suppressions to have an equivalent `biome-ignore-end` comment.