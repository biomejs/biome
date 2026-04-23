---
"@biomejs/biome": minor
---

Implemented `delimiterSpacing` for JSX. When enabled, Biome inserts spaces inside JSX expression braces (e.g., `attr={ value }`) and spread attributes (e.g., `{ ...props }`). Only applies when the content fits on a single line. Empty delimiters are not affected.

```diff
- <Foo bar={value} />
+ <Foo bar={ value } />
```
