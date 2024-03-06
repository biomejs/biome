---
title: noEmptyBlockStatements (since v1.3.0)
---

**Diagnostic Category: `lint/suspicious/noEmptyBlockStatements`**

Source: <a href="https://eslint.org/docs/latest/rules/no-empty" target="_blank"><code>no-empty</code></a>

Disallow empty block statements and static blocks.

Empty static blocks and block statements, while not technically errors, usually occur due to refactoring that wasn’t completed. They can cause confusion when reading code.

This rule disallows empty block statements and static blocks.
This rule ignores block statements or static blocks which contain a comment (for example, in an empty catch or finally block of a try statement to indicate that execution should continue regardless of errors).

## Examples

### Invalid

```jsx
function emptyFunctionBody () {}
```

<pre class="language-text"><code class="language-text">suspicious/noEmptyBlockStatements.js:1:31 <a href="https://biomejs.dev/linter/rules/no-empty-block-statements">lint/suspicious/noEmptyBlockStatements</a> ━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Unexpected empty block.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function emptyFunctionBody () {}
   <strong>   │ </strong>                              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Empty blocks are usually the result of an incomplete refactoring. Remove the empty block or add a comment inside it if it is intentional.</span>
  
</code></pre>

```jsx
try {
    doSomething();
} catch(ex) {

}
```

<pre class="language-text"><code class="language-text">suspicious/noEmptyBlockStatements.js:3:13 <a href="https://biomejs.dev/linter/rules/no-empty-block-statements">lint/suspicious/noEmptyBlockStatements</a> ━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Unexpected empty block.</span>
  
    <strong>1 │ </strong>try {
    <strong>2 │ </strong>    doSomething();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>} catch(ex) {
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Empty blocks are usually the result of an incomplete refactoring. Remove the empty block or add a comment inside it if it is intentional.</span>
  
</code></pre>

```jsx
class Foo {
  static {}
}
```

<pre class="language-text"><code class="language-text">suspicious/noEmptyBlockStatements.js:2:3 <a href="https://biomejs.dev/linter/rules/no-empty-block-statements">lint/suspicious/noEmptyBlockStatements</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Unexpected empty block.</span>
  
    <strong>1 │ </strong>class Foo {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  static {}
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Empty blocks are usually the result of an incomplete refactoring. Remove the empty block or add a comment inside it if it is intentional.</span>
  
</code></pre>

### Valid

```jsx
function foo () {
    doSomething();
}
```

```jsx
try {
  doSomething();
} catch (ex) {
  // continue regardless of error
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
