---
"@biomejs/biome": minor
---

Added `groups`, `ignoreCase`, and `sortScope` options to [`useSortedAttributes`](https://biomejs.dev/assist/actions/use-sorted-attributes/).

- `groups` accepts an ordered list of predefined group tokens (`:CALLBACK:`, `:IMPLICIT:`, `:MULTILINE:`, `:RESERVED:`, `:DOM_RESERVED:`) that control where special prop categories appear relative to each other.
- `ignoreCase` makes prop name comparisons case-insensitive.
- `sortScope: "group"` enables group-aware sorting: props are partitioned into groups, sorted within each group, then concatenated in group order. Props not matching any named group preserve their original relative order.

When `sortScope` is `"global"` (the default), all existing behavior is preserved.
