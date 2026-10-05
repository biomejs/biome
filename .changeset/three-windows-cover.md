---
"@biomejs/biome": patch
---

Vue rules that check props, such as [`noVueReservedProps`](https://biomejs.dev/linter/rules/no-vue-reserved-props/), now detect props declared inside `withDefaults()`, for example `withDefaults(defineProps<{ key?: string }>(), {})`.
