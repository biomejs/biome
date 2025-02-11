---
@biomejs/biome: major
---

The rule `lint/a11y/useAltText` doesn't check the element's attributes containing object spread.

The following code can pass check:

```jsx
<img src="test.png" alt={alt} {...restProps}></img>
```