---
"@biomejs/biome": patch
---

Fixed Vue lint rules such as [`useVueVModel`](https://biomejs.dev/linter/rules/use-vue-v-model/) and [`useVueVOnExact`](https://biomejs.dev/linter/rules/use-vue-v-on-exact/) treating MathML elements like `<math>` and `<mi>` as components. Biome now recognizes the MathML Core elements as native elements, and in Vue, Svelte, and Astro files it treats known hyphenated elements such as `<annotation-xml>` and the SVG `<font-face>` as native elements instead of components.
