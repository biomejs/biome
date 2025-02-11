---
"@biomejs/biome": major
---

The rule `lint/a11y/useAltText` doesn't check the element's attributes containing object spread.

The following code doesn't trigger the rule anymore:

```jsx
<img src="test.png" alt={alt} {...restProps}></img>
```
