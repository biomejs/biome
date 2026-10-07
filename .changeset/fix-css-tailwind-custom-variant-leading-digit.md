---
"@biomejs/biome": patch
---

Fixed [#12179](https://github.com/biomejs/biome/issues/12179): the CSS parser now accepts Tailwind `@custom-variant` names that start with a digit, such as the `2xl` breakpoint.

```css
@custom-variant 2xl (@container page (width >= 96rem));
```
