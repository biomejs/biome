---
"@biomejs/biome": patch
---
Added the nursery rule [`noScriptUrl`](https://biomejs.dev/linter/rules/no-script-url/).

This rule disallows the use of `javascript:` URLs, which are considered a form of `eval` and can pose security risks such as XSS vulnerabilities.

```jsx
<a href="javascript:alert('XSS')">Click me</a>
```
