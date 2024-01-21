---
title: useLiteralEnumMembers (since v1.0.0)
---

**Diagnostic Category: `lint/style/useLiteralEnumMembers`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://typescript-eslint.io/rules/prefer-literal-enum-member" target="_blank"><code>prefer-literal-enum-member</code></a>

Require all enum members to be literal values.

Usually, an enum member is initialized with a literal number or a literal string.
However, _TypeScript_ allows the value of an enum member to be many different kinds of expressions.
Using a computed enum member is often error-prone and confusing.
This rule requires the initialization of enum members with constant expressions.
It allows numeric and bitwise expressions for supporting [enum flags](https://stackoverflow.com/questions/39359740/what-are-enum-flags-in-typescript/39359953#39359953).
It also allows referencing previous enum members.

## Examples

### Invalid

```ts
const x = 2;
enum Computed {
    A,
    B = x,
}
```

<pre class="language-text"><code class="language-text">style/useLiteralEnumMembers.js:4:9 <a href="https://biomejs.dev/linter/rules/use-literal-enum-members">lint/style/useLiteralEnumMembers</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The enum member should be initialized with a literal value such as a number or a string.</span>
  
    <strong>2 │ </strong>enum Computed {
    <strong>3 │ </strong>    A,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    B = x,
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>}
    <strong>6 │ </strong>
  
</code></pre>

### Valid

```ts
enum Direction {
    Left,
    Right,
}
```

```ts
enum Order {
    Less = -1,
    Equal = 0,
    Greater = 1,
}
```

```ts
enum State {
    Open = "Open",
    Close = "Close",
}
```

```ts
enum FileAccess {
    None = 0,
    Read = 1,
    Write = 1 << 1,
    All = Read | Write
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
