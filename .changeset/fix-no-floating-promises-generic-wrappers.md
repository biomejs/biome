---
"@biomejs/biome": "patch"
---

The [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) rule now detects floating promises through cross-module generic wrapper functions. Previously, patterns like `export const fn = trace(asyncFn)` — where `trace` preserves the function signature via a generic `<F>(fn: F): F` — were invisible to the rule when the wrapper was defined in a different file.
