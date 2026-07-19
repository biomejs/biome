---
"@biomejs/biome": patch
---

[`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) now recognizes `readonly` index signatures when comparing return annotations, so a wider annotation like `Readonly<Record<string, string>>` is reported when the function returns a narrower value.

```ts
export function createRecord(): Readonly<Record<string, string>> {
  return { key: "value" } as const;
}
```
