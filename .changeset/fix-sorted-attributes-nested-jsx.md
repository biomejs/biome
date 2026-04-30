---
"@biomejs/biome": patch
---

Fixed [#9884](https://github.com/biomejs/biome/issues/9884): The [`useSortedAttributes`](https://biomejs.dev/assist/actions/use-sorted-attributes/) auto-fix no longer corrupts the source when both an outer JSX element and a nested JSX-valued attribute have unsorted attributes in the same fix pass. The fix now replaces the entire attribute list as a single mutation, so it does not collide at the slot level with mutations propagated up from nested elements. Multiple unsorted groups separated by spread or shorthand attributes within the same JSX element are now reported as a single diagnostic.
