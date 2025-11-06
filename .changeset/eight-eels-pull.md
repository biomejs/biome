---
"@biomejs/biome": minor
---

Added support for `jsxFactory` and `jsxFragmentFactory`.Biome now respects `jsxFactory` and `jsxFragmentFactory` settings from `tsconfig.json` when using the classic JSX runtime, preventing false positive [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/) errors for custom JSX libraries like Preact.

```json5
// tsconfig.json
{
  "compilerOptions": {
    "jsx": "react",
    "jsxFactory": "h",
    "jsxFragmentFactory": "Fragment"
  }
}
```

```jsx
// Component.jsx
import { h, Fragment } from 'preact';

function App() {
  return <div>Hello</div>;
}
```
