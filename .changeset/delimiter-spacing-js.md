---
"@biomejs/biome": minor
---

Implemented `delimiterSpacing` for JavaScript. When enabled, Biome inserts spaces inside parentheses (e.g., `foo( a, b )`), square brackets (e.g., `[ a, b ]`), template literal interpolations (e.g., `${ expr }`), and the logical NOT operator (e.g., `! x`, but in chains only after the last one: `!! x`). Only applies when the content fits on a single line. Empty delimiters and the space before the opening delimiter are not affected.

```diff
- if (condition) {}
+ if ( condition ) {}
```

```diff
- `Hello ${name}!`
+ `Hello ${ name }!`
```
