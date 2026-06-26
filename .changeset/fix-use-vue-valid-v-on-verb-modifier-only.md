---
"@biomejs/biome": patch
---

[`useVueValidVOn`](https://biomejs.dev/linter/rules/use-vue-valid-v-on/) no longer reports a missing handler when the v-on directive uses a verb modifier (`.stop` or `.prevent`) without an expression. Modifier-only forms such as `<div @click.stop></div>` and `<form @submit.prevent></form>` carry an intrinsic side effect (`event.stopPropagation()` / `event.preventDefault()`) and are valid Vue syntax. This matches `eslint-plugin-vue`'s `valid-v-on`, which exempts the same `VERB_MODIFIERS` set.
