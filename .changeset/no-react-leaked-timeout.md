---
"@biomejs/biome": patch
---

Added the new nursery rule [`noReactLeakedTimeout`](https://biomejs.dev/linter/rules/no-react-leaked-timeout). This rule enforces that every `setTimeout` in a React component or custom hook has a corresponding `clearTimeout` in the cleanup function of `useEffect`. Forgetting to clear a timeout can lead to memory leaks and unexpected callback executions after component unmounts or dependency changes.
