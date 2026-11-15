---
"@biomejs/biome": patch
---

[`noSvelteUnnecessaryStateWrap`](https://biomejs.dev/linter/rules/no-svelte-unnecessary-state-wrap/) now reports reactive classes imported from `svelte/reactivity` under another name or through a namespace import, as in `$state(new reactivity.SvelteMap())`. With `allowReassign`, it also withholds the fix when the template reassigns the variable.

[`noSvelteBindValueOnCheckableInputs`](https://biomejs.dev/linter/rules/no-svelte-bind-value-on-checkable-inputs/) now reports inputs whose `type` is an expression with a constant value, as in `<input type={kind} bind:value={checked} />` with `const kind = "checkbox"`.
