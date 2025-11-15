---
"@biomejs/biome": minor
---

Added `groupByNesting` option to the `useSortedKeys` assist. When enabled, object keys are grouped by their value's nesting depth before sorting alphabetically.

Simple values (primitives, single-line arrays, and single-line objects) are sorted first, followed by nested values (multi-line arrays and multi-line objects).

#### Example

To enable this option, configure it in your `biome.json`:

```json
{
  "linter": {
    "rules": {
      "source": {
        "useSortedKeys": {
          "options": {
            "groupByNesting": true
          }
        }
      }
    }
  }
}
```

With this option, the following unsorted object:

```js
const object = {
  "name": "Sample",
  "details": {
    "description": "nested"
  },
  "id": 123
}
```

Will be sorted as:

```js
const object ={
  "id": 123,
  "name": "Sample",
  "details": {
    "description": "nested"
  }
}
```
