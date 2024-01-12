---
title: noNegationElse (since v1.0.0)
---

**Diagnostic Category: `lint/style/noNegationElse`**

Source: <a href="https://eslint.org/docs/latest/rules/no-negated-condition" target="_blank"><code>no-negated-condition</code></a>

Disallow negation in the condition of an `if` statement if it has an `else` clause.

## Examples

### Invalid

```jsx
if (!cond) { f();} else { g();}
```

<pre class="language-text"><code class="language-text">style/noNegationElse.js:1:1 <a href="https://biomejs.dev/linter/rules/no-negation-else">lint/style/noNegationElse</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Invert blocks when performing a negation test.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>if (!cond) { f();} else { g();}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Invert the condition and the blocks.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">i</span><span style="color: Tomato;">f</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>!</strong></span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;">(</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span><span style="color: Tomato;">}</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">l</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>g</strong></span><span style="color: Tomato;">(</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span><span style="color: Tomato;">}</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>g</strong></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span><span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>f</strong></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span><span style="color: MediumSeaGreen;">}</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
!cond ? 0 : 1
```

<pre class="language-text"><code class="language-text">style/noNegationElse.js:1:1 <a href="https://biomejs.dev/linter/rules/no-negation-else">lint/style/noNegationElse</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Invert blocks when performing a negation test.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>!cond ? 0 : 1
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Invert the condition and the blocks.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>!</strong></span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">?</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">:</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>1</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">?</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">:</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>0</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
if (!cond) { f(); }
```

```jsx
cond ? 1 : 0
```

```jsx
if (!cond) { f(); }
```

```jsx
if (!!val) { f(); } else { g(); }
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
