---
title: noClassAssign (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noClassAssign`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-class-assign" target="_blank"><code>no-class-assign</code></a>

Disallow reassigning class members.

A class declaration creates a variable that we can modify, however, the modification is a mistake in most cases.

## Examples

### Invalid

```jsx
class A {}
A = 0;
```

<pre class="language-text"><code class="language-text">suspicious/noClassAssign.js:2:1 <a href="https://biomejs.dev/linter/rules/no-class-assign">lint/suspicious/noClassAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">'A' is a class.</span>
  
    <strong>1 │ </strong>class A {}
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>A = 0;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'A' is defined here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>class A {}
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>A = 0;
    <strong>3 │ </strong>
  
</code></pre>

```jsx
A = 0;
class A {}
```

<pre class="language-text"><code class="language-text">suspicious/noClassAssign.js:1:1 <a href="https://biomejs.dev/linter/rules/no-class-assign">lint/suspicious/noClassAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">'A' is a class.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>A = 0;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>class A {}
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'A' is defined here.</span>
  
    <strong>1 │ </strong>A = 0;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>class A {}
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
</code></pre>

```jsx
class A {
	b() {
		A = 0;
	}
}
```

<pre class="language-text"><code class="language-text">suspicious/noClassAssign.js:3:3 <a href="https://biomejs.dev/linter/rules/no-class-assign">lint/suspicious/noClassAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">'A' is a class.</span>
  
    <strong>1 │ </strong>class A {
    <strong>2 │ </strong>	b() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>		A = 0;
   <strong>   │ </strong>		<strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>	}
    <strong>5 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'A' is defined here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>class A {
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>	b() {
    <strong>3 │ </strong>		A = 0;
  
</code></pre>

```jsx
let A = class A {
	b() {
		A = 0;
		// `let A` is shadowed by the class name.
	}
}
```

<pre class="language-text"><code class="language-text">suspicious/noClassAssign.js:3:3 <a href="https://biomejs.dev/linter/rules/no-class-assign">lint/suspicious/noClassAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">'A' is a class.</span>
  
    <strong>1 │ </strong>let A = class A {
    <strong>2 │ </strong>	b() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>		A = 0;
   <strong>   │ </strong>		<strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>		// `let A` is shadowed by the class name.
    <strong>5 │ </strong>	}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'A' is defined here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let A = class A {
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>	b() {
    <strong>3 │ </strong>		A = 0;
  
</code></pre>

### Valid

```jsx
let A = class A {}
A = 0; // A is a variable.
```

```jsx
let A = class {
    b() {
        A = 0; // A is a variable.
    }
}
```

```jsx
class A {
	b(A) {
		A = 0; // A is a parameter.
	}
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
