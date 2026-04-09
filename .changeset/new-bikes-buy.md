---
"@biomejs/biome": minor
---

Added the new assist action [`useSortedSelectionSet`](https://biomejs.dev/assist/actions/use-sorted-selection-set/), which sorts GraphQL selection sets alphabetically, e.g. `name, age, id` becomes `age, id, name`.

**Invalid**:

```graphql
query {
  name
  age
  id
}
```
