---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueVOnExact`](https://biomejs.dev/linter/rules/use-vue-v-on-exact/), which requires the `.exact` modifier on a `v-on` listener when another listener for the same event on the element needs more modifier keys, such as `.ctrl`. Without `.exact`, both listeners run when those keys are held down. The rule accepts the following code:

```vue
<template>
  <button @click.exact="select" @click.ctrl="open"></button>
</template>
```
