---
"@biomejs/biome": patch
---

Fixed [#9020](https://github.com/biomejs/biome/issues/9020): When `javascript.jsxRuntime` is set to `reactClassic`, `noUnusedImports` and `useImportType` rules now allow importing the `React` identifier from a package other than `react`. This aligns the behavior with `tsc` (`--jsx=react`), which also allows importing `React` from any package.
