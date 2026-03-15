---
"@biomejs/biome": minor
---

Added the `delimiterSpacing` formatter option. This option inserts spaces inside delimiters. Affects parentheses `()`, square brackets `[]`, TypeScript angle brackets `<>`, and JSX curly braces `{}`. It can be configured globally via `formatter.delimiterSpacing` or per-language via `javascript.formatter.delimiterSpacing`, `json.formatter.delimiterSpacing`, and `css.formatter.delimiterSpacing`. Defaults to false.

**JavaScript:**

```diff
- if (condition) {}
+ if ( condition ) {}
```

**JSON:**

```diff
- [1, 2, 3]
+ [ 1, 2, 3 ]
```

**CSS:**

```diff
- rgba(0, 0, 0, 1)
+ rgba( 0, 0, 0, 1 )
```
