---
"@biomejs/biome": patch
---

Fixed [`useSortedProperties`](https://biomejs.dev/assist/actions/use-sorted-properties/) to withhold sorting fixes that would change the effective values of overlapping declarations, physical/logical border or corner properties, or declarations followed by `all`.
Updated [`noShorthandPropertyOverrides`](https://biomejs.dev/linter/rules/no-shorthand-property-overrides/) to recognize logical border, corner shape, `position-try`, `text-box`, and `transition-behavior` overrides.
