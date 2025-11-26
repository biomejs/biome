---
"@biomejs/biome": patch
---

Added the rule [`noScriptUrl`](https://biomejs.dev/linter/rules/no-script-url/).

This rule disallows the use of `javascript:` URLs, which are considered a form of `eval` and can pose security risks such as XSS vulnerabilities.

**Invalid code**
```jsx
<a href="javascript:void(0)">Click me</a>
<a href="javascript:alert('XSS')">Click me</a>
React.createElement('a', { href: 'javascript:void(0)' });
<Link href="javascript:void(0)">Click me</Link>
<A href="javascript:void(0)">Click me</A>
<my-link href="javascript:void(0)">Click me</my-link>
```
**Valid code**
```jsx
<a href="https://example.com">Click me</a>
<a href="/path/to/page">Click me</a>
<a href="#section">Click me</a>
<span href="javascript:alert('XSS')">Click me</span>
React.createElement('span', { href: 'javascript:void(0)' });
```

