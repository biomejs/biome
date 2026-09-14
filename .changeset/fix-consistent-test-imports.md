---
"@biomejs/biome": patch
---

Fixed [#11453](https://github.com/biomejs/biome/issues/11453): [`useConsistentTestIt`](https://biomejs.dev/linter/rules/use-consistent-test-it/) now updates imports alongside calls, preserving the original export through an alias. The rule ignores locally declared functions and withholds fixes when the preferred name would conflict with another binding or global reference.
