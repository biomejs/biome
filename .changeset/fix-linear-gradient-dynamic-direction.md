---
"@biomejs/biome": patch
---

Fixed [`noInvalidDirectionInLinearGradient`](https://biomejs.dev/linter/rules/no-invalid-direction-in-linear-gradient/) reporting directions it can't know, such as `linear-gradient(var(--top), red, blue)` or `linear-gradient($dir-top, red, blue)` in SCSS. A direction that contains a function call, a Sass variable, or interpolation is no longer checked.
