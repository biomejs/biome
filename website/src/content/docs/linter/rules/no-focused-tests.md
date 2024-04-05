---
title: noFocusedTests (since v1.6.0)
---

**Diagnostic Category: `lint/nursery/noFocusedTests`**

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

<pre class="language-text"><code class="language-text">nursery/noFocusedTests.js:1:10 <a href="https://biomejs.dev/linter/rules/no-focused-tests">lint/nursery/noFocusedTests</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't focus the test.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>describe.only(&quot;foo&quot;, () =&gt; {});
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The 'only' method is often used for debugging or during implementation. It should be removed before deploying to production.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Consider removing 'only' to ensure all tests are executed.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove focus from test.</span>
  
<strong>  </strong><strong>  1 │ </strong>describe<span style="color: Tomato;">.</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">l</span><span style="color: Tomato;">y</span>(&quot;foo&quot;,<span style="opacity: 0.8;">·</span>()<span style="opacity: 0.8;">·</span>=&gt;<span style="opacity: 0.8;">·</span>{});
<strong>  </strong><strong>    │ </strong>        <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>                  
</code></pre>

```jsx
test.only("foo", () => {});
```

<pre class="language-text"><code class="language-text">nursery/noFocusedTests.js:1:6 <a href="https://biomejs.dev/linter/rules/no-focused-tests">lint/nursery/noFocusedTests</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't focus the test.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>test.only(&quot;foo&quot;, () =&gt; {});
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The 'only' method is often used for debugging or during implementation. It should be removed before deploying to production.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Consider removing 'only' to ensure all tests are executed.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove focus from test.</span>
  
<strong>  </strong><strong>  1 │ </strong>test<span style="color: Tomato;">.</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">l</span><span style="color: Tomato;">y</span>(&quot;foo&quot;,<span style="opacity: 0.8;">·</span>()<span style="opacity: 0.8;">·</span>=&gt;<span style="opacity: 0.8;">·</span>{});
<strong>  </strong><strong>    │ </strong>    <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>                  
</code></pre>

### Valid

```jsx
test("foo", () => {});
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
