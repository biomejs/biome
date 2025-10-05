---
"@biomejs/biome": patch
---

Added a nursery rule [`noDeprecatedProperties`](https://biomejs.dev/linter/rules/no-deprecated-properties/) that detects usages of deprecated CSS properties. The rule was ported from the [`property-no-deprecated`](https://stylelint.io/user-guide/rules/property-no-deprecated) rule in Stylelint.

For example, the following patterns are considered as invalid:

```css
a {
  word-wrap: break-word;
}
```

If applicable, Biome will suggest to replace it with another valid property:

```css
a {
  overflow-wrap: break-word;
}
```
