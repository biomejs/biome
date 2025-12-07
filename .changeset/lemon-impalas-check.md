---
"@biomejs/biome": patch
---

Fixed [#8190](https://github.com/biomejs/biome/issues/8190): The HTML parser will now parse Vue event handlers that contain `:` correctly, e.g. `@update:modelValue="onUpdate"`.
