---
"@biomejs/biome": patch
---

Added a new nursery rule [`noVueVOnNumberValues`](https://biomejs.dev/linter/rules/no-vue-v-on-number-values/), that disallows deprecated number modifiers on Vue `v-on` directives.

For example, the following snippet triggers the rule:

```vue
<input @keyup.13="submit" />
```
