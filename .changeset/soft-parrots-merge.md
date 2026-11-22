---
"@biomejs/biome": patch
---

Fixed [#13354](https://github.com/biomejs/biome/issues/13354): [`useAwaitThenable`](https://biomejs.dev/linter/rules/use-await-thenable/) no longer reports `await` on calls that select a Promise-returning overload declared inside a namespace, such as React's `act`:

```tsx
import { act } from "react";

test("renders", async () => {
    // The Promise overload of `act` is selected for the async callback.
    await act(async () => {});
});
```

Namespace members and exports now resolve same-name function overloads to the merged overload set, so call matching can select among all signatures as it already does for top-level and interface overloads.
