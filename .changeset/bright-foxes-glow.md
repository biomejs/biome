---
"@biomejs/biome": minor
---

Added JSON as a target language for GritQL pattern matching. You can now write Grit plugins for JSON files.

This enables users to write GritQL patterns that match against JSON files, useful for:
- Searching and transforming JSON configuration files
- Enforcing patterns in `package.json` and other JSON configs
- Writing custom lint rules for JSON using GritQL

**Example patterns:**

Match all key-value pairs:
```grit
language json

pair(key = $k, value = $v)
```

Match objects with specific structure:
```grit
language json

JsonObjectValue()
```

Supports both native Biome AST names (`JsonMember`, `JsonObjectValue`) and TreeSitter-compatible names (`pair`, `object`, `array`) for compatibility with existing Grit patterns.

For more details, see the [GritQL documentation](https://biomejs.dev/reference/gritql/).
