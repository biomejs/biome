---
"@biomejs/biome": patch
---

Fixed [#2862](https://github.com/biomejs/biome/issues/2862): [`noInteractiveElementToNoninteractiveRole`](https://biomejs.dev/linter/rules/no-interactive-element-to-noninteractive-role/) no longer reports custom elements (a tag name containing a dash, e.g. `<my-button role="img" />`). Per the [W3C HTML-ARIA specification](https://www.w3.org/TR/html-aria/#el-autonomous-custom-element), a custom element may be given any role or none.
