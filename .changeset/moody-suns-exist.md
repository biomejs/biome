---
"@biomejs/biome": patch
---

Fixed [#6547](https://github.com/biomejs/biome/issues/6547). Now the Biome CSS parser correctly parses `@starting-style` when it's used inside other at-rules. The following example doesn't raise an error anymore:

```css
@layer my-demo-layer {
  @starting-style {
    div.showing {
      background-color: red;
    }
  }
}
```
