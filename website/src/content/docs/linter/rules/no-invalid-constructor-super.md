---
title: noInvalidConstructorSuper (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noInvalidConstructorSuper`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/constructor-super" target="_blank"><code>constructor-super</code></a>

Prevents the incorrect use of `super()` inside classes. It also checks whether a call `super()` is missing from classes that extends other constructors.

## Examples

### Invalid

```jsx
class A {
    constructor() {
        super();
    }
}
```

<pre class="language-text"><code class="language-text">correctness/noInvalidConstructorSuper.js:3:9 <a href="https://biomejs.dev/linter/rules/no-invalid-constructor-super">lint/correctness/noInvalidConstructorSuper</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This class should not have a </span><span style="color: Tomato;"><strong>super()</strong></span><span style="color: Tomato;"> call. You should remove it.</span>
  
    <strong>1 │ </strong>class A {
    <strong>2 │ </strong>    constructor() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        super();
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    }
    <strong>5 │ </strong>}
  
</code></pre>

```jsx
class A extends undefined {
    constructor() {
        super();
    }
}
```

<pre class="language-text"><code class="language-text">correctness/noInvalidConstructorSuper.js:3:9 <a href="https://biomejs.dev/linter/rules/no-invalid-constructor-super">lint/correctness/noInvalidConstructorSuper</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This class calls </span><span style="color: Tomato;"><strong>super()</strong></span><span style="color: Tomato;">, but the class extends from a non-constructor.</span>
  
    <strong>1 │ </strong>class A extends undefined {
    <strong>2 │ </strong>    constructor() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        super();
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    }
    <strong>5 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This is where the non-constructor is used.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>class A extends undefined {
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    constructor() {
    <strong>3 │ </strong>        super();
  
</code></pre>

### Valid

```jsx
export default class A extends B {
    constructor() {
        super();
    }
}
```

```jsx
export class A {
    constructor() {}
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
