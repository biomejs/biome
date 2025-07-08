---
"@biomejs/biome": minor
---

Allow customization for the sort order for different sorting rules. These rules are supported with following options:

- [assist/source/useSortedKeys](https://biomejs.dev/assist/actions/use-sorted-keys/) (sortOrder)
- [assist/source/useSortedAttributes](https://biomejs.dev/assist/actions/use-sorted-attributes/) (sortOrder)
- [assist/source/organizeImports](https://biomejs.dev/assist/actions/organize-imports/) (identifierOrder)

Following options are supported for ordering:

*1. Natural (default)*

Compare two strings using a natural ASCII order. Uppercase letters come first (e.g. `A` < `a` < `B` < `b`) and number are compared in a human way (e.g. `9` < `10`).

*2. Lexicographic*

Strings are ordered lexicographically by their byte values. This orders Unicode code points based on their positions in the code charts. This is not necessarily the same as “alphabetical” order, which varies by language and locale.
