---
"@biomejs/biome": minor
---

Added `groups`, `ignoreCase`, `sortScope`, and `multiline` options to [`useSortedAttributes`](https://biomejs.dev/assist/actions/use-sorted-attributes/).

- `groups` accepts an ordered list of predefined group tokens (`:CALLBACK:`, `:IMPLICIT:`, `:RESERVED:`, `:DOM_RESERVED:`, `:REST:`) that control where special prop categories appear relative to each other. `:REST:` is a catch-all for props matching no other group and, unlike the implicit fallback used when it's omitted, is sorted like a regular group.
- `ignoreCase` makes prop name comparisons case-insensitive.
- `sortScope: "group"` enables group-aware sorting: props are partitioned into groups, sorted within each group, then concatenated in group order. Props not matching any named group (and not covered by a `:REST:` group) preserve their original relative order.
- `multiline` controls how props with multiline values are ordered relative to single-line props. Accepted values are `"group"` (default, no special treatment), `"groupFirst"` and `"groupLast"` (multiline props first or last within each group), and `"first"` and `"last"` (all multiline props before or after all single-line props across groups).

When `sortScope` is `"global"` (the default), all existing behavior is preserved.
