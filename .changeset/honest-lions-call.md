---
"@biomejs/biome": patch
---

Fixed [#5213](https://github.com/biomejs/biome/issues/5213): The [`noDoneCallback`](https://biomejs.dev/linter/rules/no-done-callback/) rule no longer flags false positives when a method is called on a regular variable bound to identifiers such as `before`, `after`, `beforeEach`, and `afterEach`.
