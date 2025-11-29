---
'@biomejs/biome': patch
---

Fixed [#8288](https://github.com/biomejs/biome/issues/8288): Fixed the issue with false positive errors

This new change will ignore attribute and only show diagnostics for JSX Expressions

For example

Valid:

```jsx
<Something checked={isOpen && items.length} />
```

Invalid:
```jsx
const Component = () =>{
  return isOpen && items.length
}
```
