---
"@biomejs/biome": major
---

The rule [`useAltText`](https://biomejs.dev/linter/rules/use-alt-text/) no longer checks the element's attributes containing object spread.

The following code doesn't trigger the rule anymore:

```jsx
<img src="test.png" alt={alt} {...restProps}></img>
```
