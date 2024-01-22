---
title: noSkippedTests (not released)
---

**Diagnostic Category: `lint/nursery/noSkippedTests`**

:::danger
This rule hasn't been released yet.
:::

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

<pre class="language-text"><code class="language-text">nursery/noSkippedTests.js:1:10 <a href="https://biomejs.dev/linter/rules/no-skipped-tests">lint/nursery/noSkippedTests</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Don't disable tests.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>describe.skip(&quot;test&quot;, () =&gt; {});
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Disabling tests is useful when debugging or creating placeholder while working.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If this is intentional, and you want to commit a disabled test, add a suppression comment.</span>
  
</code></pre>

```jsx
test.skip("test", () => {});
```

<pre class="language-text"><code class="language-text">nursery/noSkippedTests.js:1:6 <a href="https://biomejs.dev/linter/rules/no-skipped-tests">lint/nursery/noSkippedTests</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Don't disable tests.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>test.skip(&quot;test&quot;, () =&gt; {});
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Disabling tests is useful when debugging or creating placeholder while working.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If this is intentional, and you want to commit a disabled test, add a suppression comment.</span>
  
</code></pre>

## Valid

```jsx
test.only("test", () => {});
test("test", () => {});
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
