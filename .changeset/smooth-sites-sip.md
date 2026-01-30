---
"@biomejs/biome": patch
---

Improved the summary provided by `biome migrate eslint` to be clearer on why rules were not migrated. Biome now specifies a reason when a rule is not migrated, such as being incompatible with the formatter or not implemented yet. This helps users make more informed decisions when migrating their ESLint configurations to Biome.
