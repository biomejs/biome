---
"@biomejs/biome": patch
---

The CSS rules [`noDuplicateProperties`](https://biomejs.dev/linter/rules/no-duplicate-properties/), [`noDuplicateCustomProperties`](https://biomejs.dev/linter/rules/no-duplicate-custom-properties/), [`noDuplicateFontNames`](https://biomejs.dev/linter/rules/no-duplicate-font-names/), [`noDuplicateSelectorsKeyframeBlock`](https://biomejs.dev/linter/rules/no-duplicate-selectors-keyframe-block/), [`noDuplicateSelectors`](https://biomejs.dev/linter/rules/no-duplicate-selectors/), and [`noDuplicateAtImportRules`](https://biomejs.dev/linter/rules/no-duplicate-at-import-rules/) now report every duplicated item instead of only the first one found. Each duplicated item gets one diagnostic that points at the first occurrence and lists all the later occurrences.
