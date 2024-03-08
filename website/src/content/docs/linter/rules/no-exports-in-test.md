---
title: noExportsInTest (since v1.6.0)
---

**Diagnostic Category: `lint/nursery/noExportsInTest`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Inspired from: <a href="https://github.com/jest-community/eslint-plugin-jest/blob/main/docs/rules/no-export.md" target="_blank"><code>no-export</code></a>

Disallow using `export` or `module.exports` in files containing tests

This rule aims to eliminate duplicate runs of tests by exporting things from test files.
If you import from a test file, then all the tests in that file will be run in each imported instance,
so bottom line, don't export from a test, but instead move helper functions into a separate file when they need to be shared across tests.

## Examples

### Invalid

```jsx
export function myHelper() {}
describe('a test', () => {
    expect(1).toBe(1);
});
```

<pre class="language-text"><code class="language-text">nursery/noExportsInTest.js:1:1 <a href="https://biomejs.dev/linter/rules/no-exports-in-test">lint/nursery/noExportsInTest</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not export from a test file.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>export function myHelper() {}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>describe('a test', () =&gt; {
    <strong>3 │ </strong>    expect(1).toBe(1);
  
</code></pre>

### Valid

```jsx
function myHelper() {}
describe('a test', () => {
    expect(1).toBe(1);
});
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
