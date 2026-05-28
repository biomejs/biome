---
"@biomejs/biome": patch
---

Fix [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) for projects that rely on TypeScript decorator metadata.

The new `preserveDecoratorMetadata` option preserves value imports used by constructor parameter types that can be emitted as decorator metadata. This lets dependency-injection projects keep `useImportType` enabled for the rest of the file.
