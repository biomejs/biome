---
"@biomejs/biome": patch
---

Added the nursery rule [`noIteratorProperty`](https://biomejs.dev/linter/rules/no-iterator-property/), which disallows the obsolete, non-standard `__iterator__` property. The rule reports code such as `Foo.prototype.__iterator__ = function () {};`.
