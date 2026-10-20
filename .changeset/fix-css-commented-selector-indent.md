---
"@biomejs/biome": patch
---

Fixed [#12471](https://github.com/biomejs/biome/issues/12471): a comment above a CSS rule no longer removes the indentation of the rule's wrapped selector.

```css
/* comment */
.sidebar-navigation
  .sidebar-navigation-item-with-a-long-descriptive-class-name {
  color: red;
}
```
