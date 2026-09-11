---
"@biomejs/biome": patch
---

Fixed [#6206](https://github.com/biomejs/biome/issues/6206): [`useUniqueElementIds`](https://biomejs.dev/linter/rules/use-unique-element-ids) no longer reports static IDs on elements in SVG contexts.

```jsx
<svg>
    <defs>
        <pattern id="dots" width="10" height="10" />
    </defs>
    <rect fill="url(#dots)" width="100%" height="100%" />
</svg>
```
