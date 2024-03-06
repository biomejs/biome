---
title: noUnusedPrivateClassMembers (since v1.3.3)
---

**Diagnostic Category: `lint/correctness/noUnusedPrivateClassMembers`**

Source: <a href="https://eslint.org/docs/latest/rules/no-unused-private-class-members" target="_blank"><code>no-unused-private-class-members</code></a>

Disallow unused private class members

Private class members that are declared and not used anywhere in the code are most likely an error due to incomplete refactoring.
Such class members take up space in the code and can lead to confusion by readers.

## Examples

### Invalid

```jsx
class OnlyWrite {
  #usedOnlyInWrite = 5;

  method() {
       this.#usedOnlyInWrite = 212;
  }
}
```

<pre class="language-text"><code class="language-text">correctness/noUnusedPrivateClassMembers.js:2:3 <a href="https://biomejs.dev/linter/rules/no-unused-private-class-members">lint/correctness/noUnusedPrivateClassMembers</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This private class member is defined but never used.</span>
  
    <strong>1 │ </strong>class OnlyWrite {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  #usedOnlyInWrite = 5;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
    <strong>4 │ </strong>  method() {
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove unused declaration.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  class OnlyWrite {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>#</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>O</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>y</strong></span><span style="color: Tomato;"><strong>I</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>W</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>=</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>5</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>  
    <strong>4</strong> <strong>3</strong><strong> │ </strong>    method() {
  
</code></pre>

```ts
 class TsBioo {
   private unusedProperty = 5;
 }
```

<pre class="language-text"><code class="language-text">correctness/noUnusedPrivateClassMembers.js:2:12 <a href="https://biomejs.dev/linter/rules/no-unused-private-class-members">lint/correctness/noUnusedPrivateClassMembers</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This private class member is defined but never used.</span>
  
    <strong>1 │ </strong> class TsBioo {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>   private unusedProperty = 5;
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong> }
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove unused declaration.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>   class TsBioo {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>v</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>P</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>y</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>=</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>5</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>   }
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

```ts
 class TsBioo {
   private unusedMethod() {}
 }
```

<pre class="language-text"><code class="language-text">correctness/noUnusedPrivateClassMembers.js:2:12 <a href="https://biomejs.dev/linter/rules/no-unused-private-class-members">lint/correctness/noUnusedPrivateClassMembers</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This private class member is defined but never used.</span>
  
    <strong>1 │ </strong> class TsBioo {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>   private unusedMethod() {}
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong> }
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove unused declaration.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>   class TsBioo {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>v</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>M</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>h</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>(</strong></span><span style="color: Tomato;"><strong>)</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>{</strong></span><span style="color: Tomato;"><strong>}</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>   }
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
class UsedMember {
  #usedMember = 42;

  method() {
       return this.#usedMember;
  }
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
