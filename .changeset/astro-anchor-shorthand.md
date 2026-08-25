---
@biomejs/biome: patch
---

Fixed [`useValidAnchor`](https://biomejs.dev/linter/rules/use-valid-anchor/) so Astro JSX shorthand attributes like `<a {href}>` inside expressions are treated as a valid `href`.
