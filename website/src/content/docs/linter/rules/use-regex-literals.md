---
title: useRegexLiterals (since v1.3.0)
---

**Diagnostic Category: `lint/complexity/useRegexLiterals`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/prefer-regex-literals" target="_blank"><code>prefer-regex-literals</code></a>

Enforce the use of the regular expression literals instead of the RegExp constructor if possible.

There are two ways to create a regular expression:

- Regular expression literals, e.g., `/abc/u`.
- The RegExp constructor function, e.g., `new RegExp("abc", "u")` .

The constructor function is particularly useful when you want to dynamically generate the pattern,
because it takes string arguments.

Using regular expression literals avoids some escaping required in a string literal,
and are easier to analyze statically.

## Examples

### Invalid

```jsx
new RegExp("abc", "u");
```

<pre class="language-text"><code class="language-text">complexity/useRegexLiterals.js:1:1 <a href="https://biomejs.dev/linter/rules/use-regex-literals">lint/complexity/useRegexLiterals</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use a regular expression literal instead of the </span><span style="color: Tomato;"><strong>RegExp</strong></span><span style="color: Tomato;"> constructor.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>new RegExp(&quot;abc&quot;, &quot;u&quot;);
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Regular expression literals avoid some escaping required in a string literal, and are easier to analyze statically.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use a </span><span style="color: lightgreen;"><strong>literal notation</strong></span><span style="color: lightgreen;"> instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>w</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>R</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>g</strong></span><span style="color: Tomato;"><strong>E</strong></span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>(</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">a</span><span style="color: Tomato;">b</span><span style="color: Tomato;">c</span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>,</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">u</span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>)</strong></span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>/</strong></span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;"><strong>/</strong></span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
/abc/u;

new RegExp("abc", flags);
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
