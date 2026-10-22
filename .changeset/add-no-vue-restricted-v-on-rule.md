---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueRestrictedVOn`](https://biomejs.dev/linter/rules/no-vue-restricted-v-on/), which reports `v-on` event listeners that are disallowed through the `listeners` option. An entry can match an event name, a set of modifiers, and an element. For example, with `{ "listeners": [{ "event": "click", "element": "MyButton" }] }`, `<MyButton @click="onClick" />` is now reported.
