---
title: noExcessiveNestedTestSuites (since v1.6.0)
---

**Diagnostic Category: `lint/nursery/noExcessiveNestedTestSuites`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://github.com/jest-community/eslint-plugin-jest/blob/main/docs/rules/max-nested-describe.md" target="_blank"><code>max-nested-describe</code></a>

This rule enforces a maximum depth to nested `describe()` in test files.

To improve code clarity in your tests, the rule limits nested `describe` to 5.

## Examples

### Invalid

```jsx
describe('foo', () => {
  describe('bar', () => {
    describe('baz', () => {
      describe('qux', () => {
        describe('quxx', () => {
          describe('too many', () => {
            it('should get something', () => {
              expect(getSomething()).toBe('Something');
            });
          });
        });
      });
    });
  });
});
```

<pre class="language-text"><code class="language-text">nursery/noExcessiveNestedTestSuites.js:6:11 <a href="https://biomejs.dev/linter/rules/no-excessive-nested-test-suites">lint/nursery/noExcessiveNestedTestSuites</a> ━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Excessive `describe()` nesting detected.</span>
  
     <strong>4 │ </strong>      describe('qux', () =&gt; {
     <strong>5 │ </strong>        describe('quxx', () =&gt; {
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>          describe('too many', () =&gt; {
    <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>7 │ </strong>            it('should get something', () =&gt; {
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>8 │ </strong>              expect(getSomething()).toBe('Something');
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>9 │ </strong>            });
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>10 │ </strong>          });
    <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>11 │ </strong>        });
    <strong>12 │ </strong>      });
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Excessive nesting of </span><span style="color: lightgreen;"><strong>describe()</strong></span><span style="color: lightgreen;"> calls can hinder test readability.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Consider refactoring and </span><span style="color: lightgreen;"><strong>reduce the level of nested describe</strong></span><span style="color: lightgreen;"> to improve code clarity.</span>
  
</code></pre>

### Valid

```jsx
describe('foo', () => {
  describe('bar', () => {
    it('should get something', () => {
      expect(getSomething()).toBe('Something');
    });
  });
  describe('qux', () => {
    it('should get something', () => {
      expect(getSomething()).toBe('Something');
    });
  });
});
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
