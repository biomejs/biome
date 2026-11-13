---
"@biomejs/biome": patch
---

The Vue emit rules now read events declared with a function type, as in `defineEmits<(e: "change", value: string) => void>()`, and event names written as template literals without substitutions, as in ``emit(`change`)``. [`noVueUndeclaredEmits`](https://biomejs.dev/linter/rules/no-vue-undeclared-emits/), [`noVueUnusedEmitDeclarations`](https://biomejs.dev/linter/rules/no-vue-unused-emit-declarations/), [`useVueSingleEventPayload`](https://biomejs.dev/linter/rules/use-vue-single-event-payload/) and [`useVueEmitValidator`](https://biomejs.dev/linter/rules/use-vue-emit-validator/) previously skipped events declared in these forms.

[`noVueShadowNativeEvents`](https://biomejs.dev/linter/rules/no-vue-shadow-native-events/) now checks `this.$emit()` only where `this` is the component, and no longer reports `$emit()` called through a variable that holds `this`, as in `const vm = this; vm.$emit("click")`.
