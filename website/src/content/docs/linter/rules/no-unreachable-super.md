---
title: noUnreachableSuper (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noUnreachableSuper`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-this-before-super" target="_blank"><code>no-this-before-super</code></a>

Ensures the `super()` constructor is called exactly once on every code  path in a class constructor before `this` is accessed if the class has a superclass

## Examples

### Invalid

```jsx
class A extends B {
    constructor() {}
}
```

<pre class="language-text"><code class="language-text">correctness/noUnreachableSuper.js:2:5 <a href="https://biomejs.dev/docs/linter/rules/no-unreachable-super">lint/correctness/noUnreachableSuper</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This constructor has code paths that return without calling `</span><span style="color: Tomato;"><strong>super()</strong></span><span style="color: Tomato;">`.</span>
  
    <strong>1 │ </strong>class A extends B {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    constructor() {}
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If this is intentional, add an explicit throw statement in unsupported paths.</span>
  
</code></pre>

```jsx
class A extends B {
    constructor(value) {
        this.prop = value;
        super();
    }
}
```

<pre class="language-text"><code class="language-text">correctness/noUnreachableSuper.js:2:5 <a href="https://biomejs.dev/docs/linter/rules/no-unreachable-super">lint/correctness/noUnreachableSuper</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This constructor has code paths accessing `</span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;">` without calling `</span><span style="color: Tomato;"><strong>super()</strong></span><span style="color: Tomato;">` first.</span>
  
    <strong>1 │ </strong>class A extends B {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    constructor(value) {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        this.prop = value;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>        super();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>    }
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>}
    <strong>7 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">`</span><span style="color: lightgreen;"><strong>this</strong></span><span style="color: lightgreen;">` is accessed here:</span>
  
    <strong>1 │ </strong>class A extends B {
    <strong>2 │ </strong>    constructor(value) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        this.prop = value;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>        super();
    <strong>5 │ </strong>    }
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If this is intentional, add an explicit throw statement in unsupported paths.</span>
  
</code></pre>

```jsx
class A extends B {
    constructor(cond) {
        if(cond) {
            super();
        }
    }
}
```

<pre class="language-text"><code class="language-text">correctness/noUnreachableSuper.js:2:5 <a href="https://biomejs.dev/docs/linter/rules/no-unreachable-super">lint/correctness/noUnreachableSuper</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This constructor has code paths that return without calling `</span><span style="color: Tomato;"><strong>super()</strong></span><span style="color: Tomato;">`.</span>
  
    <strong>1 │ </strong>class A extends B {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    constructor(cond) {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        if(cond) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>            super();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>        }
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>    }
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>}
    <strong>8 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If this is intentional, add an explicit throw statement in unsupported paths.</span>
  
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
