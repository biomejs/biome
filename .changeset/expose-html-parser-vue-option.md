---
"@biomejs/biome": minor
---

Exposed the `html.parser.vue` configuration option. This option enables parsing of Vue syntax (directives like `v-if`, `v-bind`, etc.) in `.html` files. Most Vue users don't need to enable this option since Vue files typically use the `.vue` extension, but it can be useful for projects that embed Vue syntax in regular HTML files.
