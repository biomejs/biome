---
"@biomejs/biome": patch
---

Fixed Vue single-file components failing to parse when they contain a custom block such as `<i18n>` or `<docs>`, or a `<template>` written in another language. Their content is no longer read as HTML, so a block may hold whatever its own tooling expects:

```vue
<docs>
This block is prose, and it may mention a `<my-component>` without closing it.
</docs>

<template lang="pug">
  .test
    #foo
</template>
```

Previously both blocks produced a parse error and the whole file was left unformatted. Biome now prints their content unchanged while still formatting the opening tag.
