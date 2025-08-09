---
"@biomejs/biome": patch
---

Added the rule [`noNextAsyncClientComponent`](https://biomejs.dev/linter/rules/no-next-async-client-component).

This rule prevents the use of async functions for client components in Next.js applications. Client components marked with "use client" directive should not be async as this can cause hydration mismatches, break component rendering lifecycle, and lead to unexpected behavior with React's concurrent features.

```jsx
"use client";

// Invalid - async client component
export default async function MyComponent() {
  return <div>Hello</div>;
}

// Valid - synchronous client component
export default function MyComponent() {
  return <div>Hello</div>;
}
```
