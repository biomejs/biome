---
"@biomejs/biome": patch
---

Fixed [#12410](https://github.com/biomejs/biome/issues/12410): the [`useSortedProperties`](https://biomejs.dev/assist/actions/use-sorted-properties/) assist now preserves the relative order of Tailwind CSS wildcard resets such as `--font-*: initial` and other custom properties.
