---
"@biomejs/biome": patch
---

Fixed [#6767](https://github.com/biomejs/biome/issues/6767): `useSortedClasses` now correctly removes leading and trailing whitespace in className.

Previously, trailing spaces in className were not fully removed.

```jsx
// Think we have this code:
<div className="text-sm font-bold            " /> 

// Before: applied fix, but trailing space was preserved
<div className="font-bold text-sm " />

// After: applied fix, trailing spaces removed
<div className="font-bold text-sm" />
```
