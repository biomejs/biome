---
"@biomejs/biome": patch
---

Implements [#7339](https://github.com/biomejs/biome/discussions/7339). GritQL patterns can now use native Biome AST nodes using their `PascalCase` names, in addition to the existing TreeSitter-compatible `snake_case` names.

```grit
engine biome(1.0)
language js(typescript,jsx)

or {
  // TreeSitter-compatible pattern
  if_statement(),

  // Native Biome AST node pattern
  JsIfStatement()
} as $stmt where {
  register_diagnostic(
    span=$stmt,
    message="Found an if statement"
  )
}
```
