---
"@biomejs/biome": patch
---

Fixed [#12503](https://github.com/biomejs/biome/issues/12503): [`noUnknownProperty`](https://biomejs.dev/linter/rules/no-unknown-property/) now checks SCSS nested properties by the name Sass emits for them. For example, `radius` in `border: { radius: 4px; }` is checked as `border-radius` and is no longer reported, while `color` in `font: { color: red; }` is checked as `font-color` and is now reported.
