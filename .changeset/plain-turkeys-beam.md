---
"@biomejs/biome": patch
---

Fixed [#9589](https://github.com/biomejs/biome/issues/9589). Now Biome correctly parses object expressions inside props and directives. The following code doesn't emit errors anymore:

```astro
<style is:global define:vars={{ bgLight: light }}>
<Component name={{ first, name }} />
```
