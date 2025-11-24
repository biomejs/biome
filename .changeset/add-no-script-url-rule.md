---
"@biomejs/biome": minor
---

Added the rule [`noScriptUrl`](https://biomejs.dev/linter/rules/no-script-url/).

This rule disallows the use of `javascript:` URLs, which are considered a form of `eval` and can pose security risks such as XSS vulnerabilities.

```jsx
// ❌ Invalid code
<a href="javascript:void(0)">Click me</a>
<a href="javascript:alert('XSS')">Click me</a>
React.createElement('a', { href: 'javascript:void(0)' });

// ✅ Valid code
<a href="https://example.com">Click me</a>
<a href="/path/to/page">Click me</a>
<a href="#section">Click me</a>
```

Contributed by @romano
