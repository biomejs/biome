---
"@biomejs/biome": patch
---

Added the recommended nursery rule [`noVueImportCompilerMacros`](https://biomejs.dev/linter/rules/no-vue-import-compiler-macros/), which disallows importing Vue compiler macros such as `defineProps` from `vue` because they are automatically available.
