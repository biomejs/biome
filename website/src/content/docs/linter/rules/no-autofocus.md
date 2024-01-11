---
title: noAutofocus (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/noAutofocus`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-autofocus.md" target="_blank"><code>no-autofocus</code></a>

Enforce that autoFocus prop is not used on elements.

Autofocusing elements can cause usability issues for sighted and non-sighted users, alike.

## Examples

### Invalid

```jsx
<input autoFocus />
```

<pre class="language-text"><code class="language-text">a11y/noAutofocus.js:1:8 <a href="https://biomejs.dev/linter/rules/no-autofocus">lint/a11y/noAutofocus</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>autoFocus</strong></span><span style="color: Tomato;"> attribute.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;input autoFocus /&gt;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>autoFocus</strong></span><span style="color: lightgreen;"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;input<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">u</span><span style="color: Tomato;">t</span><span style="color: Tomato;">o</span><span style="color: Tomato;">F</span><span style="color: Tomato;">o</span><span style="color: Tomato;">c</span><span style="color: Tomato;">u</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>/&gt;
<strong>  </strong><strong>    │ </strong>       <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>  
</code></pre>

```jsx
<input autoFocus="true" />
```

<pre class="language-text"><code class="language-text">a11y/noAutofocus.js:1:8 <a href="https://biomejs.dev/linter/rules/no-autofocus">lint/a11y/noAutofocus</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>autoFocus</strong></span><span style="color: Tomato;"> attribute.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;input autoFocus=&quot;true&quot; /&gt;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>autoFocus</strong></span><span style="color: lightgreen;"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;input<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">u</span><span style="color: Tomato;">t</span><span style="color: Tomato;">o</span><span style="color: Tomato;">F</span><span style="color: Tomato;">o</span><span style="color: Tomato;">c</span><span style="color: Tomato;">u</span><span style="color: Tomato;">s</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">t</span><span style="color: Tomato;">r</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span><span style="color: Tomato;">&quot;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>/&gt;
<strong>  </strong><strong>    │ </strong>       <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>  
</code></pre>

```jsx
<input autoFocus={"false"} />
```

<pre class="language-text"><code class="language-text">a11y/noAutofocus.js:1:8 <a href="https://biomejs.dev/linter/rules/no-autofocus">lint/a11y/noAutofocus</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>autoFocus</strong></span><span style="color: Tomato;"> attribute.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;input autoFocus={&quot;false&quot;} /&gt;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>autoFocus</strong></span><span style="color: lightgreen;"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;input<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">u</span><span style="color: Tomato;">t</span><span style="color: Tomato;">o</span><span style="color: Tomato;">F</span><span style="color: Tomato;">o</span><span style="color: Tomato;">c</span><span style="color: Tomato;">u</span><span style="color: Tomato;">s</span><span style="color: Tomato;">=</span><span style="color: Tomato;">{</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">f</span><span style="color: Tomato;">a</span><span style="color: Tomato;">l</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">}</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>/&gt;
<strong>  </strong><strong>    │ </strong>       <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>  
</code></pre>

```jsx
<input autoFocus={undefined} />
```

<pre class="language-text"><code class="language-text">a11y/noAutofocus.js:1:8 <a href="https://biomejs.dev/linter/rules/no-autofocus">lint/a11y/noAutofocus</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>autoFocus</strong></span><span style="color: Tomato;"> attribute.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;input autoFocus={undefined} /&gt;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>autoFocus</strong></span><span style="color: lightgreen;"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;input<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">u</span><span style="color: Tomato;">t</span><span style="color: Tomato;">o</span><span style="color: Tomato;">F</span><span style="color: Tomato;">o</span><span style="color: Tomato;">c</span><span style="color: Tomato;">u</span><span style="color: Tomato;">s</span><span style="color: Tomato;">=</span><span style="color: Tomato;">{</span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">f</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">d</span><span style="color: Tomato;">}</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>/&gt;
<strong>  </strong><strong>    │ </strong>       <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>  
</code></pre>

### Valid

```jsx
<input />
```

```jsx
<div />
```

```jsx
<button />
```

```jsx
// `autoFocus` prop in user created component is valid
<MyComponent autoFocus={true} />
```

## Resources

- [WHATWG HTML Standard, The autofocus attribute](https://html.spec.whatwg.org/multipage/interaction.html#attr-fe-autofocus)
- [The accessibility of HTML 5 autofocus](https://brucelawson.co.uk/2009/the-accessibility-of-html-5-autofocus/)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
