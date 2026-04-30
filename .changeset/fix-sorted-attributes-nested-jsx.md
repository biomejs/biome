---
"@biomejs/biome": patch
---

Fixed [#9884](https://github.com/biomejs/biome/issues/9884): The [`useSortedAttributes`](https://biomejs.dev/assist/actions/use-sorted-attributes/) auto-fix no longer corrupts source code when both an outer JSX element and a nested JSX-valued attribute have unsorted attributes in the same pass. Multiple unsorted groups separated by spread or shorthand attributes within the same JSX element are now reported as a single diagnostic.
