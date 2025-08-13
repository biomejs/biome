---
"@biomejs/biome": minor
---

Allow customization of the sort order for different sorting actions. These actions now support a sort option:

- [`assist/source/useSortedKeys`](https://biomejs.dev/assist/actions/use-sorted-keys/) now has a `sortOrder` option
- [`assist/source/useSortedAttributes`](https://biomejs.dev/assist/actions/use-sorted-attributes/) now has a `sortOrder` option
- [`assist/source/organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) now has an `identifierOrder` option

For each of these options, the supported values are the same:

1. **`natural`**. Compares two strings using a natural ASCII order. Uppercase letters come first (e.g. `A < a < B < b`) and number are compared in a human way (e.g. `9` < `10`). This is the default value.
2. **`lexicographic`**. Strings are ordered lexicographically by their byte values. This orders Unicode code points based on their positions in the code charts. This is not necessarily the same as “alphabetical” order, which varies by language and locale.
