---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueContentOutsideBlock`](https://biomejs.dev/linter/rules/no-vue-content-outside-block/), which reports text and code placed outside of the top-level blocks of a Vue component. Vue drops this content, so it never renders or runs.

For example, `console.log(1);` below is reported:

```vue
<template>
  <div>Hello</div>
</template>

console.log(1);

<script>
export default {};
</script>
```
