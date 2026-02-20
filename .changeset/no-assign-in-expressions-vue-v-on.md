---
"@biomejs/biome": patch
---

Fixed [#9161](https://github.com/biomejs/biome/issues/9161): The `noAssignInExpressions` rule no longer flags assignments in Vue v-on directives (e.g., `@click="counter += 1"`). Assignments in event handlers are idiomatic Vue patterns and are now skipped by the rule.
