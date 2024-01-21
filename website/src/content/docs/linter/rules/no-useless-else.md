---
title: noUselessElse (since v1.3.0)
---

**Diagnostic Category: `lint/style/noUselessElse`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Inspired from: <a href="https://eslint.org/docs/latest/rules/no-else-return" target="_blank"><code>no-else-return</code></a>

Disallow `else` block when the `if` block breaks early.

If an `if` block breaks early using a breaking statement (`return`, `break`, `continue`, or `throw`),
then the `else` block becomes useless.
Its contents can be placed outside of the block.

If an `if` block breaks early using a breaking statement (`return`, `break`, `continue`, or `throw`),
then the `else` block becomes unnecessary.
This is because the content of the `else` block will never be executed in conjunction with the `if` block,
as the breaking statement ensures the control flow exits the `if` block immediately.
Therefore, the `else` block is redundant, and its content can be placed outside of the block,
reducing the indentation level by one.

## Examples

### Invalid

```jsx
while (x > 0) {
    if (f(x)) {
        break;
    } else {
        x++
    }
}
```

<pre class="language-text"><code class="language-text">style/noUselessElse.js:4:7 <a href="https://biomejs.dev/linter/rules/no-useless-else">lint/style/noUselessElse</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>else</strong></span><span style="color: Tomato;"> clause can be omitted because previous branches break early.</span>
  
    <strong>2 │ </strong>    if (f(x)) {
    <strong>3 │ </strong>        break;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    } else {
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>        x++
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>    }
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>}
    <strong>8 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Omit the </span><span style="color: lightgreen;"><strong>else</strong></span><span style="color: lightgreen;"> clause.</span>
  
    <strong>2</strong> <strong>2</strong><strong> │ </strong>      if (f(x)) {
    <strong>3</strong> <strong>3</strong><strong> │ </strong>          break;
    <strong>4</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>{</strong></span>
      <strong>4</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span>
    <strong>5</strong> <strong>5</strong><strong> │ </strong>          x++
    <strong>6</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>}</strong></span>
    <strong>7</strong> <strong>6</strong><strong> │ </strong>  }
    <strong>8</strong> <strong>7</strong><strong> │ </strong>  
  
</code></pre>

```jsx
function f(x) {
    if (x < 0) {
        return 0;
    } else {
        return x;
    }
}
```

<pre class="language-text"><code class="language-text">style/noUselessElse.js:4:7 <a href="https://biomejs.dev/linter/rules/no-useless-else">lint/style/noUselessElse</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>else</strong></span><span style="color: Tomato;"> clause can be omitted because previous branches break early.</span>
  
    <strong>2 │ </strong>    if (x &lt; 0) {
    <strong>3 │ </strong>        return 0;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    } else {
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>        return x;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>    }
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>}
    <strong>8 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Omit the </span><span style="color: lightgreen;"><strong>else</strong></span><span style="color: lightgreen;"> clause.</span>
  
    <strong>2</strong> <strong>2</strong><strong> │ </strong>      if (x &lt; 0) {
    <strong>3</strong> <strong>3</strong><strong> │ </strong>          return 0;
    <strong>4</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>{</strong></span>
      <strong>4</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span>
    <strong>5</strong> <strong>5</strong><strong> │ </strong>          return x;
    <strong>6</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>}</strong></span>
    <strong>7</strong> <strong>6</strong><strong> │ </strong>  }
    <strong>8</strong> <strong>7</strong><strong> │ </strong>  
  
</code></pre>

```jsx
function f(x) {
    if (x < 0) {
        throw new RangeError();
    } else {
        return x;
    }
}
```

<pre class="language-text"><code class="language-text">style/noUselessElse.js:4:7 <a href="https://biomejs.dev/linter/rules/no-useless-else">lint/style/noUselessElse</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>else</strong></span><span style="color: Tomato;"> clause can be omitted because previous branches break early.</span>
  
    <strong>2 │ </strong>    if (x &lt; 0) {
    <strong>3 │ </strong>        throw new RangeError();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    } else {
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>        return x;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>    }
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>}
    <strong>8 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Omit the </span><span style="color: lightgreen;"><strong>else</strong></span><span style="color: lightgreen;"> clause.</span>
  
    <strong>2</strong> <strong>2</strong><strong> │ </strong>      if (x &lt; 0) {
    <strong>3</strong> <strong>3</strong><strong> │ </strong>          throw new RangeError();
    <strong>4</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>{</strong></span>
      <strong>4</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span>
    <strong>5</strong> <strong>5</strong><strong> │ </strong>          return x;
    <strong>6</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>}</strong></span>
    <strong>7</strong> <strong>6</strong><strong> │ </strong>  }
    <strong>8</strong> <strong>7</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
function f(x) {
    if (x < 0) {
        return 0;
    }
    return x;
}
```

```jsx
function f(x) {
    if (x < 0) {
        console.info("negative number");
    } else if (x > 0) {
        return x;
    } else {
        console.info("number 0");
    }
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
