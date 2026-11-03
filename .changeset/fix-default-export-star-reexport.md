---
"@biomejs/biome": patch
---

Fixed [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/): it now reports default imports from modules that only forward exports with `export * from "..."`. `export *` never re-exports a default export, so `import foo from "./barrel.js"` fails when `./barrel.js` contains only `export * from "./foo.js"`.
