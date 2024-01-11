---
title: useNumericLiterals (since v1.0.0)
---

**Diagnostic Category: `lint/style/useNumericLiterals`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/prefer-numeric-literals" target="_blank"><code>prefer-numeric-literals</code></a>

Disallow `parseInt()` and `Number.parseInt()` in favor of binary, octal, and hexadecimal literals

_JavaScript_ provides literal forms for binary, octal, and hexadecimal numbers.
For example: `0b11`, `0o77`, and `0xff`.
Using the literal forms enable static code analysis and avoid unnecessary computations.

## Examples

### Invalid

```jsx
parseInt("111110111", 2);
```

<pre class="language-text"><code class="language-text">style/useNumericLiterals.js:1:1 <a href="https://biomejs.dev/linter/rules/use-numeric-literals">lint/style/useNumericLiterals</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This call to </span><span style="color: Tomato;"><strong>parseInt()</strong></span><span style="color: Tomato;"> can be replaced by a binary literal.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>parseInt(&quot;111110111&quot;, 2);
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Using a literal avoids unnecessary computations.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use the computed binary literal instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>I</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>(</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>,</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>2</strong></span><span style="color: Tomato;"><strong>)</strong></span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>b</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
Number.parseInt("767", 8);
```

<pre class="language-text"><code class="language-text">style/useNumericLiterals.js:1:1 <a href="https://biomejs.dev/linter/rules/use-numeric-literals">lint/style/useNumericLiterals</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This call to </span><span style="color: Tomato;"><strong>Number.parseInt()</strong></span><span style="color: Tomato;"> can be replaced by an octal literal.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>Number.parseInt(&quot;767&quot;, 8);
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Using a literal avoids unnecessary computations.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use the computed octal literal instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>N</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>I</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>(</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>7</strong></span><span style="color: Tomato;"><strong>6</strong></span><span style="color: Tomato;"><strong>7</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>,</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>8</strong></span><span style="color: Tomato;"><strong>)</strong></span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>7</strong></span><span style="color: MediumSeaGreen;"><strong>6</strong></span><span style="color: MediumSeaGreen;"><strong>7</strong></span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
Number.parseInt("-1f7", 16);
```

<pre class="language-text"><code class="language-text">style/useNumericLiterals.js:1:1 <a href="https://biomejs.dev/linter/rules/use-numeric-literals">lint/style/useNumericLiterals</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This call to </span><span style="color: Tomato;"><strong>Number.parseInt()</strong></span><span style="color: Tomato;"> can be replaced by a hexadecimal literal.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>Number.parseInt(&quot;-1f7&quot;, 16);
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Using a literal avoids unnecessary computations.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use the computed hexadecimal literal instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>N</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>I</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>(</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">-</span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;"><strong>7</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>,</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>6</strong></span><span style="color: Tomato;"><strong>)</strong></span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">-</span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>x</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>f</strong></span><span style="color: MediumSeaGreen;"><strong>7</strong></span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
parseInt(1);
parseInt(1, 3);
Number.parseInt(1);
Number.parseInt(1, 3);

0b111110111 === 503;
0o767 === 503;
0x1F7 === 503;

a[parseInt](1,2);

parseInt(foo);
parseInt(foo, 2);
Number.parseInt(foo);
Number.parseInt(foo, 2);
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
