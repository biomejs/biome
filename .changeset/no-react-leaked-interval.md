---
"@biomejs/biome": patch
---

Added the new nursery rule [`noReactLeakedInterval`](https://biomejs.dev/linter/rules/no-react-leaked-interval) rule. This rule enforces that every `setInterval` in a React component or custom hook has a corresponding `clearInterval` in the cleanup function of `useEffect`. Forgetting to clear an interval can lead to memory leaks and unexpected callback executions after component unmounts or dependency changes.
