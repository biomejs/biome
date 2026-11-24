---
"@biomejs/biome": patch
---

Fixed [#13384](https://github.com/biomejs/biome/issues/13384): with full Svelte support enabled, module scripts no longer see instance-script variables, nested DOM scripts have their own scope, and template reads no longer hide unused same-named variables in scripts or event handlers. Biome distinguishes type and value declarations, including interfaces merged with other interfaces or classes and component generics that shadow module types. Qualified type names such as `API.Result` can refer to a `declare module API` in the module script. It reports undeclared render callees and recognizes top-level snippets in module scripts, even with Unicode identifier escapes, when they do not capture instance state.
