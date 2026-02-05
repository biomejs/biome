---
"@biomejs/biome": patch
---

Fixed [#4888](https://github.com/biomejs/biome/issues/4888).
[noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/) now adds `export {}` when removing the last import in a TypeScript file to prevent it from becoming an ambient module. This does not apply to embedded scripts in Vue, Svelte, or Astro files, which are already in a module context.
