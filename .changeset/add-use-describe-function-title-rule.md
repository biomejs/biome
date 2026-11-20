---
"@biomejs/biome": patch
---

Added the nursery rule [`useDescribeFunctionTitle`](https://biomejs.dev/linter/rules/use-describe-function-title/), which reports test suites (`describe` blocks) whose title is a string or a `.name` property that repeats the name of an imported value, and suggests passing the value itself as the title.

```js
import { myFunction } from "./myFunction";

describe("myFunction", () => {}); // reported
```
