---
"@biomejs/biome": patch
---

Fixed [#8117](https://github.com/biomejs/biome/issues/8117): [`useValidLang`](https://biomejs.dev/linter/rules/use-valid-lang/) now accepts valid [BCP 47 language tags](https://developer.mozilla.org/en-US/docs/Glossary/BCP_47_language_tag) with script subtags.

**Valid:**

```html
<html lang="zh-Hans-CN"></html>
```
