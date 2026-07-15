---
"@biomejs/biome": patch
---

Fixed false positives in [`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) when generic-constraint, normalization, substitution, or structural return-type comparison cannot complete. The rule now suppresses diagnostics rather than suggesting a return type derived from partial information. For example, this unresolved return type is no longer reported:

```ts
function unresolvedReturnType(): MissingType {
	return "value" as const;
}
```
