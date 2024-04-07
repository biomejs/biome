---
title: noDuplicateTestHooks (since v1.6.0)
---

**Diagnostic Category: `lint/nursery/noDuplicateTestHooks`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Inspired from: <a href="https://github.com/jest-community/eslint-plugin-jest/blob/main/docs/rules/no-duplicate-hooks.md" target="_blank"><code>no-duplicate-hooks</code></a>

A `describe` block should not contain duplicate hooks.

## Examples

### Invalid

```jsx
describe('foo', () => {
  beforeEach(() => {
    // some setup
  });
  beforeEach(() => {
    // some setup
  });
  test('foo_test', () => {
   // some test
  });
});
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateTestHooks.js:5:3 <a href="https://biomejs.dev/linter/rules/no-duplicate-test-hooks">lint/nursery/noDuplicateTestHooks</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Disallow duplicate setup and teardown hooks.</span>
  
    <strong>3 │ </strong>    // some setup
    <strong>4 │ </strong>  });
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>  beforeEach(() =&gt; {
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>    // some setup
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>7 │ </strong>  });
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>8 │ </strong>  test('foo_test', () =&gt; {
    <strong>9 │ </strong>   // some test
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Disallow </span><span style="color: lightgreen;"><strong>beforeEach</strong></span><span style="color: lightgreen;"> duplicacy inside the describe function.</span>
  
</code></pre>

```jsx
describe('foo', () => {
  beforeEach(() => {
    // some setup
  });
  test('foo_test', () => {
    afterAll(() => {
      // some teardown
    });
   afterAll(() => {
     // some teardown
   });
  });
});
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateTestHooks.js:9:4 <a href="https://biomejs.dev/linter/rules/no-duplicate-test-hooks">lint/nursery/noDuplicateTestHooks</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Disallow duplicate setup and teardown hooks.</span>
  
     <strong>7 │ </strong>      // some teardown
     <strong>8 │ </strong>    });
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>9 │ </strong>   afterAll(() =&gt; {
    <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>10 │ </strong>     // some teardown
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>11 │ </strong>   });
    <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>12 │ </strong>  });
    <strong>13 │ </strong>});
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Disallow </span><span style="color: lightgreen;"><strong>afterAll</strong></span><span style="color: lightgreen;"> duplicacy inside the describe function.</span>
  
</code></pre>

### Valid

```jsx
describe('foo', () => {
  beforeEach(() => {
    // some setup
  });
  test('foo_test', () => {
    // some test
  });
});
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
