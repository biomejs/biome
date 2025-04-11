---
"@biomejs/backend-jsonrpc": minor
"@biomejs/biome": minor
"@biomejs/cli-darwin-arm64": minor
"@biomejs/cli-darwin-x64": minor
"@biomejs/cli-linux-arm64": minor
"@biomejs/cli-linux-arm64-musl": minor
"@biomejs/cli-linux-x64": minor
"@biomejs/cli-linux-x64-musl": minor
"@biomejs/cli-win32-arm64": minor
"@biomejs/cli-win32-x64": minor
"@biomejs/js-api": minor
"@biomejs/wasm-bundler": minor
"@biomejs/wasm-nodejs": minor
"@biomejs/wasm-web": minor
---

Suppression of syntax rules

Added support for suppressing fully resolved syntax rules that run before linting or format.

Biome now allows an escape hatch if a syntax rule needs to be suppressed due to a bug or too-narrow syntax rule for a given
project.

Example:

```typescript
// biome-ignore syntax/correctness/noTypeOnlyImportAttributes: bug
import type { MyType } from "my-esm-pkg" with { "resolution-mode": "import" };
```

Biome also now fully processes suppressions before evaluating any rules.
