---
"@biomejs/biome": patch
---

Fixed [#9245](https://github.com/biomejs/biome/issues/9245): the [`useSemanticElements`](https://biomejs.dev/linter/rules/use-semantic-elements/) rule no longer suggests `<output>` for `role="status"` and `role="alert"`. The `<output>` element is only a `relatedConcept` of these roles, not a direct semantic equivalent. These roles are now excluded from suggestions, aligning with the intended behavior of the upstream `prefer-tag-over-role` rule.
