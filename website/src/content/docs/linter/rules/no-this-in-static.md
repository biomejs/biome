---
title: noThisInStatic (since vnext)
---

**Diagnostic Category: `lint/nursery/noThisInStatic`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow `this`/`super` in static methods

In JavaScript, the `this` keyword within static methods refers to the class (the constructor) instance,
not an instance of the class. This can be confusing for developers coming from other languages where
`this` typically refers to an instance of the class, not the class itself.

Similarly, `super` in static methods also refers to the parent class, not an instance of the parent class.
This can lead to unexpected behavior if not properly understood.

This rule enforces the use of the class name itself to access static methods,
which can make the code clearer and less prone to errors. It helps to prevent
misunderstandings and bugs that can arise from the unique behavior of `this` and `super` in static methods.

Source: https://github.com/mysticatea/eslint-plugin/blob/master/docs/rules/no-this-in-static.md

## Example

### Invalid

```jsx

 class A {
    static foo() {
        doSomething()
    }

    static bar() {
        this.foo()
    }
 }
```

<pre class="language-text"><code class="language-text">nursery/noThisInStatic.js:8:9 <a href="https://biomejs.dev/lint/rules/no-this-in-static">lint/nursery/noThisInStatic</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Unexpected </span><span style="color: Orange;"><strong>this</strong></span><span style="color: Orange;">.</span>
  
     <strong>7 │ </strong>    static bar() {
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>8 │ </strong>        this.foo()
    <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
     <strong>9 │ </strong>    }
    <strong>10 │ </strong> }
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Function </span><span style="color: lightgreen;"><strong>this.foo()</strong></span><span style="color: lightgreen;"> is static, so `</span><span style="color: lightgreen;"><strong>this.</strong></span><span style="color: lightgreen;">` refers to the class (the constructor) instance.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Instead of </span><span style="color: lightgreen;"><strong>this.foo()</strong></span><span style="color: lightgreen;"> use </span><span style="color: lightgreen;"><strong>A.foo()</strong></span><span style="color: lightgreen;">.</span>
  
</code></pre>

```jsx
 class A {
    static foo() {
        doSomething()
    }
 }

 class B extends A {
    static foo() {
        super.foo()
    }
 }
```

<pre class="language-text"><code class="language-text">nursery/noThisInStatic.js:9:9 <a href="https://biomejs.dev/lint/rules/no-this-in-static">lint/nursery/noThisInStatic</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Unexpected </span><span style="color: Orange;"><strong>super</strong></span><span style="color: Orange;">.</span>
  
     <strong>7 │ </strong> class B extends A {
     <strong>8 │ </strong>    static foo() {
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>9 │ </strong>        super.foo()
    <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>10 │ </strong>    }
    <strong>11 │ </strong> }
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Function </span><span style="color: lightgreen;"><strong>super.foo()</strong></span><span style="color: lightgreen;"> is static, so `</span><span style="color: lightgreen;"><strong>super.</strong></span><span style="color: lightgreen;">` refers to the class (the constructor) instance.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Instead of </span><span style="color: lightgreen;"><strong>super.foo()</strong></span><span style="color: lightgreen;"> use </span><span style="color: lightgreen;"><strong>A.foo()</strong></span><span style="color: lightgreen;">.</span>
  
</code></pre>

### Valid

```jsx
class A {
    static foo() {
        doSomething()
    }
}

class B extends A {
    static foo() {
        A.foo()
    }
}
```

```jsx
class A {
   static foo() {
       doSomething()
   }

   bar() {
     A.foo()
   }
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
