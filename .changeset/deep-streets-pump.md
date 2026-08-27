---
"@biomejs/biome": patch
---

Fixed [`noVueRefAsOperand`](https://biomejs.dev/linter/rules/no-vue-ref-as-operand/) to track Vue refs through declaration aliases and `toRefs()` properties, and to recognize `useTemplateRef()` results. The rule no longer reports false positives such as plain ref transfers, plain `toRefs()` property access, `defineModel()` modifiers, or the supported `.effect` member as operands.

The refactor enabling these fixes also improves the performance of the rule.
