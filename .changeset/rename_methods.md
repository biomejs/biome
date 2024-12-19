---
biome_rowan: major
---

# Rename methods

Some methods were renamed, as their names weren't explicit enough:

- `AstNode::text` to `AstroNode::to_trimmed_string`
- `SyntaxNode::text` to `SyntaxNode::text_with_trivia`
- `SyntaxNode::text_range` to `SyntaxNode::text_range_with_trivia`
