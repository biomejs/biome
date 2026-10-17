---
"@biomejs/biome": patch
---

Fixed [#12417](https://github.com/biomejs/biome/issues/12417): [`useSortedProperties`](https://biomejs.dev/assist/actions/use-sorted-properties/) could produce invalid CSS when sorting properties without a final semicolon. Comments attached to reordered properties are preserved.
