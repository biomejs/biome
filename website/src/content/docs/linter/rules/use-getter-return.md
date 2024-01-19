---
title: useGetterReturn (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/useGetterReturn`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/getter-return" target="_blank"><code>getter-return</code></a>

Enforce `get` methods to always return a value.

## Examples

### Invalid

```jsx
class Person {
    get firstName() {}
}
```

<pre class="language-text"><code class="language-text">suspicious/useGetterReturn.js:2:5 <a href="https://biomejs.dev/linter/rules/use-getter-return">lint/suspicious/useGetterReturn</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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
    }
}
```

<pre class="language-text"><code class="language-text">suspicious/useGetterReturn.js:3:9 <a href="https://biomejs.dev/linter/rules/use-getter-return">lint/suspicious/useGetterReturn</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>return</strong></span><span style="color: Tomato;"> should return a value because it is located in a </span><span style="color: Tomato;"><strong>getter</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>const obj = {
    <strong>2 │ </strong>    get firstName() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        return;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    }
    <strong>5 │ </strong>}
  
</code></pre>

```jsx
class Option {
    get value() {
        if (this.hasValue) {
            log();
        } else {
            return null;
        }
    }
}
```

<pre class="language-text"><code class="language-text">suspicious/useGetterReturn.js:2:5 <a href="https://biomejs.dev/linter/rules/use-getter-return">lint/suspicious/useGetterReturn</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>getter</strong></span><span style="color: Tomato;"> should </span><span style="color: Tomato;"><strong>return</strong></span><span style="color: Tomato;"> a value.</span>
  
     <strong>1 │ </strong>class Option {
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    get value() {
    <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        if (this.hasValue) {
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>            log();
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>        } else {
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>            return null;
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>7 │ </strong>        }
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>8 │ </strong>    }
    <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
     <strong>9 │ </strong>}
    <strong>10 │ </strong>
  
</code></pre>

### Valid

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
    }
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
