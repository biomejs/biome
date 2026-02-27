---
"@biomejs/biome": patch
---

Fixed [#9143](https://github.com/biomejs/biome/issues/9143): The [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) rule no longer reports false positives for several common patterns:

- `node:fs`, `node:path`, `node:url`, and other Node.js built-in modules with the `node:` prefix are now accepted.
- Packages that declare their TypeScript entry point via `"typings"` (instead of `"types"`) in `package.json` now resolve correctly.
- Named imports from aliased re-export chains (e.g. `export { x as y } from "..."`) are now resolved correctly through the alias.
- Namespace re-exports (e.g. `export * as Ns from "..."`) are now recognized as own exports of the barrel module.
