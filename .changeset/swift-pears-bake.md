---
"@biomejs/biome": patch
---

Fixed [#12413](https://github.com/biomejs/biome/issues/12413): [`useSortedProperties`](https://biomejs.dev/assist/actions/use-sorted-properties/) could change the cascade of a block by moving declarations across nested rules or at-rules. Properties are now sorted only within each run of consecutive declarations, and the statements separating those runs keep their position.
