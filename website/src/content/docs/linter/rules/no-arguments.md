---
title: noArguments (since v1.0.0)
---

**Diagnostic Category: `lint/style/noArguments`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/prefer-rest-params" target="_blank"><code>prefer-rest-params</code></a>

Disallow the use of `arguments`.

## Examples

### Invalid

```jsx
function f() {
   console.log(arguments);
}
```

<pre class="language-text"><code class="language-text">style/noArguments.js:2:16 <a href="https://biomejs.dev/linter/rules/no-arguments">lint/style/noArguments</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use the </span><span style="color: Tomato;"><strong>rest parameters</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>arguments</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>function f() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>   console.log(arguments);
   <strong>   │ </strong>               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;"><strong>arguments</strong></span><span style="color: lightgreen;"> does not have </span><span style="color: lightgreen;"><strong>Array.prototype</strong></span><span style="color: lightgreen;"> methods and can be inconvenient to use.</span>
  
</code></pre>

### Valid

```js
function f() {
    let arguments = 1;
    console.log(arguments);
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
