---
"@biomejs/biome": patch
---

Added the new nursery rule [`noConvexProcessEnv`](https://biomejs.dev/linter/rules/no-convex-process-env/) to the `convex` domain. It reports uses of `process.env` in the `convex` folder of a package, and recommends reading environment variables from the typed `env` object exported by `_generated/server` instead. Its unsafe fix replaces `process.env` with `env` and adds the import.

```ts
// convex/giphy.ts
const apiKey = process.env.GIPHY_KEY;
```
