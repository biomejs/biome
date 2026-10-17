---
"@biomejs/biome": patch
---

Fixed [`useVueDefineMacrosOrder`](https://biomejs.dev/linter/rules/use-vue-define-macros-order/) missing misplaced bare macro calls and reporting the wrong macro for assigned calls. Its unsafe fix now orders all macro statements in one application and preserves their comments and statement boundaries.
