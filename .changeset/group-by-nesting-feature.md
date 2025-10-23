---
"@biomejs/biome": minor
---

Added `groupByNesting` option to the `useSortedKeys` assist. When enabled, object keys are grouped by their value's nesting depth before sorting alphabetically.

Simple values (primitives and single-line arrays) are sorted first, followed by nested values (objects and multi-line arrays).

#### Example

```json
{
  "options": {
    "groupByNesting": true
  }
}
```

With this option, the following unsorted object:

```js
{
  "name": "Sample",
  "details": { "description": "nested" },
  "id": "123"
}
```

Will be sorted as:

```js
{
  "id": "123",
  "name": "Sample",
  "details": { "description": "nested" }
}
```
