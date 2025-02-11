---
cli: minor
biome_js_analyze: minor
---

# Rule `lint/a11y/useAltText` don't check element's attributes contain object spread.

The following code can pass check:

```jsx
<img src="test.png" alt={alt} {...restProps}></img>
```