---
title: Lint Rule noConstructorReturn
parent: lint/rules/index
---

# noConstructorReturn (since v11.0.0)

> This rule is recommended by Rome.

Disallow returning a value from a `constructor`.

Returning a value from a `constructor` of a class is a possible error.
Forbidding this pattern prevents errors resulting from unfamiliarity with JavaScript or a copy-paste error.

Only returning without a value is allowed, as it’s a control flow statement.

Source: https://eslint.org/docs/latest/rules/no-constructor-return

## Examples

### Invalid

```jsx
class A {
    constructor() {
        return 0;
    }
}
```

<pre class="language-text"><code class="language-text">correctness/noConstructorReturn.js:3:9 <a href="https://biomejs.dev/lint/rules/noConstructorReturn">lint/correctness/noConstructorReturn</a> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>constructor</strong></span><span style="color: Tomato;"> should not </span><span style="color: Tomato;"><strong>return</strong></span><span style="color: Tomato;"> a value.</span>
  
    <strong>1 │ </strong>class A {
    <strong>2 │ </strong>    constructor() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        return 0;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    }
    <strong>5 │ </strong>}
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The constructor is here:</span>
  
    <strong>1 │ </strong>class A {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    constructor() {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        return 0;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    }
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>}
    <strong>6 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Returning a value from a constructor is ignored.</span>
  
</code></pre>

### Valid

```jsx
class A {
    constructor() {}
}
```

```jsx
class B {
    constructor(x) {
        return;
    }
}
```

```
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
