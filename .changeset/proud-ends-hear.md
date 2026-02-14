---
"@biomejs/biome": minor
---

Added support for the top-level suppression comment `biome-ignore-all format: <explanation>`.

When the comment `biome-ignore-all format: <explanation>` is placed at the beginning of the document, Biome won't format the code.

The feature works for all supported languages. In the following JavaScript snippet, the code isn't formatted and will stay as is.

```js
// biome-ignore-all format: generated

const a = [  ]


const a = [  ]


const a = [  ]
```
