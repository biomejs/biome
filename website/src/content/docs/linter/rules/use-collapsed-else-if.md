---
title: useCollapsedElseIf (since v1.1.0)
---

**Diagnostic Category: `lint/style/useCollapsedElseIf`**

Source: <a href="https://eslint.org/docs/latest/rules/no-lonely-if" target="_blank"><code>no-lonely-if</code></a>

Enforce using `else if` instead of nested `if` in `else` clauses.

If an `if` statement is the only statement in the `else` block, it is often clearer to use an `else if` form.

## Examples

### Invalid

```jsx
if (condition) {
    // ...
} else {
    if (anotherCondition) {
        // ...
    }
}
```

<pre class="language-text"><code class="language-text">style/useCollapsedElseIf.js:3:9 <a href="https://biomejs.dev/linter/rules/use-collapsed-else-if">lint/style/useCollapsedElseIf</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>if</strong></span><span style="color: Orange;"> statement can be collapsed into an </span><span style="color: Orange;"><strong>else if</strong></span><span style="color: Orange;"> statement.</span>
  
    <strong>1 │ </strong>if (condition) {
    <strong>2 │ </strong>    // ...
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>} else {
   <strong>   │ </strong>        
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    if (anotherCondition) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>        // ...
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>    }
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>}
    <strong>8 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use collapsed </span><span style="color: lightgreen;"><strong>else if</strong></span><span style="color: lightgreen;"> instead.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  if (condition) {
    <strong>2</strong> <strong>2</strong><strong> │ </strong>      // ...
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">}</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">l</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>{</strong></span>
    <strong>4</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;">i</span><span style="color: Tomato;">f</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">o</span><span style="color: Tomato;">t</span><span style="color: Tomato;">h</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">C</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">t</span><span style="color: Tomato;">i</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span>
      <strong>3</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">h</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">C</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span>
    <strong>5</strong> <strong>4</strong><strong> │ </strong>          // ...
    <strong>6</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>}</strong></span>
    <strong>7</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">}</span>
      <strong>5</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span>
    <strong>8</strong> <strong>6</strong><strong> │ </strong>  
  
</code></pre>

```jsx
if (condition) {
    // ...
} else {
    if (anotherCondition) {
        // ...
    } else {
        // ...
    }
}
```

<pre class="language-text"><code class="language-text">style/useCollapsedElseIf.js:3:9 <a href="https://biomejs.dev/linter/rules/use-collapsed-else-if">lint/style/useCollapsedElseIf</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>if</strong></span><span style="color: Orange;"> statement can be collapsed into an </span><span style="color: Orange;"><strong>else if</strong></span><span style="color: Orange;"> statement.</span>
  
     <strong>1 │ </strong>if (condition) {
     <strong>2 │ </strong>    // ...
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>} else {
    <strong>   │ </strong>        
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    if (anotherCondition) {
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>        // ...
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>    } else {
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>7 │ </strong>        // ...
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>8 │ </strong>    }
    <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
     <strong>9 │ </strong>}
    <strong>10 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use collapsed </span><span style="color: lightgreen;"><strong>else if</strong></span><span style="color: lightgreen;"> instead.</span>
  
    <strong> 1</strong> <strong>1</strong><strong> │ </strong>  if (condition) {
    <strong> 2</strong> <strong>2</strong><strong> │ </strong>      // ...
    <strong> 3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">}</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">l</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>{</strong></span>
    <strong> 4</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;">i</span><span style="color: Tomato;">f</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">o</span><span style="color: Tomato;">t</span><span style="color: Tomato;">h</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">C</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">t</span><span style="color: Tomato;">i</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span>
       <strong>3</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">h</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">C</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span>
    <strong> 5</strong> <strong>4</strong><strong> │ </strong>          // ...
    <strong> 6</strong> <strong>5</strong><strong> │ </strong>      } else {
    <strong> 7</strong> <strong>6</strong><strong> │ </strong>          // ...
    <strong> 8</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>}</strong></span>
    <strong> 9</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">}</span>
       <strong>7</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span>
    <strong>10</strong> <strong>8</strong><strong> │ </strong>  
  
</code></pre>

```jsx
if (condition) {
    // ...
} else {
    // Comment
    if (anotherCondition) {
        // ...
    }
}
```

<pre class="language-text"><code class="language-text">style/useCollapsedElseIf.js:3:9 <a href="https://biomejs.dev/linter/rules/use-collapsed-else-if">lint/style/useCollapsedElseIf</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>if</strong></span><span style="color: Orange;"> statement can be collapsed into an </span><span style="color: Orange;"><strong>else if</strong></span><span style="color: Orange;"> statement.</span>
  
    <strong>1 │ </strong>if (condition) {
    <strong>2 │ </strong>    // ...
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>} else {
   <strong>   │ </strong>        
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    // Comment
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>    if (anotherCondition) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>        // ...
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>7 │ </strong>    }
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>8 │ </strong>}
    <strong>9 │ </strong>
  
</code></pre>

### Valid

```jsx
if (condition) {
    // ...
} else if (anotherCondition) {
    // ...
}
```

```jsx
if (condition) {
    // ...
} else if (anotherCondition) {
    // ...
} else {
    // ...
}
```

```jsx
if (condition) {
    // ...
} else {
    if (anotherCondition) {
        // ...
    }
    doSomething();
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
