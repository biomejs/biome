---
"@biomejs/biome": patch
---

Fixed [#7727](https://github.com/biomejs/biome/issues/7727): GritQL snippets such as `import $what from $where` now match namespace imports, including type-only imports. Explicit `import type $what from $where` patterns also match type-only named and namespace imports.
