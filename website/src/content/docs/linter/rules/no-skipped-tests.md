---
title: noSkippedTests (since v1.6.0)
---

**Diagnostic Category: `lint/nursery/noSkippedTests`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Inspired from: <a href="https://github.com/jest-community/eslint-plugin-jest/blob/main/docs/rules/no-disabled-tests.md" target="_blank"><code>no-disabled-tests</code></a>

Disallow disabled tests.

Disabled test are useful when developing and debugging, although they should not be committed in production.

## Examples

### Invalid

```jsx
describe.skip("test", () => {});
```

<pre class="language-text"><code class="language-text">nursery/noSkippedTests.js:1:10 <a href="https://biomejs.dev/linter/rules/no-skipped-tests">lint/nursery/noSkippedTests</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Don't disable tests.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>describe.skip(&quot;test&quot;, () =&gt; {});
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Disabling tests is useful when debugging or creating placeholder while working.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If this is intentional, and you want to commit a disabled test, add a suppression comment.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Enable the test.</span>
  
<strong>  </strong><strong>  1 │ </strong>describe<span style="color: Tomato;">.</span><span style="color: Tomato;">s</span><span style="color: Tomato;">k</span><span style="color: Tomato;">i</span><span style="color: Tomato;">p</span>(&quot;test&quot;,<span style="opacity: 0.8;">·</span>()<span style="opacity: 0.8;">·</span>=&gt;<span style="opacity: 0.8;">·</span>{});
<strong>  </strong><strong>    │ </strong>        <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>                   
</code></pre>

```jsx
test.skip("test", () => {});
```

<pre class="language-text"><code class="language-text">nursery/noSkippedTests.js:1:6 <a href="https://biomejs.dev/linter/rules/no-skipped-tests">lint/nursery/noSkippedTests</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Don't disable tests.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>test.skip(&quot;test&quot;, () =&gt; {});
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Disabling tests is useful when debugging or creating placeholder while working.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If this is intentional, and you want to commit a disabled test, add a suppression comment.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Enable the test.</span>
  
<strong>  </strong><strong>  1 │ </strong>test<span style="color: Tomato;">.</span><span style="color: Tomato;">s</span><span style="color: Tomato;">k</span><span style="color: Tomato;">i</span><span style="color: Tomato;">p</span>(&quot;test&quot;,<span style="opacity: 0.8;">·</span>()<span style="opacity: 0.8;">·</span>=&gt;<span style="opacity: 0.8;">·</span>{});
<strong>  </strong><strong>    │ </strong>    <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>                   
</code></pre>

## Valid

```jsx
test.only("test", () => {});
test("test", () => {});
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
