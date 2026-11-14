---
"@biomejs/biome": patch
---

Fixed [#12057](https://github.com/biomejs/biome/issues/12057): The safe fix of the [`useSortedKeys`](https://biomejs.dev/assist/actions/use-sorted-keys/) assist no longer moves a property onto the same line as a `//` comment, which commented the property out.
