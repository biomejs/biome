---
title: useGetterReturn (since v1.0.0)
---


Enforce `get` methods to always return a value.

Source: https://eslint.org/docs/latest/rules/getter-return

## Examples

### Invalid

```jsx
class Person {
    get firstName() {}
}
```

<pre class="language-text"><code class="language-text">nursery/useGetterReturn.js:2:5 <a href="https://biomejs.dev/linter/rules/use-getter-return">lint/nursery/useGetterReturn</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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

<pre class="language-text"><code class="language-text">nursery/useGetterReturn.js:3:9 <a href="https://biomejs.dev/linter/rules/use-getter-return">lint/nursery/useGetterReturn</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>return</strong></span><span style="color: Tomato;"> should return a value because it is located in a </span><span style="color: Tomato;"><strong>return</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>const obj = {
    <strong>2 │ </strong>    get firstName() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        return;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    },
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
    },
}
```

<pre class="language-text"><code class="language-text">nursery/useGetterReturn.js:8:6 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">expected an identifier, a string literal, a number literal, a private field name, or a computed name but instead found ','</span>
  
     <strong>6 │ </strong>            return null;
     <strong>7 │ </strong>        }
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>8 │ </strong>    },
    <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong>
     <strong>9 │ </strong>}
    <strong>10 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Expected an identifier, a string literal, a number literal, a private field name, or a computed name here</span>
  
     <strong>6 │ </strong>            return null;
     <strong>7 │ </strong>        }
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>8 │ </strong>    },
    <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong>
     <strong>9 │ </strong>}
    <strong>10 │ </strong>
  
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
