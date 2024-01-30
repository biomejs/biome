---
title: noUselessFragments (since v1.0.0)
---

**Diagnostic Category: `lint/complexity/noUselessFragments`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/jsx-no-useless-fragment.md" target="_blank"><code>jsx-no-useless-fragment</code></a>

Disallow unnecessary fragments

## Examples

### Invalid

```jsx
<>
foo
</>
```

<pre class="language-text"><code class="language-text">complexity/noUselessFragments.js:1:1 <a href="https://biomejs.dev/linter/rules/no-useless-fragments">lint/complexity/noUselessFragments</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid using unnecessary </span><span style="color: Tomato;"><strong>Fragment</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>foo
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>&lt;/&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">A fragment is redundant if it contains only one child, or if it is the child of a html element, and is not a keyed </span><span style="color: lightgreen;"><a href="https://legacy.reactjs.org/docs/fragments.html#keyed-fragments">fragment</a></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the Fragment</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;</strong></span><span style="color: Tomato;"><strong>&gt;</strong></span>
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span>
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;</strong></span><span style="color: Tomato;"><strong>/</strong></span><span style="color: Tomato;"><strong>&gt;</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span>
    <strong>4</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
<React.Fragment>
foo
</React.Fragment>
```

<pre class="language-text"><code class="language-text">complexity/noUselessFragments.js:1:1 <a href="https://biomejs.dev/linter/rules/no-useless-fragments">lint/complexity/noUselessFragments</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid using unnecessary </span><span style="color: Tomato;"><strong>Fragment</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;React.Fragment&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>foo
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>&lt;/React.Fragment&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">A fragment is redundant if it contains only one child, or if it is the child of a html element, and is not a keyed </span><span style="color: lightgreen;"><a href="https://legacy.reactjs.org/docs/fragments.html#keyed-fragments">fragment</a></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the Fragment</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;</strong></span><span style="color: Tomato;"><strong>R</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>F</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>g</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>&gt;</strong></span>
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span>
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;</strong></span><span style="color: Tomato;"><strong>/</strong></span><span style="color: Tomato;"><strong>R</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>F</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>g</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>&gt;</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span>
    <strong>4</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
<>
    <>foo</>
    <SomeComponent />
</>
```

<pre class="language-text"><code class="language-text">complexity/noUselessFragments.js:2:5 <a href="https://biomejs.dev/linter/rules/no-useless-fragments">lint/complexity/noUselessFragments</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid using unnecessary </span><span style="color: Tomato;"><strong>Fragment</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>&lt;&gt;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    &lt;&gt;foo&lt;/&gt;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    &lt;SomeComponent /&gt;
    <strong>4 │ </strong>&lt;/&gt;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">A fragment is redundant if it contains only one child, or if it is the child of a html element, and is not a keyed </span><span style="color: lightgreen;"><a href="https://legacy.reactjs.org/docs/fragments.html#keyed-fragments">fragment</a></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the Fragment</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">&gt;</span>foo<span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span>
<strong>  </strong><strong>    │ </strong>    <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>   <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
</code></pre>

```jsx
<></>
```

<pre class="language-text"><code class="language-text">complexity/noUselessFragments.js:1:1 <a href="https://biomejs.dev/linter/rules/no-useless-fragments">lint/complexity/noUselessFragments</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid using unnecessary </span><span style="color: Tomato;"><strong>Fragment</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;&gt;&lt;/&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">A fragment is redundant if it contains only one child, or if it is the child of a html element, and is not a keyed </span><span style="color: lightgreen;"><a href="https://legacy.reactjs.org/docs/fragments.html#keyed-fragments">fragment</a></span><span style="color: lightgreen;">.</span>
  
</code></pre>

### Valid

```jsx
<>
    <Foo />
    <Bar />
</>
```

```jsx
<>foo {bar}</>
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
