---
"@biomejs/biome": patch
---

Fixed [`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) false positive on returns that use a widening type assertion: `"a" as string` is no longer reported as misleading. The rule now also reports a literal-pinning assertion such as `false as false`, matching the existing `as const` behavior.

```ts
// No longer flagged (returns are `string`):
function getValue(b: boolean): string {
  if (b) return "a" as string;
  return "b" as string;
}

// Now also reported, like `as const` (returns `false`):
function isReady(): boolean {
  return false as false;
}
```
