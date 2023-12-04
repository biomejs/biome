---
title: noUnnecessaryContinue (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noUnnecessaryContinue`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Avoid using unnecessary `continue`.

## Examples

### Invalid

```jsx
loop: for (let i = 0; i < 5; i++) {
  continue loop;
}
```

<pre class="language-text"><code class="language-text">correctness/noUnnecessaryContinue.js:2:3 <a href="https://biomejs.dev/linter/rules/no-unnecessary-continue">lint/correctness/noUnnecessaryContinue</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary continue statement</span>
  
    <strong>1 │ </strong>loop: for (let i = 0; i &lt; 5; i++) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  continue loop;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Delete the unnecessary continue statement</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  loop: for (let i = 0; i &lt; 5; i++) {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>  }
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

```jsx
while (i--) {
  continue;
}
```

<pre class="language-text"><code class="language-text">correctness/noUnnecessaryContinue.js:2:3 <a href="https://biomejs.dev/linter/rules/no-unnecessary-continue">lint/correctness/noUnnecessaryContinue</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary continue statement</span>
  
    <strong>1 │ </strong>while (i--) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  continue;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Delete the unnecessary continue statement</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  while (i--) {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>  }
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

```jsx
while (1) {
  continue;
}
```

<pre class="language-text"><code class="language-text">correctness/noUnnecessaryContinue.js:2:3 <a href="https://biomejs.dev/linter/rules/no-unnecessary-continue">lint/correctness/noUnnecessaryContinue</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary continue statement</span>
  
    <strong>1 │ </strong>while (1) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  continue;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Delete the unnecessary continue statement</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  while (1) {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>  }
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

```jsx
for (let i = 0; i < 10; i++) {
  if (i > 5) {
    console.log("foo");
    continue;
  } else if (i >= 5 && i < 8) {
    console.log("test");
  } else {
    console.log("test");
  }
}
```

<pre class="language-text"><code class="language-text">correctness/noUnnecessaryContinue.js:4:5 <a href="https://biomejs.dev/linter/rules/no-unnecessary-continue">lint/correctness/noUnnecessaryContinue</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary continue statement</span>
  
    <strong>2 │ </strong>  if (i &gt; 5) {
    <strong>3 │ </strong>    console.log(&quot;foo&quot;);
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    continue;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>  } else if (i &gt;= 5 &amp;&amp; i &lt; 8) {
    <strong>6 │ </strong>    console.log(&quot;test&quot;);
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Delete the unnecessary continue statement</span>
  
    <strong> 2</strong> <strong> 2</strong><strong> │ </strong>    if (i &gt; 5) {
    <strong> 3</strong> <strong> 3</strong><strong> │ </strong>      console.log(&quot;foo&quot;);
    <strong> 4</strong>   <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong> 5</strong> <strong> 4</strong><strong> │ </strong>    } else if (i &gt;= 5 &amp;&amp; i &lt; 8) {
    <strong> 6</strong> <strong> 5</strong><strong> │ </strong>      console.log(&quot;test&quot;);
  
</code></pre>

```jsx
for (let i = 0; i < 9; i++) {
  continue;
}
```

<pre class="language-text"><code class="language-text">correctness/noUnnecessaryContinue.js:2:3 <a href="https://biomejs.dev/linter/rules/no-unnecessary-continue">lint/correctness/noUnnecessaryContinue</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary continue statement</span>
  
    <strong>1 │ </strong>for (let i = 0; i &lt; 9; i++) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  continue;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Delete the unnecessary continue statement</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  for (let i = 0; i &lt; 9; i++) {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>  }
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

```jsx
test2: do {
	continue test2;
} while (true);
```

<pre class="language-text"><code class="language-text">correctness/noUnnecessaryContinue.js:2:2 <a href="https://biomejs.dev/linter/rules/no-unnecessary-continue">lint/correctness/noUnnecessaryContinue</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary continue statement</span>
  
    <strong>1 │ </strong>test2: do {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>	continue test2;
   <strong>   │ </strong>	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>} while (true);
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Delete the unnecessary continue statement</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  test2: do {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>→ </strong></span></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>2</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>  } while (true);
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
while (i) {
  if (i > 5) {
    continue;
  }
  console.log(i);
  i--;
}

loop: while (1) {
  forLoop: for (let i = 0; i < 5; i++) {
    if (someCondition) {
      continue loop;
    }
  }
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
