---
title: Lint Rule useGetterReturn
parent: lint/rules/index
---

# useGetterReturn (since v1.0.0)

Enforces the presence of non-empty `return` statements in getters.

A _getter_ allows defining a property which is dynamically computed.
Thus, it is desirable that a _getter_ returns a value.

Source: https://eslint.org/docs/latest/rules/getter-return

## Examples

### Invalid

```jsx
class Person {
    get firstName() {}
}
```

<pre class="language-text"><code class="language-text">nursery/useGetterReturn.js:2:5 <a href="https://biomejs.dev/lint/rules/useGetterReturn">lint/nursery/useGetterReturn</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>getter</strong></span><span style="color: Tomato;"> should </span><span style="color: Tomato;"><strong>return</strong></span><span style="color: Tomato;"> a value.</span>
  
    <strong>1 │ </strong>class Person {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    get firstName() {}
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
</code></pre>

```jsx
const obj = {
    get firstName() {
        return;
    },
}
```

<pre class="language-text"><code class="language-text">nursery/useGetterReturn.js:3:9 <a href="https://biomejs.dev/lint/rules/useGetterReturn">lint/nursery/useGetterReturn</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>return</strong></span><span style="color: Tomato;"> should return a value because it is located in a </span><span style="color: Tomato;"><strong>return</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>const obj = {
    <strong>2 │ </strong>    get firstName() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        return;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    },
    <strong>5 │ </strong>}
  
</code></pre>

## Valid

```jsx
class Person {
    get firstName() {
        return this.fullname.split(" ")[0];
    }
}
```

```jsx
const obj = {
    get firstName() {
        return this.fullname.split(" ")[0];
    },
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
