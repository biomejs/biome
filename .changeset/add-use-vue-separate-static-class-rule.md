---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueSeparateStaticClass`](https://biomejs.dev/linter/rules/use-vue-separate-static-class/), which reports class names in a Vue `:class` binding that always apply, such as `header` in `<div :class="['header', theme]">`. These classes belong in the plain `class` attribute.
