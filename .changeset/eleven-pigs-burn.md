---
"@biomejs/biome": patch
---

Fixed [#11672](https://github.com/biomejs/biome/issues/11672) and [#11671](https://github.com/biomejs/biome/issues/11671) by disabling the experimental capitalized-call and effect-dependency checks in [`useReactCompiler`](https://biomejs.dev/linter/rules/use-react-compiler/), matching their exclusion from upstream's recommended lint preset. Valid calls such as `Intl.NumberFormat()` and captures of variables declared inside effects no longer produce these diagnostics.
