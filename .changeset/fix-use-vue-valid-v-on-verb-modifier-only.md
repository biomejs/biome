---
"@biomejs/biome": patch
---

Fixed [#10772](https://github.com/biomejs/biome/issues/10772): [`useVueValidVOn`](https://biomejs.dev/linter/rules/use-vue-valid-v-on/) no longer reports a missing handler when the v-on directive uses a verb modifier (`.stop` or `.prevent`) without an expression. Modifier-only forms such as `<div @click.stop></div>` and `<form @submit.prevent></form>` carry an intrinsic side effect (`event.stopPropagation()` / `event.preventDefault()`) and are valid Vue syntax. This matches `eslint-plugin-vue`'s `valid-v-on`, which exempts the same `VERB_MODIFIERS` set.

Additionally, fixed a related false positive where `<div v-on="$listeners"></div>` and other arg-less `v-on` with an object value were reported as missing an event name. An arg-less `v-on` with a value is the Vue object syntax (parallel to `<div v-bind="object"></div>`).
