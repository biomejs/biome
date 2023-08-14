---
title: Lint Rule useNamingConvention
parent: lint/rules/index
---

# useNamingConvention (since v13.0.0)

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

<pre class="language-text"><code class="language-text">nursery/useNamingConvention.js:1:5 <a href="https://docs.rome.tools/lint/rules/useNamingConvention">lint/nursery/useNamingConvention</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>top-level let</strong></span><span style="color: Orange;"> name should be in </span><span style="color: Orange;"><strong>camelCase</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let a_value = 0;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The name could be renamed to `aValue`.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Rename this symbol in </span><span style="color: rgb(38, 148, 255);"><strong>camelCase</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>_</strong></span><span style="color: Tomato;"><strong>v</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">0</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>V</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>l</strong></span><span style="color: MediumSeaGreen;"><strong>u</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">0</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
function f(FirstParam) {}
```

<pre class="language-text"><code class="language-text">nursery/useNamingConvention.js:1:12 <a href="https://docs.rome.tools/lint/rules/useNamingConvention">lint/nursery/useNamingConvention</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>function parameter</strong></span><span style="color: Orange;"> name should be in </span><span style="color: Orange;"><strong>camelCase</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(FirstParam) {}
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The name could be renamed to `firstParam`.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Rename this symbol in </span><span style="color: rgb(38, 148, 255);"><strong>camelCase</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
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


- A static property name and a static getter name are in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case) or [`CONSTANT_CASE`](https://en.wikipedia.org/wiki/Snake_case).


- A class property name and a class method name are in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case).



```jsx
class Person {
    static MAX_FRIEND_COUNT = 256;

    static get SPECIAL_PERSON_INSTANCE() { /*...*/ }

    initializedProperty = 0;

    specialMethod() {}
}
```

### TypeScript `type` aliases and `interface`

- A `type` alias and an interface name are in [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).


- A property name and a method name in a type or interface are in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case) or [`CONSTANT_CASE`](https://en.wikipedia.org/wiki/Snake_case).


- A `readonly` property name and a getter name can also be in [`CONSTANT_CASE`](https://en.wikipedia.org/wiki/Snake_case).



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

<pre class="language-text"><code class="language-text">nursery/useNamingConvention.js:1:6 <a href="https://docs.rome.tools/lint/rules/useNamingConvention">lint/nursery/useNamingConvention</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>type alias</strong></span><span style="color: Orange;"> name should be in </span><span style="color: Orange;"><strong>PascalCase</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>type person = { fullName: string };
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The name could be renamed to `Person`.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Rename this symbol in </span><span style="color: rgb(38, 148, 255);"><strong>PascalCase</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
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

<pre class="language-text"><code class="language-text">nursery/useNamingConvention.js:2:5 <a href="https://docs.rome.tools/lint/rules/useNamingConvention">lint/nursery/useNamingConvention</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>object property</strong></span><span style="color: Orange;"> name should be in </span><span style="color: Orange;"><strong>camelCase</strong></span><span style="color: Orange;">.</span>
  
    <strong>1 │ </strong>const alice = {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    FULL_NAME: &quot;Alice&quot;,
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The name could be renamed to `fullName`.</span>
  
</code></pre>

### Imported and exported module aliases

Imported and exported module aliases are in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case).

```jsx
import * as myLib from "my-lib";

export * as myLib from "my-lib";
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
import * as MyLib from "my-lib";
```

<pre class="language-text"><code class="language-text">nursery/useNamingConvention.js:1:13 <a href="https://docs.rome.tools/lint/rules/useNamingConvention">lint/nursery/useNamingConvention</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>import namespace</strong></span><span style="color: Orange;"> name should be in </span><span style="color: Orange;"><strong>camelCase</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import * as MyLib from &quot;my-lib&quot;;
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The name could be renamed to `myLib`.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Rename this symbol in </span><span style="color: rgb(38, 148, 255);"><strong>camelCase</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">*</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>M</strong></span><span style="color: Tomato;"><strong>y</strong></span><span style="color: Tomato;"><strong>L</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">m</span><span style="color: Tomato;">y</span><span style="color: Tomato;">-</span><span style="color: Tomato;">l</span><span style="color: Tomato;">i</span><span style="color: Tomato;">b</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">;</span>
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

### enumMemberCase

By default, the rule enforces the naming convention followed by the [TypeScript Compiler team](https://www.typescriptlang.org/docs/handbook/enums.html):
an `enum` member has to be in [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).

You can enforce another convention by setting `enumMemberCase` option.
The supported cases are: [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case), [`CONSTANT_CASE`](https://en.wikipedia.org/wiki/Snake_case), and [`camelCase`](https://en.wikipedia.org/wiki/Camel_case).

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
