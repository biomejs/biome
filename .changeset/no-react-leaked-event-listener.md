---
"@biomejs/biome": patch
---

Added the new nursery rule [`noReactLeakedEventListener`](https://biomejs.dev/linter/rules/no-react-leaked-event-listener) rule. This rule enforces that every `addEventListener` in a React component or custom hook has a corresponding `removeEventListener` in the cleanup function of `useEffect`. Forgetting to remove an event listener can lead to memory leaks and unexpected behavior after component unmounts or dependency changes.
