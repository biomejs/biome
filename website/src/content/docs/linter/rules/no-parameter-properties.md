---
title: noParameterProperties (since v1.0.0)
---


Disallow the use of parameter properties in class constructors.

TypeScript includes a "parameter properties" shorthand for declaring a class constructor parameter and class property in one location.
Parameter properties can confuse those new to TypeScript as they are less explicit than other ways of declaring and initializing class members.
Moreover, private class properties, starting with `#`, cannot be turned into "parameter properties".
This questions the future of this feature.

Source: https://typescript-eslint.io/rules/parameter-properties

## Examples

### Invalid

```ts
class A {
    constructor(readonly name: string) {}
}
```

<pre class="language-text"><code class="language-text">style/noParameterProperties.js:2:17 <a href="https://biomejs.dev/linter/rules/no-parameter-properties">lint/style/noParameterProperties</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Use a more explicit </span><span style="color: Orange;"><strong>class property</strong></span><span style="color: Orange;"> instead of a </span><span style="color: Orange;"><strong>parameter property</strong></span><span style="color: Orange;">.</span>
  
    <strong>1 │ </strong>class A {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    constructor(readonly name: string) {}
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);"><strong>Parameter properties</strong></span><span style="color: rgb(38, 148, 255);"> are less explicit than other ways of declaring and initializing </span><span style="color: rgb(38, 148, 255);"><strong>class properties</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
</code></pre>

### Valid

```ts
class A {
    constructor(name: string) {}
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
