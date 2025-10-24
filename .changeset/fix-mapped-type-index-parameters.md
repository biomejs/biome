---
"@biomejs/biome": patch
---

Fixed [#7532](https://github.com/biomejs/biome/issues/7532): The `useNamingConvention` rule now correctly treats mapped type index parameters (e.g., `[key in SomeList]`) as index parameters requiring camelCase, instead of incorrectly treating them as type parameters requiring PascalCase.

**Before**:
```typescript
type IndexedByEnum = { [key in SomeList]: string };
//                      ^^^ Error: should be PascalCase
```

**After**:
```typescript
type IndexedByEnum = { [key in SomeList]: string };
//                      ^^^ Valid: camelCase is correct for index parameters
```

This aligns mapped type index parameters with index signatures (`[s: string]`), both of which iterate over keys/properties and should use camelCase.
