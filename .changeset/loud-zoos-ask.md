---
"@biomejs/biome": patch
---

Fixed [#11962](https://github.com/biomejs/biome/issues/11962): [`noUnknownTypeSelector`](https://biomejs.dev/linter/rules/no-unknown-type-selector/) no longer reports view transition names inside view transition pseudo-elements, such as `page` in `::view-transition-group(page)`.
