---
"@biomejs/biome": patch
---

Fixed [#11644](https://github.com/biomejs/biome/issues/11644): [`useHeadingContent`](https://biomejs.dev/linter/rules/use-heading-content/) no longer reports headings that render their text with a directive: `set:html` and `set:text` in Astro files, `v-html` and `v-text` in Vue files.

```astro
<h1 set:html={heading} />
<h2 set:text={heading}></h2>
```

```vue
<template>
  <h1 v-html="heading"></h1>
  <h2 v-text="heading"></h2>
</template>
```
