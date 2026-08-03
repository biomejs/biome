---
"@biomejs/biome": patch
---

Added the nursery rule [`noInconsistentPropertyInitValue`](https://biomejs.dev/linter/rules/no-inconsistent-property-init-value/), which reports an `@property` whose `initial-value` does not match its `syntax` descriptor. For example, the following declaration triggers the rule because `red` is not a `<length>`:

```css
@property --size {
  syntax: "<length>";
  inherits: false;
  initial-value: red;
}
```
