---
"@biomejs/biome": patch
---

Added support for SCSS variable declarations in SCSS files.

- Simple variables: `$primary-color: #ff0000;`
- Namespaced variables: `namespace.$theme-color: blue;`
- Variables with modifiers: `$global-var: red !default !global;`

```scss
// Simple variables
$primary-color: #ff0000;

// Namespaced variables
namespace.$theme-color: blue;

// Variables with modifiers
$global-var: red !default !global;
```
