---
"@biomejs/biome": patch
---

Fixed [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) reporting names that a module re-exports from a CommonJS module or from a file that isn't JavaScript, such as `export { foo } from "./legacy.cjs"`. The rule now also reports default imports from a module that only re-exports another module with `export * from`, because `export * from` doesn't forward the default export.
