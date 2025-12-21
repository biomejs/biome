---
'@biomejs/biome': patch
---

Related to [#8491](https://github.com/biomejs/biome/issues/8491): Fixed the issue with false positive errors for safe boolean expressions.

This new change will check for safe boolean expressions in variable declarations.

For example

Valid:

```jsx
let isOne = 1;
let isPositiveNumber = number > 0;

return <div> {isvalid && "One } { isPositiveNumber && "Is positive" }</div>
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
