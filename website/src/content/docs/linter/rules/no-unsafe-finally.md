---
title: noUnsafeFinally (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noUnsafeFinally`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-unsafe-finally" target="_blank"><code>no-unsafe-finally</code></a>

Disallow control flow statements in finally blocks.

JavaScript suspends the control flow statements of `try` and `catch` blocks until
the execution of finally block finishes. So, when `return`, `throw`, `break` or `continue`
is used in finally, control flow statements inside `try` and `catch` are overwritten,
which is considered as unexpected behavior.

## Examples

### Invalid

```jsx
(() => {
    try {
        return 1; // 1 is returned but suspended until finally block ends
    } catch(err) {
        return 2;
    } finally {
        return 3; // 3 is returned before 1, which we did not expect
    }
})();
```

<pre class="language-text"><code class="language-text">correctness/noUnsafeFinally.js:7:9 <a href="https://biomejs.dev/linter/rules/no-unsafe-finally">lint/correctness/noUnsafeFinally</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unsafe usage of 'return'.</span>
  
    <strong>5 │ </strong>        return 2;
    <strong>6 │ </strong>    } finally {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>7 │ </strong>        return 3; // 3 is returned before 1, which we did not expect
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>8 │ </strong>    }
    <strong>9 │ </strong>})();
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'return' in 'finally' overwrites the control flow statements inside 'try' and 'catch'.</span>
  
</code></pre>

```jsx
(() => {
    try {
        throw new Error("Try"); // error is thrown but suspended until finally block ends
    } finally {
        return 3; // 3 is returned before the error is thrown, which we did not expect
    }
})();
```

<pre class="language-text"><code class="language-text">correctness/noUnsafeFinally.js:5:9 <a href="https://biomejs.dev/linter/rules/no-unsafe-finally">lint/correctness/noUnsafeFinally</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unsafe usage of 'return'.</span>
  
    <strong>3 │ </strong>        throw new Error(&quot;Try&quot;); // error is thrown but suspended until finally block ends
    <strong>4 │ </strong>    } finally {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>        return 3; // 3 is returned before the error is thrown, which we did not expect
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>    }
    <strong>7 │ </strong>})();
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'return' in 'finally' overwrites the control flow statements inside 'try' and 'catch'.</span>
  
</code></pre>

```jsx
(() => {
    try {
        throw new Error("Try")
    } catch(err) {
        throw err; // The error thrown from try block is caught and re-thrown
    } finally {
        throw new Error("Finally"); // Finally(...) is thrown, which we did not expect
    }
})();
```

<pre class="language-text"><code class="language-text">correctness/noUnsafeFinally.js:7:9 <a href="https://biomejs.dev/linter/rules/no-unsafe-finally">lint/correctness/noUnsafeFinally</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unsafe usage of 'throw'.</span>
  
    <strong>5 │ </strong>        throw err; // The error thrown from try block is caught and re-thrown
    <strong>6 │ </strong>    } finally {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>7 │ </strong>        throw new Error(&quot;Finally&quot;); // Finally(...) is thrown, which we did not expect
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>8 │ </strong>    }
    <strong>9 │ </strong>})();
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'throw' in 'finally' overwrites the control flow statements inside 'try' and 'catch'.</span>
  
</code></pre>

```jsx
(() => {
    label: try {
      return 0; // 0 is returned but suspended until finally block ends
    } finally {
      break label; // It breaks out the try-finally block, before 0 is returned.
    }
    return 1;
})();
```

<pre class="language-text"><code class="language-text">correctness/noUnsafeFinally.js:5:7 <a href="https://biomejs.dev/linter/rules/no-unsafe-finally">lint/correctness/noUnsafeFinally</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unsafe usage of 'break'.</span>
  
    <strong>3 │ </strong>      return 0; // 0 is returned but suspended until finally block ends
    <strong>4 │ </strong>    } finally {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>      break label; // It breaks out the try-finally block, before 0 is returned.
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>    }
    <strong>7 │ </strong>    return 1;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'break' in 'finally' overwrites the control flow statements inside 'try' and 'catch'.</span>
  
</code></pre>

```jsx
function a() {
  switch (condition) {
    case 'a': {
      try {
        console.log('a');
        return;
      } finally {
        break;
      }
    }
    case 'b': {
      console.log('b');
    }
  }
}
```

<pre class="language-text"><code class="language-text">correctness/noUnsafeFinally.js:8:9 <a href="https://biomejs.dev/linter/rules/no-unsafe-finally">lint/correctness/noUnsafeFinally</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unsafe usage of 'break'.</span>
  
     <strong>6 │ </strong>        return;
     <strong>7 │ </strong>      } finally {
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>8 │ </strong>        break;
    <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
     <strong>9 │ </strong>      }
    <strong>10 │ </strong>    }
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'break' in 'finally' overwrites the control flow statements inside 'try' and 'catch'.</span>
  
</code></pre>

### Valid

```jsx
let foo = function() {
    try {
        return 1;
    } catch(err) {
        return 2;
    } finally {
        console.log("hola!");
    }
};
```

```jsx
let foo = function() {
    try {
        return 1;
    } catch(err) {
        return 2;
    } finally {
        let a = function() {
            return "hola!";
        }
    }
};
```

```jsx
let foo = function(a) {
    try {
        return 1;
    } catch(err) {
        return 2;
    } finally {
        switch(a) {
            case 1: {
                console.log("hola!")
                break;
            }
        }
    }
};
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
