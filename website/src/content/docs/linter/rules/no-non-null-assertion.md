---
title: noNonNullAssertion (since v1.0.0)
---

**Diagnostic Category: `lint/style/noNonNullAssertion`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://typescript-eslint.io/rules/no-non-null-assertion" target="_blank"><code>no-non-null-assertion</code></a>

Disallow non-null assertions using the `!` postfix operator.

TypeScript's `!` non-null assertion operator asserts to the type system that an expression is non-nullable, as
in not `null` or `undefined`. Using assertions to tell the type system new information is often a sign that
code is not fully type-safe. It's generally better to structure program logic so that TypeScript understands
when values may be nullable.

## Examples

### Invalid

```ts
interface Example {
  property?: string;
}
declare const example: Example;
const includesBaz = foo.property!.includes('baz');
```

<pre class="language-text"><code class="language-text">style/noNonNullAssertion.js:5:21 <a href="https://biomejs.dev/linter/rules/no-non-null-assertion">lint/style/noNonNullAssertion</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Forbidden non-null assertion.</span>
  
    <strong>3 │ </strong>}
    <strong>4 │ </strong>declare const example: Example;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>const includesBaz = foo.property!.includes('baz');
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Replace with optional chain operator </span><span style="color: lightgreen;"><strong>?.</strong></span><span style="color: lightgreen;"> This operator includes runtime checks, so it is safer than the compile-only non-null assertion operator</span>
  
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  }
    <strong>4</strong> <strong>4</strong><strong> │ </strong>  declare const example: Example;
    <strong>5</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">c</span><span style="color: Tomato;">l</span><span style="color: Tomato;">u</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">s</span><span style="color: Tomato;">B</span><span style="color: Tomato;">a</span><span style="color: Tomato;">z</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">.</span><span style="color: Tomato;">p</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">p</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="color: Tomato;">y</span><span style="color: Tomato;"><strong>!</strong></span><span style="color: Tomato;">.</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">c</span><span style="color: Tomato;">l</span><span style="color: Tomato;">u</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">s</span><span style="color: Tomato;">(</span><span style="color: Tomato;">'</span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">z</span><span style="color: Tomato;">'</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
      <strong>5</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">B</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">z</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">y</span><span style="color: MediumSeaGreen;"><strong>?</strong></span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">'</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">z</span><span style="color: MediumSeaGreen;">'</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
    <strong>6</strong> <strong>6</strong><strong> │ </strong>  
  
</code></pre>

```ts
(b!! as number) = "test";
```

<pre class="language-text"><code class="language-text">style/noNonNullAssertion.js:1:2 <a href="https://biomejs.dev/linter/rules/no-non-null-assertion">lint/style/noNonNullAssertion</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Forbidden non-null assertion.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>(b!! as number) = &quot;test&quot;;
   <strong>   │ </strong> <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

### Valid

```ts
interface Example {
  property?: string;
}

declare const example: Example;
const includesBaz = foo.property?.includes('baz') ?? false;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
