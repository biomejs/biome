---
"@biomejs/biome": minor
---

Added support for Grit [`range()` patterns](https://docs.grit.io/language/patterns#range-patterns), addressing [#8454](https://github.com/biomejs/biome/issues/8454). Plugins can filter syntax nodes by optional `start_line`, `start_column`, `end_line`, and `end_column` bounds.

For example, this pattern matches function calls contained entirely on line 2.

```grit
range(start_line=2, end_line=2) as $call where {
    $call <: call_expression()
}
```
