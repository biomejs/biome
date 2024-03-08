---
title: noUselessLoneBlockStatements (since v1.3.3)
---

**Diagnostic Category: `lint/complexity/noUselessLoneBlockStatements`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-lone-blocks" target="_blank"><code>no-lone-blocks</code></a>

Disallow unnecessary nested block statements.

>In JavaScript, prior to ES6, standalone code blocks delimited by curly braces do not create a new scope and have no use.
In ES6, code blocks may create a new scope if a block-level binding (let and const), a class declaration or a function declaration (in strict mode) are present. A block is not considered redundant in these cases.


## Examples

### Invalid

```jsx
{}
```

<pre class="language-text"><code class="language-text">complexity/noUselessLoneBlockStatements.js:1:1 <a href="https://biomejs.dev/linter/rules/no-useless-lone-block-statements">lint/complexity/noUselessLoneBlockStatements</a> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This block statement doesn't serve any purpose and can be safely removed.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>{}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Standalone block statements without any block-level declarations are redundant in JavaScript and can be removed to simplify the code.</span>
  
</code></pre>

```jsx
if (foo) {
  bar();
  {
    baz();
  }
}
```

<pre class="language-text"><code class="language-text">complexity/noUselessLoneBlockStatements.js:3:3 <a href="https://biomejs.dev/linter/rules/no-useless-lone-block-statements">lint/complexity/noUselessLoneBlockStatements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This block statement doesn't serve any purpose and can be safely removed.</span>
  
    <strong>1 │ </strong>if (foo) {
    <strong>2 │ </strong>  bar();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  {
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    baz();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>  }
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>}
    <strong>7 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Standalone block statements without any block-level declarations are redundant in JavaScript and can be removed to simplify the code.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove redundant block.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  if (foo) {
    <strong>2</strong> <strong>2</strong><strong> │ </strong>    bar();
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>{</strong></span>
    <strong>4</strong> <strong>3</strong><strong> │ </strong>      baz();
    <strong>5</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>}</strong></span>
    <strong>6</strong> <strong>4</strong><strong> │ </strong>  }
    <strong>7</strong> <strong>5</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
while (foo) {
  bar();
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
