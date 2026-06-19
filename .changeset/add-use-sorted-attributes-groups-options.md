---
"@biomejs/biome": minor
---

Added `groups` and `sortScope` options to [`useSortedAttributes`](https://biomejs.dev/assist/actions/use-sorted-attributes/).

- `sortScope: "group"` enables group-aware sorting: props are partitioned into groups, sorted within each group using `sortOrder`, then concatenated in group order. When `sortScope` is `"global"` (the default), all existing behavior is preserved.
- `groups` accepts an ordered list of predefined group tokens (`:CALLBACK:`, `:IMPLICIT:`, `:RESERVED:`, `:DOM_RESERVED:`, `:REST:`) that control where special prop categories appear relative to each other. Only active when `sortScope` is `"group"`. `:REST:` is a catch-all for props matching no other group and, unlike the implicit fallback used when it's omitted, is sorted like a regular group. The default group ordering is `[":IMPLICIT:", ":RESERVED:", ":DOM_RESERVED:", ":REST:", ":CALLBACK:"]`.
