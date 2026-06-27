---
"@biomejs/biome": patch
---

Fixed [#10772](https://github.com/biomejs/biome/issues/10772): [`useVueValidVOn`](https://biomejs.dev/linter/rules/use-vue-valid-v-on/) no longer reports a missing handler for v-on directives using a verb modifier (`.stop` / `.prevent`) without an expression, e.g. `<div @click.stop></div>`. The rule also accepts the arg-less object syntax `<div v-on="$listeners"></div>` instead of reporting a missing event name.
