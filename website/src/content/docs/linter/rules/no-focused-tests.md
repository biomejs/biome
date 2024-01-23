---
title: noFocusedTests (not released)
---

**Diagnostic Category: `lint/nursery/noFocusedTests`**

:::danger
This rule hasn't been released yet.
:::

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Inspired from: <a href="https://github.com/jest-community/eslint-plugin-jest/blob/main/docs/rules/no-focused-tests.md" target="_blank"><code>no-focused-tests</code></a>

Disallow focused tests.

Disabled test are useful when developing and debugging, because it forces the test suite to run only certain tests.

However, in pull/merge request, you usually want to run all the test suite.

## Examples

### Invalid

```jsx
describe.only("foo", () => {});
```

<pre class="language-text"><code class="language-text">nursery/noFocusedTests.js:1:10 <a href="https://biomejs.dev/linter/rules/no-focused-tests">lint/nursery/noFocusedTests</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't focus the test.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>describe.only(&quot;foo&quot;, () =&gt; {});
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This is likely a change done during debugging or implementation phases, but it's unlikely what you want in production.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Remove it.</span>
  
</code></pre>

```jsx
test.only("foo", () => {});
```

<pre class="language-text"><code class="language-text">nursery/noFocusedTests.js:1:6 <a href="https://biomejs.dev/linter/rules/no-focused-tests">lint/nursery/noFocusedTests</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't focus the test.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>test.only(&quot;foo&quot;, () =&gt; {});
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This is likely a change done during debugging or implementation phases, but it's unlikely what you want in production.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Remove it.</span>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
