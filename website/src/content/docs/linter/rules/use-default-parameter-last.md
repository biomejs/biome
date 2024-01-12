---
title: useDefaultParameterLast (since v1.0.0)
---

**Diagnostic Category: `lint/style/useDefaultParameterLast`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/default-param-last" target="_blank"><code>default-param-last</code></a>

Enforce default function parameters and optional function parameters to be last.

Default and optional parameters that precede a required parameter cannot be omitted at call site.

## Examples

### Invalid

```jsx
function f(a = 0, b) {}
```

<pre class="language-text"><code class="language-text">style/useDefaultParameterLast.js:1:12 <a href="https://biomejs.dev/linter/rules/use-default-parameter-last">lint/style/useDefaultParameterLast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>default parameter</strong></span><span style="color: Tomato;"> should follow the last </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;"> or should be a </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(a = 0, b) {}
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The last </span><span style="color: lightgreen;"><strong>required parameter</strong></span><span style="color: lightgreen;"> is here:</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(a = 0, b) {}
   <strong>   │ </strong>                  <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">A </span><span style="color: lightgreen;"><strong>default parameter</strong></span><span style="color: lightgreen;"> that precedes a </span><span style="color: lightgreen;"><strong>required parameter</strong></span><span style="color: lightgreen;"> cannot be omitted at call site.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Turn the parameter into a </span><span style="color: lightgreen;"><strong>required parameter</strong></span><span style="color: lightgreen;">.</span>
  
<strong>  </strong><strong>  1 │ </strong>function<span style="opacity: 0.8;">·</span>f(a<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">=</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">0</span>,<span style="opacity: 0.8;">·</span>b)<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>            <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>       
</code></pre>

```jsx
function f(a, b = 0, c) {}
```

<pre class="language-text"><code class="language-text">style/useDefaultParameterLast.js:1:15 <a href="https://biomejs.dev/linter/rules/use-default-parameter-last">lint/style/useDefaultParameterLast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>default parameter</strong></span><span style="color: Tomato;"> should follow the last </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;"> or should be a </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(a, b = 0, c) {}
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The last </span><span style="color: lightgreen;"><strong>required parameter</strong></span><span style="color: lightgreen;"> is here:</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(a, b = 0, c) {}
   <strong>   │ </strong>                     <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">A </span><span style="color: lightgreen;"><strong>default parameter</strong></span><span style="color: lightgreen;"> that precedes a </span><span style="color: lightgreen;"><strong>required parameter</strong></span><span style="color: lightgreen;"> cannot be omitted at call site.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Turn the parameter into a </span><span style="color: lightgreen;"><strong>required parameter</strong></span><span style="color: lightgreen;">.</span>
  
<strong>  </strong><strong>  1 │ </strong>function<span style="opacity: 0.8;">·</span>f(a,<span style="opacity: 0.8;">·</span>b<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">=</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">0</span>,<span style="opacity: 0.8;">·</span>c)<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>               <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>       
</code></pre>

```ts
function f(a: number, b?: number, c: number) {}
```

<pre class="language-text"><code class="language-text">style/useDefaultParameterLast.js:1:23 <a href="https://biomejs.dev/linter/rules/use-default-parameter-last">lint/style/useDefaultParameterLast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>optional parameter</strong></span><span style="color: Tomato;"> should follow the last </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;"> or should be a </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(a: number, b?: number, c: number) {}
   <strong>   │ </strong>                      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The last </span><span style="color: lightgreen;"><strong>required parameter</strong></span><span style="color: lightgreen;"> is here:</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(a: number, b?: number, c: number) {}
   <strong>   │ </strong>                                  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">A </span><span style="color: lightgreen;"><strong>optional parameter</strong></span><span style="color: lightgreen;"> that precedes a </span><span style="color: lightgreen;"><strong>required parameter</strong></span><span style="color: lightgreen;"> cannot be omitted at call site.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Turn the parameter into a </span><span style="color: lightgreen;"><strong>required parameter</strong></span><span style="color: lightgreen;">.</span>
  
<strong>  </strong><strong>  1 │ </strong>function<span style="opacity: 0.8;">·</span>f(a:<span style="opacity: 0.8;">·</span>number,<span style="opacity: 0.8;">·</span>b<span style="color: Tomato;">?</span>:<span style="opacity: 0.8;">·</span>number,<span style="opacity: 0.8;">·</span>c:<span style="opacity: 0.8;">·</span>number)<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>                       <span style="color: Tomato;">-</span>                       
</code></pre>

```ts
class Foo {
    constructor(readonly a = 10, readonly b: number) {}
}
```

<pre class="language-text"><code class="language-text">style/useDefaultParameterLast.js:2:17 <a href="https://biomejs.dev/linter/rules/use-default-parameter-last">lint/style/useDefaultParameterLast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>default parameter</strong></span><span style="color: Tomato;"> should follow the last </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;"> or should be a </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>class Foo {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    constructor(readonly a = 10, readonly b: number) {}
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The last </span><span style="color: lightgreen;"><strong>required parameter</strong></span><span style="color: lightgreen;"> is here:</span>
  
    <strong>1 │ </strong>class Foo {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    constructor(readonly a = 10, readonly b: number) {}
   <strong>   │ </strong>                                 <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">A </span><span style="color: lightgreen;"><strong>default parameter</strong></span><span style="color: lightgreen;"> that precedes a </span><span style="color: lightgreen;"><strong>required parameter</strong></span><span style="color: lightgreen;"> cannot be omitted at call site.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Turn the parameter into a </span><span style="color: lightgreen;"><strong>required parameter</strong></span><span style="color: lightgreen;">.</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>constructor(readonly<span style="opacity: 0.8;">·</span>a<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">=</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">1</span><span style="color: Tomato;">0</span>,<span style="opacity: 0.8;">·</span>readonly<span style="opacity: 0.8;">·</span>b:<span style="opacity: 0.8;">·</span>number)<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>                          <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>                        
</code></pre>

### Valid

```jsx
function f(a, b = 0) {}
```

```ts
function f(a: number, b?: number, c = 0) {}
```

```ts
function f(a: number, b = 0, c?: number) {}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
