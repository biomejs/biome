---
"@biomejs/biome": patch
---

Fixed [#9020](https://github.com/biomejs/biome/issues/9020): When `javascript.jsxRuntime` is set to `reactClassic`, `noUnusedImports` and `useImportType` rules now allows importing the `React` identifier from a package other than `react`. This will align the behavior with `tsc` (`--jsx=react`), which works with importing `React` from any package either.
