---
title: noUnusedVariables (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noUnusedVariables`**

Source: <a href="https://eslint.org/docs/latest/rules/no-unused-vars" target="_blank"><code>no-unused-vars</code></a>

Disallow unused variables.

There is an exception to this rule:
variables that starts with underscore, e.g. `let _something;`.

The pattern of having an underscore as prefix of a name of variable is a very diffuse
pattern among programmers, and Biome decided to follow it.

This rule won't report unused imports.
If you want to report unused imports,
enable [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/).

## Examples

### Invalid

```jsx
let a = 4;
a++;
```

<pre class="language-text"><code class="language-text">correctness/noUnusedVariables.js:1:5 <a href="https://biomejs.dev/linter/rules/no-unused-variables">lint/correctness/noUnusedVariables</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This variable is unused.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let a = 4;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>a++;
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">If this is intentional, prepend </span><span style="color: lightgreen;"><strong>a</strong></span><span style="color: lightgreen;"> with an underscore.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">4</span><span style="color: Tomato;">;</span>
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;">+</span><span style="color: Tomato;">+</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>_</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">4</span><span style="color: MediumSeaGreen;">;</span>
      <strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>_</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">;</span>
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

```jsx
function foo() {}
```

<pre class="language-text"><code class="language-text">correctness/noUnusedVariables.js:1:10 <a href="https://biomejs.dev/linter/rules/no-unused-variables">lint/correctness/noUnusedVariables</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This function is unused.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function foo() {}
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
</code></pre>

```jsx
export function foo(myVar) {
    console.log('foo');
}
```

<pre class="language-text"><code class="language-text">correctness/noUnusedVariables.js:1:21 <a href="https://biomejs.dev/linter/rules/no-unused-variables">lint/correctness/noUnusedVariables</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This parameter is unused.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>export function foo(myVar) {
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    console.log('foo');
    <strong>3 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">If this is intentional, prepend </span><span style="color: lightgreen;"><strong>myVar</strong></span><span style="color: lightgreen;"> with an underscore.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">i</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>y</strong></span><span style="color: Tomato;"><strong>V</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;"><strong>_</strong></span><span style="color: MediumSeaGreen;"><strong>m</strong></span><span style="color: MediumSeaGreen;"><strong>y</strong></span><span style="color: MediumSeaGreen;"><strong>V</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>      console.log('foo');
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  }
  
</code></pre>

```jsx
function foo() {
    foo();
}
```

<pre class="language-text"><code class="language-text">correctness/noUnusedVariables.js:1:10 <a href="https://biomejs.dev/linter/rules/no-unused-variables">lint/correctness/noUnusedVariables</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This function is unused.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function foo() {
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    foo();
    <strong>3 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
</code></pre>

```jsx
const foo = () => {
    foo();
};
```

<pre class="language-text"><code class="language-text">correctness/noUnusedVariables.js:1:7 <a href="https://biomejs.dev/linter/rules/no-unused-variables">lint/correctness/noUnusedVariables</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This variable is unused.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const foo = () =&gt; {
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    foo();
    <strong>3 │ </strong>};
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">If this is intentional, prepend </span><span style="color: lightgreen;"><strong>foo</strong></span><span style="color: lightgreen;"> with an underscore.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span>
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;">(</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>_</strong></span><span style="color: MediumSeaGreen;"><strong>f</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">&gt;</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span>
      <strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>_</strong></span><span style="color: MediumSeaGreen;"><strong>f</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  };
    <strong>4</strong> <strong>4</strong><strong> │ </strong>  
  
</code></pre>

```ts
export function f<T>() {}
```

<pre class="language-text"><code class="language-text">correctness/noUnusedVariables.js:1:19 <a href="https://biomejs.dev/linter/rules/no-unused-variables">lint/correctness/noUnusedVariables</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This type parameter is unused.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>export function f&lt;T&gt;() {}
   <strong>   │ </strong>                  <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">If this is intentional, prepend </span><span style="color: lightgreen;"><strong>T</strong></span><span style="color: lightgreen;"> with an underscore.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">i</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;"><strong>T</strong></span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">(</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span><span style="color: Tomato;">}</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">&lt;</span><span style="color: MediumSeaGreen;"><strong>_</strong></span><span style="color: MediumSeaGreen;"><strong>T</strong></span><span style="color: MediumSeaGreen;">&gt;</span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;">}</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

# Valid

```jsx
function foo(b) {
    console.log(b)
};
foo();
```

```jsx
export function foo(_unused) {}
```

```ts
function used_overloaded(): number;
function used_overloaded(s: string): string;
function used_overloaded(s?: string) {
    return s;
}
used_overloaded();
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
