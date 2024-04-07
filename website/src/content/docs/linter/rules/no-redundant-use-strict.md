---
title: noRedundantUseStrict (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noRedundantUseStrict`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Prevents from having redundant `"use strict"`.

Note that the leading trivia, e.g., comments or newlines preceding
the redundant `"use strict"` will also be removed. So that comment
directives won't be transferred to a wrong place.

## Examples

### Invalid

```js
"use strict";
function foo() {
 	"use strict";
}
```

<pre class="language-text"><code class="language-text">suspicious/noRedundantUseStrict.js:3:3 <a href="https://biomejs.dev/linter/rules/no-redundant-use-strict">lint/suspicious/noRedundantUseStrict</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Redundant </span><span style="color: Tomato;"><strong>use strict</strong></span><span style="color: Tomato;"> directive.</span>
  
    <strong>1 │ </strong>&quot;use strict&quot;;
    <strong>2 │ </strong>function foo() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong> 	&quot;use strict&quot;;
   <strong>   │ </strong> 	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This outer </span><span style="color: lightgreen;"><strong>use strict</strong></span><span style="color: lightgreen;"> directive already enables strict mode.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&quot;use strict&quot;;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>function foo() {
    <strong>3 │ </strong> 	&quot;use strict&quot;;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the redundant </span><span style="color: lightgreen;"><strong>use strict</strong></span><span style="color: lightgreen;"> directive.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  &quot;use strict&quot;;
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  function foo() {
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>→ </strong></span></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  }
    <strong>5</strong> <strong>4</strong><strong> │ </strong>  
  
</code></pre>

```js
"use strict";
"use strict";

function foo() {

}
```

<pre class="language-text"><code class="language-text">suspicious/noRedundantUseStrict.js:2:1 <a href="https://biomejs.dev/linter/rules/no-redundant-use-strict">lint/suspicious/noRedundantUseStrict</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Redundant </span><span style="color: Tomato;"><strong>use strict</strong></span><span style="color: Tomato;"> directive.</span>
  
    <strong>1 │ </strong>&quot;use strict&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>&quot;use strict&quot;;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
    <strong>4 │ </strong>function foo() {
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This outer </span><span style="color: lightgreen;"><strong>use strict</strong></span><span style="color: lightgreen;"> directive already enables strict mode.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&quot;use strict&quot;;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>&quot;use strict&quot;;
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the redundant </span><span style="color: lightgreen;"><strong>use strict</strong></span><span style="color: lightgreen;"> directive.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  &quot;use strict&quot;;
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>  
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  function foo() {
  
</code></pre>

```js
function foo() {
"use strict";
"use strict";
}
```

<pre class="language-text"><code class="language-text">suspicious/noRedundantUseStrict.js:3:1 <a href="https://biomejs.dev/linter/rules/no-redundant-use-strict">lint/suspicious/noRedundantUseStrict</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Redundant </span><span style="color: Tomato;"><strong>use strict</strong></span><span style="color: Tomato;"> directive.</span>
  
    <strong>1 │ </strong>function foo() {
    <strong>2 │ </strong>&quot;use strict&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>&quot;use strict&quot;;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This outer </span><span style="color: lightgreen;"><strong>use strict</strong></span><span style="color: lightgreen;"> directive already enables strict mode.</span>
  
    <strong>1 │ </strong>function foo() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>&quot;use strict&quot;;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>&quot;use strict&quot;;
    <strong>4 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the redundant </span><span style="color: lightgreen;"><strong>use strict</strong></span><span style="color: lightgreen;"> directive.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  function foo() {
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  &quot;use strict&quot;;
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  }
    <strong>5</strong> <strong>4</strong><strong> │ </strong>  
  
</code></pre>

```js
class C1 {
	test() {
		"use strict";
	}
}
```

<pre class="language-text"><code class="language-text">suspicious/noRedundantUseStrict.js:3:3 <a href="https://biomejs.dev/linter/rules/no-redundant-use-strict">lint/suspicious/noRedundantUseStrict</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Redundant </span><span style="color: Tomato;"><strong>use strict</strong></span><span style="color: Tomato;"> directive.</span>
  
    <strong>1 │ </strong>class C1 {
    <strong>2 │ </strong>	test() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>		&quot;use strict&quot;;
   <strong>   │ </strong>		<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>	}
    <strong>5 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All parts of a class's body are already in strict mode.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>class C1 {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>	test() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>		&quot;use strict&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>	}
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the redundant </span><span style="color: lightgreen;"><strong>use strict</strong></span><span style="color: lightgreen;"> directive.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  class C1 {
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  	test() {
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>→ </strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>→ </strong></span></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  	}
    <strong>5</strong> <strong>4</strong><strong> │ </strong>  }
  
</code></pre>

```js
const C2 = class {
	test() {
		"use strict";
	}
};

```

<pre class="language-text"><code class="language-text">suspicious/noRedundantUseStrict.js:3:3 <a href="https://biomejs.dev/linter/rules/no-redundant-use-strict">lint/suspicious/noRedundantUseStrict</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Redundant </span><span style="color: Tomato;"><strong>use strict</strong></span><span style="color: Tomato;"> directive.</span>
  
    <strong>1 │ </strong>const C2 = class {
    <strong>2 │ </strong>	test() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>		&quot;use strict&quot;;
   <strong>   │ </strong>		<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>	}
    <strong>5 │ </strong>};
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All parts of a class's body are already in strict mode.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const C2 = class {
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>	test() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>		&quot;use strict&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>	}
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>};
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the redundant </span><span style="color: lightgreen;"><strong>use strict</strong></span><span style="color: lightgreen;"> directive.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  const C2 = class {
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  	test() {
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>→ </strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>→ </strong></span></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  	}
    <strong>5</strong> <strong>4</strong><strong> │ </strong>  };
  
</code></pre>

### Valid

```js
function foo() {

}
```

```js
 function foo() {
    "use strict";
}
function bar() {
    "use strict";
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
