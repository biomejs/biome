---
"@biomejs/biome": patch
---

Fixed [#8812](https://github.com/biomejs/biome/issues/8812): `noArrayIndexKey` rule now correctly detects array index usage in any position within template strings.

Previously, the rule only detected array indices when they appeared in the last template expression. Now it properly flags violations regardless of position:

```jsx
// Now correctly detected as violations
<div key={`${index}-${item}`}>{item}</div>
<div key={`${index}-${item.title}`}>{item.title}</div>
```
