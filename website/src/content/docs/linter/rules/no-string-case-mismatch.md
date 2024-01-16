---
title: noStringCaseMismatch (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noStringCaseMismatch`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://rust-lang.github.io/rust-clippy/master/#/match_str_case_mismatch" target="_blank"><code>match_str_case_mismatch</code></a>

Disallow comparison of expressions modifying the string case with non-compliant value.

## Examples

### Invalid

```jsx
if (s.toUpperCase() === "Abc") {}
```

<pre class="language-text"><code class="language-text">correctness/noStringCaseMismatch.js:1:5 <a href="https://biomejs.dev/linter/rules/no-string-case-mismatch">lint/correctness/noStringCaseMismatch</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This expression always returns false.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>if (s.toUpperCase() === &quot;Abc&quot;) {}
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This call convert the string to upper case</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>if (s.toUpperCase() === &quot;Abc&quot;) {}
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">... but this value is not in upper case</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>if (s.toUpperCase() === &quot;Abc&quot;) {}
   <strong>   │ </strong>                        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>upper case</strong></span><span style="color: lightgreen;"> string value.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">i</span><span style="color: Tomato;">f</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;">s</span><span style="color: Tomato;">.</span><span style="color: Tomato;">t</span><span style="color: Tomato;">o</span><span style="color: Tomato;">U</span><span style="color: Tomato;">p</span><span style="color: Tomato;">p</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">C</span><span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span><span style="color: Tomato;">(</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;">=</span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"><strong>A</strong></span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span><span style="color: Tomato;">}</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">U</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">C</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;"><strong>A</strong></span><span style="color: MediumSeaGreen;"><strong>B</strong></span><span style="color: MediumSeaGreen;"><strong>C</strong></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;">}</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
while (s.toLowerCase() === "Abc") {}
```

<pre class="language-text"><code class="language-text">correctness/noStringCaseMismatch.js:1:8 <a href="https://biomejs.dev/linter/rules/no-string-case-mismatch">lint/correctness/noStringCaseMismatch</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This expression always returns false.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>while (s.toLowerCase() === &quot;Abc&quot;) {}
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This call convert the string to lower case</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>while (s.toLowerCase() === &quot;Abc&quot;) {}
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">... but this value is not in lower case</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>while (s.toLowerCase() === &quot;Abc&quot;) {}
   <strong>   │ </strong>                           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>lower case</strong></span><span style="color: lightgreen;"> string value.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">w</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;">s</span><span style="color: Tomato;">.</span><span style="color: Tomato;">t</span><span style="color: Tomato;">o</span><span style="color: Tomato;">L</span><span style="color: Tomato;">o</span><span style="color: Tomato;">w</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">C</span><span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span><span style="color: Tomato;">(</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;">=</span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"><strong>A</strong></span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span><span style="color: Tomato;">}</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">w</span><span style="color: MediumSeaGreen;">h</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">L</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">w</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">C</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>b</strong></span><span style="color: MediumSeaGreen;"><strong>c</strong></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;">}</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
if (s.toUpperCase() === "ABC") {}
while (s.toLowerCase() === "abc") {}
for (;s.toLocaleLowerCase() === "ABC";) {}
while (s.toLocaleUpperCase() === "abc") {}
for (let s = "abc"; s === "abc"; s = s.toUpperCase()) {}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
