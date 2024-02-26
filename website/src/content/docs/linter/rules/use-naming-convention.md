---
title: useNamingConvention (since v1.0.0)
---

**Diagnostic Category: `lint/style/useNamingConvention`**

Inspired from: <a href="https://typescript-eslint.io/rules/naming-convention" target="_blank"><code>naming-convention</code></a>

Enforce naming conventions for everything across a codebase.

Enforcing [naming conventions](https://en.wikipedia.org/wiki/Naming_convention_(programming)) helps to keep the codebase consistent,
and reduces overhead when thinking about the name [case](https://en.wikipedia.org/wiki/Naming_convention_(programming)#Examples_of_multiple-word_identifier_formats) of a variable.

## Naming conventions

All names can be prefixed and suffixed by underscores `_` and dollar signs `$`.

### Variable names

All variables, including function parameters and catch parameters, are in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case).

Additionally, top-level variables declared as `const` or `var` may be in [`CONSTANT_CASE`](https://en.wikipedia.org/wiki/Snake_case) or [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).
Top-level variables are declared at module or script level.
Variables declared in a TypeScript `module` or `namespace` are also considered top-level.

```jsx
function f(param, _unusedParam) {
    let localValue = 0;
    try {
        /* ... */
    } catch (customError) {
        /* ... */
    }
}

export const A_CONSTANT = 5;

export const Person = class {}

let aVariable = 0;

export namespace ns {
    export const ANOTHER_CONSTANT = "";
}
```

Examples of incorrect names:

```jsx
let a_value = 0;
```

<pre class="language-text"><code class="language-text">style/useNamingConvention.js:1:5 <a href="https://biomejs.dev/linter/rules/use-naming-convention">lint/style/useNamingConvention</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>top-level let</strong></span><span style="color: Orange;"> name should be in </span><span style="color: Orange;"><strong>camelCase</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let a_value = 0;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The name could be renamed to `aValue`.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Rename this symbol in </span><span style="color: lightgreen;"><strong>camelCase</strong></span><span style="color: lightgreen;">.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>_</strong></span><span style="color: Tomato;"><strong>v</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">0</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>V</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>l</strong></span><span style="color: MediumSeaGreen;"><strong>u</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">0</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
const fooYPosition = 0;
```

<pre class="language-text"><code class="language-text">style/useNamingConvention.js:1:7 <a href="https://biomejs.dev/linter/rules/use-naming-convention">lint/style/useNamingConvention</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Two consecutive uppercase characters are not allowed in camelCase and PascalCase because </span><span style="color: Orange;"><strong>strictCase</strong></span><span style="color: Orange;"> is set to `true`.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const fooYPosition = 0;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If you want to use consecutive uppercase characters in camelCase and PascalCase, then set the </span><span style="color: lightgreen;"><strong>strictCase</strong></span><span style="color: lightgreen;"> option to `false`.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">See the rule </span><span style="color: lightgreen;"><a href="https://biomejs.dev/linter/rules/use-naming-convention#options">options</a></span><span style="color: lightgreen;"> for more details.</span>
  
</code></pre>

```jsx
function f(FirstParam) {}
```

<pre class="language-text"><code class="language-text">style/useNamingConvention.js:1:12 <a href="https://biomejs.dev/linter/rules/use-naming-convention">lint/style/useNamingConvention</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>function parameter</strong></span><span style="color: Orange;"> name should be in </span><span style="color: Orange;"><strong>camelCase</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(FirstParam) {}
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The name could be renamed to `firstParam`.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Rename this symbol in </span><span style="color: lightgreen;"><strong>camelCase</strong></span><span style="color: lightgreen;">.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">f</span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">i</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>F</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>P</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span><span style="color: Tomato;">}</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;"><strong>f</strong></span><span style="color: MediumSeaGreen;"><strong>i</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>t</strong></span><span style="color: MediumSeaGreen;"><strong>P</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>m</strong></span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;">}</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Function names

A `function` name is in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case) or [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).

```jsx
function trimString(s) { /*...*/ }

function Component() {
    return <div></div>;
}
```

### TypeScript `enum` names

A _TypeScript_ `enum` name is in [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).

`enum` members are by default in [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).
However, you can configure the [case](https://en.wikipedia.org/wiki/Naming_convention_(programming)#Examples_of_multiple-word_identifier_formats) of `enum` members.
See [options](#options) for more details.

```ts
enum Status {
    Open,
    Close,
}
```

### Classes

- A class name is in [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).


- Static property and static getter names are in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case) or [`CONSTANT_CASE`](https://en.wikipedia.org/wiki/Snake_case).


- Class property and method names are in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case).



```jsx
class Person {
    static MAX_FRIEND_COUNT = 256;

    static get SPECIAL_PERSON_INSTANCE() { /*...*/ }

    initializedProperty = 0;

    specialMethod() {}
}
```

### TypeScript `type` aliases and `interface`

- A `type` alias or an interface name are in [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).


- Property and method names in a type are in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case).


- `readonly` property and getter names can also be in [`CONSTANT_CASE`](https://en.wikipedia.org/wiki/Snake_case).



```ts
type Named = {
    readonly fullName: string;

    specialMethod(): void;
};

interface Named {
    readonly fullName: string;

    specialMethod(): void;
}

interface PersonConstructor {
    readonly MAX_FRIEND_COUNT: number;

    get SPECIAL_PERSON_INSTANCE(): Person;

    new(): Person;
}
```

Examples of an incorrect type alias:

```ts
type person = { fullName: string };
```

<pre class="language-text"><code class="language-text">style/useNamingConvention.js:1:6 <a href="https://biomejs.dev/linter/rules/use-naming-convention">lint/style/useNamingConvention</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>type alias</strong></span><span style="color: Orange;"> name should be in </span><span style="color: Orange;"><strong>PascalCase</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>type person = { fullName: string };
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The name could be renamed to `Person`.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Rename this symbol in </span><span style="color: lightgreen;"><strong>PascalCase</strong></span><span style="color: lightgreen;">.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">t</span><span style="color: Tomato;">y</span><span style="color: Tomato;">p</span><span style="color: Tomato;">e</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">u</span><span style="color: Tomato;">l</span><span style="color: Tomato;">l</span><span style="color: Tomato;">N</span><span style="color: Tomato;">a</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">:</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;">r</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">g</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">y</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>P</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">N</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">:</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">g</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Literal object property and method names

Literal object property and method names are in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case).

```jsx
const alice = {
    fullName: "Alice",
}
```

Example of an incorrect name:

```jsx
const alice = {
    FULL_NAME: "Alice",
}
```

<pre class="language-text"><code class="language-text">style/useNamingConvention.js:2:5 <a href="https://biomejs.dev/linter/rules/use-naming-convention">lint/style/useNamingConvention</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>object property</strong></span><span style="color: Orange;"> name should be in </span><span style="color: Orange;"><strong>camelCase</strong></span><span style="color: Orange;">.</span>
  
    <strong>1 │ </strong>const alice = {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    FULL_NAME: &quot;Alice&quot;,
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The name could be renamed to `fullName`.</span>
  
</code></pre>

### Imported and exported module aliases

Imported and exported module aliases are in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case) or [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).

```jsx
import * as myLib from "my-lib";
import * as Framework from "framework";

export * as myLib from "my-lib";
export * as Framework from "framework";
```

`import` and `export` aliases are in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case), [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case), or [`CONSTANT_CASE`](https://en.wikipedia.org/wiki/Snake_case):

```jsx
import assert, {
    deepStrictEqual as deepEqual,
    AssertionError as AssertError
} from "node:assert";
```

Examples of an incorrect name:

```ts
import * as MY_LIB from "my-lib";
```

<pre class="language-text"><code class="language-text">style/useNamingConvention.js:1:13 <a href="https://biomejs.dev/linter/rules/use-naming-convention">lint/style/useNamingConvention</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>import namespace</strong></span><span style="color: Orange;"> name should be in </span><span style="color: Orange;"><strong>camelCase or PascalCase</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import * as MY_LIB from &quot;my-lib&quot;;
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The name could be renamed to `myLib`.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Rename this symbol in </span><span style="color: lightgreen;"><strong>camelCase</strong></span><span style="color: lightgreen;">.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">*</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>M</strong></span><span style="color: Tomato;"><strong>Y</strong></span><span style="color: Tomato;"><strong>_</strong></span><span style="color: Tomato;"><strong>L</strong></span><span style="color: Tomato;"><strong>I</strong></span><span style="color: Tomato;"><strong>B</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">m</span><span style="color: Tomato;">y</span><span style="color: Tomato;">-</span><span style="color: Tomato;">l</span><span style="color: Tomato;">i</span><span style="color: Tomato;">b</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">*</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>m</strong></span><span style="color: MediumSeaGreen;"><strong>y</strong></span><span style="color: MediumSeaGreen;"><strong>L</strong></span><span style="color: MediumSeaGreen;"><strong>i</strong></span><span style="color: MediumSeaGreen;"><strong>b</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">y</span><span style="color: MediumSeaGreen;">-</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### TypeScript type parameter names

A _TypeScript_ type parameter name is in [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).

```ts
function id<Val>(value: Val): Val { /* ... */}
```

### TypeScript `namespace` names

A _TypeScript_ `namespace` name is in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case) or in [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).

```ts
namespace mathExtra {
    /*...*/
}

namespace MathExtra {
    /*...*/
}
```

## Options

The rule provides two options that are detailed in the following subsections.

```json
{
    "//": "...",
    "options": {
        "strictCase": false,
        "enumMemberCase": "CONSTANT_CASE"
    }
}
```

### strictCase

When this option is set to `true`, it forbids consecutive uppercase characters in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case) and [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).
For instance,  when the option is set to `true`, `HTTPServer` or `aHTTPServer` will throw an error.
These names should be renamed to `HttpServer` and `aHttpServer`

When the option is set to `false`, consecutive uppercase characters are allowed.
`HTTPServer` and `aHTTPServer` are so valid.

Default: `true`

### requireAscii

When this option is set to `true`, it forbids names that include non-ASCII characters.
For instance,  when the option is set to `true`, `café` or `안녕하세요` will throw an error.

When the option is set to `false`, anames may include non-ASCII characters.
`café` and `안녕하세요` are so valid.

Default: `false`

**This option will be turned on by default in Biome 2.0.**

### enumMemberCase

By default, the rule enforces the naming convention followed by the [TypeScript Compiler team](https://www.typescriptlang.org/docs/handbook/enums.html):
an `enum` member is in [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).

You can enforce another convention by setting `enumMemberCase` option.
The supported cases are: [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case), [`CONSTANT_CASE`](https://en.wikipedia.org/wiki/Snake_case), and [`camelCase`](https://en.wikipedia.org/wiki/Camel_case).

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
