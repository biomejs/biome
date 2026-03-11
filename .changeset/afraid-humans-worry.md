---
"@biomejs/biome": minor
---

Added the assist [`useSortedTypeFields`](https://biomejs.dev/assist/actions/use-sorted-type-fields/).

Biome now sorts the fields of GraphQL object types, interface types, and input object types alphabetically, e.g. `name, age, id` becomes `age, id, name`.
