---
"@biomejs/biome": minor
---

Add the `preserveDecoratorMetadata` option to `useImportType`.

When enabled, the rule preserves value imports used by constructor parameter types that can be emitted as TypeScript decorator metadata. This helps projects that rely on decorator metadata for dependency injection keep `useImportType` enabled for the rest of the file.
