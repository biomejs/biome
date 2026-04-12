---
"@biomejs/biome": patch
---

Fixed [#9884](https://github.com/biomejs/biome/issues/9884): [`useSortedAttributes`](https://biomejs.dev/assist/actions/use-sorted-attributes/) no longer duplicates an attribute and drops another when both an outer JSX element and a nested JSX-valued attribute have unsorted attributes.
