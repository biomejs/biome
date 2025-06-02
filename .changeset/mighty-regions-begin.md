---
"@biomejs/biome": minor
---

Added the rule [noExcessiveLinesPerFunction](https://biomejs.dev/linter/rules/no-excessive-lines-per-function/).
This rule restrict a maximum number of lines of code in a function body.

The following code is now reported as invalid when the limit of maximum lines is set to 2:

```js
function foo() {
  const x = 0;
  const y = 1;
  const z = 2;
}
```

The following code is now reported as valid when the limit of maximum lines is set to 3:

```jsx
const bar = () => {
  const x = 0;
  const z = 2;
};
```
