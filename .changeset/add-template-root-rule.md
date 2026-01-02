---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueValidTemplateRoot`](https://biomejs.dev/linter/rules/use-vue-valid-template-root/).

This rule validates only root-level `<template>` elements in Vue single-file components. If the `<template>` has a `src` attribute, it must be empty. Otherwise, it must contain content.

Invalid examples:

```vue
<template src="./foo.html">content</template>
```

```vue
<template></template>
```

Valid examples:

```vue
<template>content</template>
```

```vue
<template src="./foo.html"></template>
```
