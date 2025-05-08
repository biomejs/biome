---
"@biomejs/biome": minor
---

Added a new rule [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/), which detects any missing cases for switch statements.
Currently, it supports only literal union types.

For example:

```ts
type Day =
  | 'Monday'
  | 'Tuesday'
  | 'Wednesday'
  | 'Thursday'
  | 'Friday'
  | 'Saturday'
  | 'Sunday';

const day: Day = 'Monday';
let result = 0;

switch (day) {
  case 'Monday': {
    result = 1;
    break;
  }
}
```

The switch statement is missing other cases than `'Monday'`, which will cause a runtime error.
To fix this issue, add missing cases or a default case to the statement.
