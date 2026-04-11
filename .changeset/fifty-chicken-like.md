---
"@biomejs/biome": patch
---

Added the new nursery rule [`noReactStringRefs`](https://biomejs.dev/linter/rules/no-react-string-refs/), which disallows legacy React string refs such as `ref="hello"` and `this.refs.hello`.

Biome also reports template-literal refs such as ``ref={`hello`}``, so React code can consistently migrate to callback refs, `createRef()`, or `useRef()`.
