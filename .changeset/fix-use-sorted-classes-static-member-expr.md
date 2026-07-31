---
"@biomejs/biome": patch
---

Fixed [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/) to correctly detect unsorted classes in static member expression tagged templates (e.g. `tw.div\`...\``). Previously, these were silently skipped due to surrounding whitespace trivia not being stripped from the tag name.
