---
"@biomejs/biome": patch
---

Refactor: remove one level of indirection for CSS declarations with semicolon
Previously, accessing a declaration from a list required an extra step:

```rust
item
.as_any_css_declaration_with_semicolon()
.as_css_declaration_with_semicolon()
```

Now, it can be done directly with:

```rust
item.as_css_declaration_with_semicolon()
```
