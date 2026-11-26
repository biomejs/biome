---
"@biomejs/biome": patch
---

Added the nursery rule [`noConvexWrongRuntimeImports`](https://biomejs.dev/linter/rules/no-convex-wrong-runtime-imports/), which reports files in a Convex `convex` folder that run in the Convex runtime but import a file with the `"use node"` directive.

```ts
// convex/payments.ts, where convex/stripe.ts starts with "use node"
import { formatAmount } from "./stripe";
```
