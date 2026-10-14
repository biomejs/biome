---
"@biomejs/biome": patch
---

Improved the diagnostics of the GraphQL rules [`noDuplicateArgumentNames`](https://biomejs.dev/linter/rules/no-duplicate-argument-names/), [`noDuplicateEnumValueNames`](https://biomejs.dev/linter/rules/no-duplicate-enum-value-names/), [`noDuplicateFieldDefinitionNames`](https://biomejs.dev/linter/rules/no-duplicate-field-definition-names/), [`noDuplicateGraphqlOperationName`](https://biomejs.dev/linter/rules/no-duplicate-graphql-operation-name/), [`noDuplicateInputFieldNames`](https://biomejs.dev/linter/rules/no-duplicate-input-field-names/) and [`noDuplicateVariableNames`](https://biomejs.dev/linter/rules/no-duplicate-variable-names/). Each diagnostic now names the duplicated identifier, highlights its first occurrence and points to every repeated occurrence, instead of highlighting the whole parent node or only the repeated occurrences. Names that are duplicated more than once in the same list are reported separately.
