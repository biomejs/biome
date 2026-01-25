---
"@biomejs/biome": patch
---

Improved [`useVueValidVOn`](https://biomejs.dev/linter/rules/use-vue-valid-v-on/) to be more closely aligned with the source rule. It will now properly allow modifiers for all possible keyboard events. It should have better performance when there are no violations of the rule as well.

Now treated valid:
```vue
<div @keydown.arrow-down="handler"></div>
<div @keydown.a="handler"></div>
<div @keydown.b="handler"></div>
<div @keydown.27="foo"></div>
```
