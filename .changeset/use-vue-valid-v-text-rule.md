---
"@biomejs/biome": patch
---

Added the `useVueValidVText` lint rule to enforce valid `v-text` directives. The rule reports when `v-text` has an argument, has modifiers, or is missing a value.

Invalid:

```vue
<div v-text /> <!-- missing value -->
<div v-text:aaa="foo" /> <!-- has argument -->
<div v-text.bbb="foo" /> <!-- has modifier -->
```
