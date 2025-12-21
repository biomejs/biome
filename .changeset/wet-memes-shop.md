---
"@biomejs/biome": patch
---

Fixed [#8331](https://github.com/biomejs/biome/issues/8331) false positive for `noUnusedVariables` rule. The rule now does not mark top-level interfaces/namespaces declarations as unused when the file has no top-level imports or exports (script file), such as `interface Foo {}`.
