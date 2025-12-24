---
'@biomejs/biome': patch
---

Fixed [#8491](https://github.com/biomejs/biome/issues/8491): Fixed the issue with false positive errors for safe boolean expressions. There are still pending fixes. Head to [#8491 (comment)](https://github.com/biomejs/biome/issues/8491#issuecomment-3669243551) for more details

This new change will check for safe boolean expressions in variable declarations.

For example,

Valid:

```jsx
let isOne = 1;
let isPositiveNumber = number > 0;

return <div> {isOne && "One" } { isPositiveNumber && "Is positive" }</div>
```

Invalid:

```jsx
let emptyStr = '';
let isZero = 0;

return (
  <div>
    {emptyStr && 'Empty String'} {isZero && 'Number is zero'}{' '}
  </div>
);
```
