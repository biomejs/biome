---
"@biomejs/biome": patch
---

The HTML formatter now formats the value of a `style` attribute as CSS, the same way it already formatted a `<style>` element. The declarations stay on the tag's line while they fit there, and break onto their own lines once they do not:

```diff
- <div style="color:#fFf;  background:red"></div>
+ <div style="color: #fff; background: red"></div>
```
