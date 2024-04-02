---
title: noUselessSwitchCase (since v1.0.0)
---

**Diagnostic Category: `lint/complexity/noUselessSwitchCase`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/no-useless-switch-case.md" target="_blank"><code>no-useless-switch-case</code></a>

Disallow useless `case` in `switch` statements.

A `switch` statement can optionally have a `default` clause.

The `default` clause will be still executed only if there is no match in the `case` clauses.
An empty `case` clause that precedes the `default` clause is thus useless.

## Examples

### Invalid

```jsx
switch (foo) {
    case 0:
    default:
        break;
    case 1:
        break;
}
```

<pre class="language-text"><code class="language-text">complexity/noUselessSwitchCase.js:2:5 <a href="https://biomejs.dev/linter/rules/no-useless-switch-case">lint/complexity/noUselessSwitchCase</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Useless </span><span style="color: Tomato;"><strong>case clause</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>switch (foo) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    case 0:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    default:
    <strong>4 │ </strong>        break;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">because the </span><span style="color: lightgreen;"><strong>default clause</strong></span><span style="color: lightgreen;"> is present:</span>
  
    <strong>1 │ </strong>switch (foo) {
    <strong>2 │ </strong>    case 0:
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    default:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>        break;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>    case 1:
    <strong>6 │ </strong>        break;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the useless </span><span style="color: lightgreen;"><strong>case</strong></span><span style="color: lightgreen;">.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  switch (foo) {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;"><strong>:</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>      default:
    <strong>4</strong> <strong>3</strong><strong> │ </strong>          break;
  
</code></pre>

```jsx
switch (foo) {
    default:
    case 0:
        break;
    case 1:
        break;
}
```

<pre class="language-text"><code class="language-text">complexity/noUselessSwitchCase.js:3:5 <a href="https://biomejs.dev/linter/rules/no-useless-switch-case">lint/complexity/noUselessSwitchCase</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Useless </span><span style="color: Tomato;"><strong>case clause</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>switch (foo) {
    <strong>2 │ </strong>    default:
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    case 0:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>        break;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>    case 1:
    <strong>6 │ </strong>        break;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">because the </span><span style="color: lightgreen;"><strong>default clause</strong></span><span style="color: lightgreen;"> is present:</span>
  
    <strong>1 │ </strong>switch (foo) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    default:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    case 0:
    <strong>4 │ </strong>        break;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the useless </span><span style="color: lightgreen;"><strong>case</strong></span><span style="color: lightgreen;">.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  switch (foo) {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>:</strong></span>
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;">:</span>
      <strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>d</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>f</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>u</strong></span><span style="color: MediumSeaGreen;"><strong>l</strong></span><span style="color: MediumSeaGreen;"><strong>t</strong></span><span style="color: MediumSeaGreen;">:</span>
    <strong>4</strong> <strong>3</strong><strong> │ </strong>          break;
    <strong>5</strong> <strong>4</strong><strong> │ </strong>      case 1:
  
</code></pre>

### Valid

```jsx
switch (foo) {
    case 0:
        break;
    default:
        break;
}
```

```jsx
switch (foo) {
    case 0:
        break;
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
