---
title: Lint Rule noRedundantRoles
parent: lint/rules/index
---

# noRedundantRoles (since v1.0.0)

Enforce explicit `role` property is not the same as implicit/default role property on an element.

ESLint (eslint-plugin-jsx-a11y) Equivalent: [no-redundant-roles](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-redundant-roles.md)

## Examples

### Invalid

```jsx
<article role='article'></article>
```

<pre class="language-text"><code class="language-text">nursery/noRedundantRoles.js:1:15 <a href="https://biomejs.dev/lint/rules/noRedundantRoles">lint/nursery/noRedundantRoles</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Using the role attribute 'article' on the 'article' element is redundant.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;article role='article'&gt;&lt;/article&gt;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the </span><span style="color: rgb(38, 148, 255);"><strong>role</strong></span><span style="color: rgb(38, 148, 255);"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;article<span style="opacity: 0.8;">·</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">=</span><span style="color: Tomato;">'</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="color: Tomato;">i</span><span style="color: Tomato;">c</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">'</span>&gt;&lt;/article&gt;
<strong>  </strong><strong>    │ </strong>         <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>           
</code></pre>

```jsx
<button role='button'></button>
```

<pre class="language-text"><code class="language-text">nursery/noRedundantRoles.js:1:14 <a href="https://biomejs.dev/lint/rules/noRedundantRoles">lint/nursery/noRedundantRoles</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Using the role attribute 'button' on the 'button' element is redundant.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;button role='button'&gt;&lt;/button&gt;
   <strong>   │ </strong>             <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the </span><span style="color: rgb(38, 148, 255);"><strong>role</strong></span><span style="color: rgb(38, 148, 255);"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;button<span style="opacity: 0.8;">·</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">=</span><span style="color: Tomato;">'</span><span style="color: Tomato;">b</span><span style="color: Tomato;">u</span><span style="color: Tomato;">t</span><span style="color: Tomato;">t</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">'</span>&gt;&lt;/button&gt;
<strong>  </strong><strong>    │ </strong>        <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>          
</code></pre>

```jsx
<h1 role='heading' aria-level='1'>title</h1>
```

<pre class="language-text"><code class="language-text">nursery/noRedundantRoles.js:1:10 <a href="https://biomejs.dev/lint/rules/noRedundantRoles">lint/nursery/noRedundantRoles</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Using the role attribute 'heading' on the 'h1' element is redundant.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;h1 role='heading' aria-level='1'&gt;title&lt;/h1&gt;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the </span><span style="color: rgb(38, 148, 255);"><strong>role</strong></span><span style="color: rgb(38, 148, 255);"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;h1<span style="opacity: 0.8;">·</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">=</span><span style="color: Tomato;">'</span><span style="color: Tomato;">h</span><span style="color: Tomato;">e</span><span style="color: Tomato;">a</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">g</span><span style="color: Tomato;">'</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>aria-level='1'&gt;title&lt;/h1&gt;
<strong>  </strong><strong>    │ </strong>    <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>                         
</code></pre>

## Valid

```jsx
<article role='presentation'></article>
```

```jsx
<Button role='button'></Button>
```

```jsx
<span></span>
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
