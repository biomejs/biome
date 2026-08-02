---
"@biomejs/biome": patch
---

Fixed CSS parsing of registered custom properties and added the nursery rule [`noInconsistentPropertyInitValue`](https://biomejs.dev/linter/rules/no-inconsistent-property-init-value/). The rule reports incompatible descriptors such as `@property --size { syntax: "<length>"; initial-value: red; inherits: false; }`.
