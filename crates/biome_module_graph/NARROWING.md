# Narrowing reference

This file describes what flow-sensitive narrowing in Biome's type inference
does, compared with TypeScript 6.0.3 run with `--strict`. It is the live
checklist of TypeScript's narrowing behaviour: a pull request that changes
narrowing updates the rows it affects in the same pull request, including the
deliberate choices below.

How narrowing is implemented, and how to extend it, is described in the flow
section of [`CONTRIBUTING.md`](./CONTRIBUTING.md#flow-sensitive-expression-reads).
The remaining work is tracked in
[#13147](https://github.com/biomejs/biome/issues/13147).

## Terms

- **Narrowing** gives one read a more specific type than its declared type,
  from the conditions and statements on the paths that reach the read. The
  declared type never changes.
- **Declared type**: a variable's type before narrowing, from its annotation
  or from inference that ignores flow.
- **Read**: one place where the program uses the value of a variable or of a
  property path.
- **Execution root**: a body analyzed on its own: a module, a namespace, a
  function, a method, an accessor, a constructor, or a class static block.
- **Condition**: the expression whose result decides which branch runs, such
  as the test of an `if` or an operand of `&&`. A **runtime test** is one
  check of one variable that a condition performs, such as `typeof x ===
  "string"`.
- **Subject**: what a runtime test checks: a variable or `this`, optionally
  followed by a path of property names, such as `opts.timeout` or `pair[0]`.
- **Type predicate**: a return annotation such as `value is T`,
  `asserts value is T`, or `asserts value`. A **type-guard function** returns a
  type predicate; an **assertion function** is a type-guard function whose type
  predicate starts with `asserts`.

## Correctness contract

Every narrowing behaviour follows these rules:

1. At any read, Biome's narrowed type is never narrower than TypeScript's
   narrowed type at the same read.
2. When inference cannot decide, the read keeps its declared type. Where a
   work limit or a query cycle stops the computation, the read has type
   Unknown instead.
3. A read in a branch proven impossible has type Unknown, not `never`. A rule
   that needs to know that a point is unreachable must get that answer from a
   separate request, never from the read's type. Biome has no such request yet.

A type wider than TypeScript's can still make a rule report a union member
that TypeScript has removed, but a narrower type makes rules report code that
TypeScript accepts, or suggest deleting a check that the program needs.

## TypeScript's unsound narrowing

Some of TypeScript's narrowing is unsound: the narrowed type excludes values
that the read can have at runtime. Biome copies such narrowing only when the
sound alternative loses narrowing that idiomatic code relies on and the
unsound case needs unusual code. When a sound result keeps TypeScript's
precision, Biome uses it instead.

Each deliberate choice has a row here: a place where Biome differs from
TypeScript, or unsound narrowing that Biome copies from TypeScript. Do not
change these results, to match TypeScript or to make them sound, without
revisiting the row.

| Behaviour | Example | TypeScript | Biome | Reason |
| --- | --- | --- | --- | --- |
| `??` used as a condition | `x: string \| null \| undefined; y: boolean;`<br>`if (x ?? y) {} else { x }` | `null \| undefined` | `string \| null \| undefined`: `??` in a condition narrows neither operand in either branch | `x` is `""` in the `else` branch when `x` is `""` and `y` is false, because `""` is not nullish. The pattern is rare, so the sound result loses little. |
| Falsy part of a broad `number` in the value of `&&` | `n: number; s: string;`<br>`n && s` | `string \| 0` | `number \| string` | `-0` and `NaN` are falsy numbers too, so `0` is wrong at runtime. A `string` or `bigint` left operand still contributes `""` or `0n`, its only falsy value. |
| `eval` and the `arguments` object do not stop narrowing | `x: string \| null;`<br>`if (x !== null) { eval("x = null"); x }`<br>`if (x !== null) { arguments[0] = null; x }` | `string` | `string`, the same as TypeScript | A direct `eval` can write any variable it can see, including a type-guard function. In a function that is not in strict mode and has no default, rest, or destructured parameter, `arguments[0] = null` also writes the first parameter. Either way, `x` can be `null` at the read. Excluding them would stop narrowing in every function that reads `arguments`, even only `arguments.length`, or that contains a direct `eval`, while code that writes a narrowed variable this way is rare. |
| Calls, callbacks, and writes through other variables keep property narrowing | `o: { v: string \| null }; reset: () => void;`<br>`xs: (string \| null)[];`<br>`if (o.v) { reset(); o.v }`<br>`if (xs[0]) { xs.shift(); xs[0] }`<br>`if (o.v) { const a = o; a.v = null; o.v }`<br>`if (o.v) { [1].forEach(() => { o.v = null; }); o.v }` | `string` | `string`, the same as TypeScript. A write to `o.v` in the function's own code still stops the narrowing, including inside a function or arrow expression called where it is defined and inside a class static block. Biome also counts such a function when it is `async`, where TypeScript gives `string`. | `reset` or the callback can set `o.v` to `null`, and `a.v = null` sets it through another variable, so `o.v` can be `null` at the read. `xs.shift()` moves the next element, which can be `null`, into `xs[0]`. Stopping property narrowing at every call would make it useless, because most tests of a property are followed by a call, and rules would report code that TypeScript accepts. An `async` function called where it is defined runs until its first `await` before the code after the call, so Biome counts its writes. |
| A write through an element access whose key is not a literal | `o: { v: string \| null; w: string \| null };`<br>`k: "v" \| "w";`<br>`if (o.v) { o[k] = null; o.v }` | `string` | `string \| null`: a write through a key that is not a string or integer literal, such as `k` or `-0`, stops the narrowing of every property of the object. A literal key writes only the property it names, so `o["w"] = null` keeps the narrowing of `o.v`. `delete o["v"]` also stops the narrowing of `o.v`, where TypeScript gives `string`. | TypeScript ignores the write unless the key is a literal or a `const` with a literal type, although `k` can be `"v"` here, and it ignores `delete` through an element access. Treating such a write as a write is sound, and such writes are rare next to tests of a property. |
| A getter narrows like a field | `class G { get v(): string \| null }`<br>`g: G; if (g.v) g.v` | `string` | `string`, the same as TypeScript | A getter can return a different value each time it runs, so `g.v` can be `null` at the second read. Treating getters as fields keeps narrowing for classes that expose their state through getters, while getters that change between two reads close together are rare. |

## Limits

These limits apply to every *Supported* and *Partial* row below:

- A read narrows only when it reads a parameter, or a variable declared with
  `let`, `const`, or `function`, that is declared in the execution root of the
  read and never written after its declaration. `var` declarations and
  imported bindings never narrow, and neither does a variable that a
  destructuring default can supply, as in `const { a = 0 } = o`, because its
  declared type ignores the default. A read inside a nested function or class
  member never sees facts from the enclosing execution root, and a read in a
  class field, including its decorators and computed key, never narrows.
- A property path, such as `o.v` or `a.b.c`, narrows only when its variable
  can narrow by the rule above, so a path on a variable declared in an
  enclosing execution root, such as a module variable read in a function,
  does not narrow. Its declared type at a read is the property's type on the
  narrowed type of its object at that read. A path is made of named
  properties, written with `.` or `?.`, and of element accesses whose key is
  a string literal without escapes or a decimal integer literal from 0 to
  2^53 - 1 without leading zeros or separators, such as `o["v"]` or `t[0]`.
  `o["v"]`, `o?.v`, and `(o).v` are the same path as `o.v`, and `t[0]` is
  the same path as `t["0"]`. Element access with any other key, `this.x`, and
  private names such as `o.#v` do not narrow yet.
- A property path does not narrow anywhere in its execution root when the
  root's own code writes the path or one of its prefixes, or writes any
  property of one of its prefixes through a key that is not a literal of the
  kind above, such as `o[k] = x` or `o[-0] = x`. A write through such a
  literal, such as `o["v"] = x`, writes only the path it names. Assignments
  of every kind, `++` and `--`, destructuring targets, the targets of
  `for...in` and `for...of`, and `delete` count as writes, even after the
  read. So do writes inside a function or arrow expression called where it
  is defined, a class static block, a field initializer, a decorator, a
  computed key, or a parameter default. Writes in the body of any other
  function, method, accessor, or constructor do not count, and neither do
  writes through another variable.
- Variables whose declared type is `any`, or includes a part that inference
  cannot determine, never narrow.
- An execution root gets no narrowing at all when its own code, outside nested
  functions, contains a `with` statement or a syntax error, or when it is very
  large or deeply nested.
- Biome does not model the control flow inside a `switch`, `try`, `for...in`,
  or `for...of` statement yet. A read inside one of these statements has the
  type from just before the statement, narrowed further only by the rest of the
  expression that contains the read, as in `x && x.length`. An `if`, a
  `return`, or a loop inside the statement narrows nothing. The code after the
  statement continues with the types from just before it, and so does the
  target of a `break` or `continue` that leaves the statement. Class and enum
  declarations, namespaces, `import x = require()`, and `declare` statements
  do not stop narrowing either.

## Behaviour

Statuses:

- *Supported*: Biome gives TypeScript's narrowed type, within the limits above.
- *Partial*: Biome narrows less than TypeScript; the row says what is missing.
- *Not yet*: Biome keeps the declared type.
- *Deliberately different*: Biome differs on purpose; the row gives the reason.

Examples write a parameter or declared variable as `x: T` and elide the
function around it. Unless a row says otherwise, the TypeScript result is the
type of the last read of `x` in the example. TypeScript reports unreachable
code as TS7027, an error with `allowUnreachableCode: false`.

### Truthiness

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| The truthy branch removes nullish types and falsy literals | `x: string \| null; if (x) x` | `string` | Supported |
| The falsy branch keeps broad `string`, `number`, and `bigint` | `x: string \| null; if (!x) x` | `string \| null` | Supported |
| `boolean` splits into `true` and `false` | `x: boolean \| undefined; if (!x) x` | `false \| undefined` | Supported |
| The falsy branch removes object types | `interface Foo { a: number }`<br>`x: Foo \| undefined; if (!x) x` | `undefined` | Partial: removes `object`, functions, tuples, object literals, and instances of built-in classes such as arrays, `Date`, and `Map`, but keeps interfaces, object type literals, and instances of other classes, because a primitive can satisfy them (`{}` describes `0`). |
| `!`, `!!`, and parentheses | `x: string \| null; if (!!x) x` | `string` | Supported |
| `&&` and `\|\|` in conditions and as operands | `x: string \| null; y: number \| undefined;`<br>`if (x && y) x`; `x && x.length` | `string` at both reads | Supported |
| The branches of a conditional expression | `x: string \| null; x ? f(x) : g(x)` | `string` in `f(x)`, `string \| null` in `g(x)` | Supported |
| Truthiness of a property narrows the property | `o: { v: string \| null }; if (o.v) o.v` | `string` | Supported |
| Truthiness of a generic gives `NonNullable<T>` | `function f<T>(x: T) { x && x }` | `NonNullable<T>` at the right operand | Not yet |
| A constant condition makes a branch unreachable | `x: string \| null;`<br>`if (x === null) { if (true) return; } x` | `string`; the branch of `if (false)` is unreachable (TS7027) | Partial: only `while` and `do...while` loops whose condition is a truthy literal, and `for` loops without a condition, have no exit other than `break`. The branches of `if (true)` and `if (false)` stay reachable, so the example gives `string \| null`. |

### Equality

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| `=== null` and `!== null` | `x: string \| null; if (x !== null) x` | `string` | Supported |
| `=== undefined` and `!== undefined` | `x: string \| undefined; if (x === undefined) return; x` | `string` | Supported |
| `== null` and `!= undefined` match both nullish values | `x: string \| null \| undefined; if (x != null) x` | `string` | Supported |
| Strict equality with a string, number, bigint, or boolean literal | `x: "a" \| "b"; if (x === "a") x` | `"a"`; `"b"` in the `else` branch | Supported |
| A broad primitive becomes the literal | `x: string; if (x === "a") x` | `"a"` | Supported |
| Loose `==` with a literal | `x: "a" \| "b" \| null; if (x == "a") x` | `"a"` | Not yet |
| Equality with a negative number, `void 0`, a template literal, or an enum member | `x: number; if (x === -1) x`<br>`enum E { A, B }; e: E; if (e === E.A) e` | `-1`; `E.A` | Not yet |
| Equality with another variable | `x: string \| number; y: "a"; if (x === y) x` | `"a"` | Not yet |
| Equality with the result of a runtime test | `x: string \| number;`<br>`if ((typeof x === "string") === true) x` | `string` | Not yet |
| `unknown` compared with a literal | `x: unknown; if (x === "a") x` | `"a"` | Supported |
| `unknown` compared with `null` loosely | `x: unknown; if (x != null) x` | `{}` | Not yet |
| `unknown` compared with an object | `x: unknown; o: { a: number }; if (x === o) x` | `object` | Not yet |

### `typeof`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Primitive kinds, in both branches | `x: string \| number; if (typeof x === "string") x` | `string`; `number` in the `else` branch | Supported |
| Reversed operands | `x: string \| number; if ("string" === typeof x) x` | `string` | Supported |
| A primitive kind removes object types | `interface Foo { a: number }`<br>`x: string \| Foo; if (typeof x === "string") x` | `string` | Partial: removes `object`, functions, tuples, object literals, and instances of built-in classes, but keeps interfaces, object type literals, and instances of other classes, so the example gives `string \| Foo`. |
| `"object"` keeps object types and `null`; its negation removes them | `x: string \| Foo \| null; if (typeof x !== "object") x` | `string` | Partial: the positive branch matches TypeScript; the negative branch keeps the object types that a primitive `typeof` keeps, so the example gives `string \| Foo`. |
| `typeof` of `unknown` | `x: unknown; if (typeof x === "object") x`<br>`if (typeof x === "function") x` | `object \| null`; `Function` | Partial: only `"string"`, `"number"`, `"bigint"`, `"boolean"`, `"symbol"`, and `"undefined"` narrow `unknown`. |
| `typeof` of `{}` gives the primitive | `x: {}; if (typeof x === "string") x` | `string` | Not yet |
| `typeof` of a generic gives an intersection | `function f<T>(x: T) { if (typeof x === "string") x }` | `T & string` | Not yet |
| `typeof` of a property | `o: { v: string \| number }; if (typeof o.v === "string") o.v` | `string` | Supported |
| `typeof` of a discriminant narrows the object | `o: { k: "a"; a: 1 } \| { k: 1; b: 2 };`<br>`if (typeof o.k === "string") o` | `{ k: "a"; a: 1 }` | Not yet |
| `typeof x?.y` removes nullish types from `x` | `x: { y: number } \| undefined;`<br>`if (typeof x?.y === "number") x` | `{ y: number }` | Not yet |
| `switch (typeof x)` | `x: string \| number;`<br>`switch (typeof x) { case "string": x }` | `string` | Not yet |

### `instanceof`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| The true branch | `x: Date \| string; if (x instanceof Date) x` | `Date` | Not yet |
| The false branch removes the class and its derived types | `x: Date \| string; if (!(x instanceof Date)) x` | `string` | Not yet |
| `unknown` and `any` become the instance type | `x: unknown; if (x instanceof Map) x` | `Map<any, any>` | Not yet |
| `Symbol.hasInstance` with a type predicate | `class Checker { static [Symbol.hasInstance](v: unknown): v is string }`<br>`x: unknown; if (x instanceof Checker) x` | `string` | Not yet |
| `x.constructor === C` | `class Foo { a = 1 }; class Bar { b = 2 }`<br>`x: Foo \| Bar; if (x.constructor === Foo) x` | `Foo` | Not yet |

### `in`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Keeps the union members that declare the property | `x: { a: 1 } \| { b: 2 }; if ("a" in x) x` | `{ a: 1 }` | Not yet |
| An undeclared property adds a `Record` | `x: object; if ("a" in x) x` | `object & Record<"a", unknown>` | Not yet |
| `#brand in x` | `class C { #p = 1 }`<br>`x: C \| Date; if (#p in x) x` (inside `C`) | `C` | Not yet |
| `in` and `hasOwnProperty` remove a missing optional property | `x: { a?: string }; if ("a" in x) x.a`<br>(with `exactOptionalPropertyTypes`) | `string`; `string \| undefined` without `exactOptionalPropertyTypes` | Not yet |

### Library type predicates

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| `Array.isArray` | `x: string \| string[]; if (Array.isArray(x)) x` | `string[]` | Not yet |
| `every` with a type predicate callback | `isString: (v: unknown) => v is string;`<br>`xs: (string \| number)[]; if (xs.every(isString)) xs` | `string[]` | Not yet |
| DOM `matches` | `el: Element; if (el.matches("a")) el` (with the `dom` library) | `HTMLAnchorElement` | Not yet |

### Discriminated unions

The rows use `type Circle = { kind: "c"; r: number }` and
`type Square = { kind: "s"; w: number }`.

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Equality of a discriminant | `s: Circle \| Square; if (s.kind === "c") s` | `Circle` | Not yet |
| Truthiness of a discriminant | `r: { ok: true; v: number } \| { ok: false; e: string };`<br>`if (r.ok) r` | `{ ok: true; v: number }` | Not yet |
| Destructured `const` or parameter | `s: { kind: "c"; v: number } \| { kind: "s"; v: string };`<br>`const { kind, v } = s; if (kind === "c") v` | `number` | Not yet |
| Dependent rest parameters | `(...[k, p]: ["a", number] \| ["b", string]) => { if (k === "a") p }` | `number` | Not yet |
| Discriminant held in a `const` | `s: Circle \| Square; const k = s.kind; if (k === "c") s` | `Circle` | Not yet |
| Discriminant read through optional chaining | `s: Circle \| Square \| undefined; if (s?.kind === "c") s` | `Circle` | Not yet |

### `switch`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Literal cases | `x: "a" \| "b" \| "c"; switch (x) { case "a": x }` | `"a"` | Not yet |
| Grouped cases and fallthrough | `x: "a" \| "b" \| "c"; switch (x) { case "a": case "b": x }` | `"a" \| "b"` | Not yet |
| `default` removes the handled values | `x: "a" \| "b" \| "c";`<br>`switch (x) { case "a": break; case "b": break; default: x }` | `"c"` | Not yet |
| `switch (true)` | `x: string \| number;`<br>`switch (true) { case typeof x === "string": x }` | `string` | Not yet |
| `switch` on a discriminant | `s: Circle \| Square; switch (s.kind) { case "c": s }` | `Circle` | Not yet |
| An exhaustive `switch` ends the path that matches no case | `x: "a" \| "b";`<br>`switch (x) { case "a": return 1; case "b": return 2; } x` | `never`; the read is unreachable (TS7027) | Not yet |

### Type predicates

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A local, non-generic type-guard function with a primitive target | `function isString(v: unknown): v is string;`<br>`x: string \| number; if (isString(x)) x` | `string` | Supported, with the restrictions below the table |
| The false branch excludes the target | `x: string \| number; if (!isString(x)) x` | `number` | Supported |
| An object, interface, class, or union target | `function isFoo(v: unknown): v is Foo;`<br>`x: Foo \| string; if (isFoo(x)) x` | `Foo` | Not yet |
| An imported, namespace, or method type-guard function | `import { isString } from "./guards"`; `u.isString(x)` | `string` | Not yet |
| A generic type-guard function | `function isNonNull<T>(v: T \| null): v is T;`<br>`x: string \| null; if (isNonNull(x)) x` | `string` | Not yet |
| An overloaded type-guard function | `function isS(v: string \| number): v is string;`<br>`function isS(v: boolean): boolean;`<br>`x: string \| number; if (isS(x)) x` | `string` | Not yet |
| `this is T` | `class TreeNode { isLeaf(): this is Leaf }`<br>`n: TreeNode; if (n.isLeaf()) n` | `Leaf` | Not yet |
| The argument is a property or an optional chain | `o: { v: string \| number }; if (isString(o.v)) o.v`<br>`x: { y: string \| number } \| undefined; if (isString(x?.y)) x` | `string`; `{ y: string \| number }` | Partial: the property narrows, but the receiver of an optional chain does not, so the second example gives `{ y: string \| number } \| undefined`. |
| Inferred type predicates | `const isStr = (v: unknown) => typeof v === "string";`<br>`x: string \| number; if (isStr(x)) x` | `string` | Not yet |
| A variable annotated with a function type that returns a type predicate | `const g: (v: unknown) => v is string = ...;`<br>`if (g(x)) x` | `string` | Not yet |

A supported type-guard function is declared in the same module with
`function` or `declare function`, or is an arrow or function expression that
initializes an unannotated `let` or `const`. Nothing reassigns it, and it has
one signature that is not generic, `async`, or a generator. The tested
parameter is named, is not a rest parameter, and has no default value. The
target is a primitive type or a primitive literal, directly, through a local
type alias, or through `typeof` of a value. The call is not optional, has no
type arguments, passes a variable as the tested argument, and has no spread
argument before or at the tested one. Neither Biome nor TypeScript checks the
body of a type-guard function.

### Assertions

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| `asserts value` | `function assert(c: unknown): asserts c;`<br>`x: string \| null; assert(x !== null); x` | `string` | Supported, with the restrictions below the table |
| `asserts value is T` with a primitive target | `function assertString(v: unknown): asserts v is string;`<br>`x: string \| number; assertString(x); x` | `string` | Supported |
| `asserts value is T` with an object or union target | `function assertFoo(v: unknown): asserts v is Foo;`<br>`x: Foo \| string; assertFoo(x); x` | `Foo` | Not yet |
| `asserts this` and `asserts this is T` | `class R { ready?: string; assertReady(): asserts this is { ready: string } }`<br>`this.assertReady(); this` (inside `R`) | `this & { ready: string }` | Not yet |
| A method, imported, or annotated assertion function | `x: string \| null; assert.ok(x); x`<br>`a: (v: unknown) => asserts v; a(x); x` | `string` | Not yet |
| An assertion function declared as a variable without a type annotation is ignored (TS2775) | `const a = (v: unknown): asserts v => {};`<br>`x: string \| null; a(x); x` | `string \| null`, with error TS2775 | Supported |
| An assertion call as an operand of the comma operator | `x: string \| null; (assert(x), 0); x` | `string` | Not yet |
| `assert(false)` ends the path | `x: string \| null; if (x === null) assert(false); x` | `string`; code after `assert(false)` is unreachable (TS7027) | Not yet |

An assertion function follows the restrictions of a type-guard function and
must also be declared with `function` or `declare function`. The call must
form a whole expression statement, optionally in parentheses, and must not be
optional. Calls inside other expressions, and calls to a type-guard function
whose result is not tested, do not narrow.

### Calls that never return

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A call to a function returning `never` ends the path | `declare function fail(): never;`<br>`x: string \| null; if (x === null) fail(); x` | `string`; code after `fail()` is unreachable (TS7027) | Not yet |
| `return` and `throw` end the path | `x: string \| null; if (!x) throw new Error(); x` | `string` | Supported |

### Assignments

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| An assignment narrows to the matching members of the declared type | `let x: string \| number; x = "a"; x` | `string` | Not yet |
| An initializer narrows an annotated union | `const x: string \| number = 1; x` | `number` | Not yet |
| Compound assignments and `++` | `let x: number \| string = 0; x += 1; x` | `number` | Not yet |
| Logical assignments | `x: string \| undefined; x ??= "d"; x` | `string` | Not yet |
| Destructuring assignments | `let x: string \| number; [x] = ["a"]; x` | `string` | Not yet |
| An unannotated `let` takes the type of its assignments | `let x; x = 1; x` | `number` | Not yet |
| `delete` counts as an assignment | `o: { p?: string }; if (o.p) { delete o.p; o.p }` | `undefined` | Not yet |

### Aliased conditions

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A `const` holding a runtime test, up to five levels deep | `x: string \| number; const isS = typeof x === "string"; if (isS) x` | `string` | Not yet |
| A `const` holding `&&` or `\|\|` of runtime tests | `a: string \| null; b: string \| null;`<br>`const ok = a !== null && b !== null; if (ok) a` | `string` | Not yet |

### Closures

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A `const` read in an arrow or function expression keeps the outer narrowing | `const x: string \| null = get(); if (x) () => x` | `string` | Not yet |
| A parameter or `let` read in a closure created after its last assignment | `function f(x: string \| null) { if (x) [1].forEach(() => x) }` | `string` | Not yet |
| A function declaration never sees the outer narrowing | `x: string \| null; if (x) { function g() { x } }` | `string \| null` | Supported |
| An immediately invoked function expression continues the outer flow | `x: string \| null; if (x) (() => x)()` | `string` | Not yet |

### Class members

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A method or accessor of a class expression keeps the outer narrowing as an arrow function does, and a static block of any class continues the outer flow | `x: string \| null;`<br>`if (x) { const C = class { m() { x } }; class D { static { x } n() { x } } }` | `string` in `m` and in the static block; `string \| null` in `n`, because a method of a class declaration never sees the outer narrowing | Not yet: Biome analyzes every method, accessor, and static block on its own, so a read of an outer variable there keeps its declared type. |
| The decorators and computed keys of class members | `x: string \| null;`<br>`if (x) { const C = class { @dec(x) m() {} @dec(x) f?: number; @dec(x) g = 1 } }` | `string` in the decorators of `m` and `f`, `string \| null` in the decorator of `g`. In any class, the decorators and computed keys of a field without an initializer narrow, and those of a field with an initializer do not. Those of a method or accessor narrow in a class expression but not in a class declaration. | Not yet: Biome evaluates no class member in the enclosing flow, so these reads keep their declared type. |

### Loops

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| The condition of `while`, `do...while`, and `for` narrows the body and the exit | `x: string \| null; while (x) x` | `string` | Supported |
| A read in a loop joins the entry and the back edge | `x: string \| null; while (flag) { x; if (!x) return; }` | `string \| null` at `x;` | Supported |
| `while (true)` and `for (;;)` exit only through `break` | `x: string \| null; while (true) { if (x) break; } x` | `string` | Supported |
| `for...in` and `for...of` | `xs: (string \| null)[]; for (const v of xs) { if (v !== null) v }` | `string` | Not yet: a condition in the body narrows nothing, but the narrowing from before the loop still applies in the body and after the loop, as the limits above describe. |
| `for (k in o)` removes nullish types from `o` | `o: { a: 1 } \| undefined; for (const k in o) o` | `{ a: 1 }` | Not yet |
| Labeled `break` and `continue` | `x: string \| null; out: { if (x === null) break out; x }` | `string` | Supported |

### `try`, `catch`, and `finally`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| `catch` and `finally` see every assignment in `try` | `let x: string \| number = "a"; try { x = 1; f(); } catch { x }` | `string \| number` | Not yet |

### Optional chaining

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A truthy `x?.y` removes nullish types from `x` | `x: { y: string } \| undefined; if (x?.y) x` | `{ y: string }` | Not yet |
| Equality of `x?.y` with a value | `x: { y: number } \| undefined; if (x?.y === 1) x` | `{ y: number }` | Not yet |
| The receiver is not nullish inside the chain | `x: { f(v: unknown): void } \| undefined; x?.f(x)` | `{ f(v: unknown): void }` at the argument | Not yet |
| `instanceof`, `switch`, or a type-guard function applied to `x?.y` | `x: { y: unknown } \| undefined; if (x?.y instanceof Date) x` | `{ y: unknown }` | Not yet |

### Nullish coalescing

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| The right operand sees the left operand as nullish | `x: string \| null; x ?? f(x)` | `null` at `f(x)` | Supported |
| The value of `??` | `x: string \| null; d: number; x ?? d` | `string \| number` | Supported |
| `??` used as a condition | `x: string \| null \| undefined; y: boolean;`<br>`if (x ?? y) {} else { x }` | `null \| undefined` | Deliberately different: Biome keeps the declared type, because `x` can be `""` in the `else` branch. See the deliberate choices above. |

### Property subjects

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Property paths such as `a.b.c` | `a: { b: { c: string \| null } }; if (a.b.c) a.b.c` | `string` | Supported |
| An optional chain reads the same property as a plain access | `x: { y: string \| null } \| undefined; if (x?.y) x?.y` | `string` | Supported |
| `this.x`, including assignments in a constructor | `class K { x: string \| null = null; m() { if (this.x) this.x } }` | `string` | Not yet |
| Element access with a literal key | `o: { a: string \| null }; if (o["a"]) o["a"]` | `string` | Partial: `o["a"]` reads the same property as `o.a`, and `t[0]` the same as `t["0"]`, but only a string literal without escapes or a decimal integer from 0 to 2^53 - 1 without leading zeros or separators is a key. TypeScript also matches other spellings of the same key, so `if (t[0x0]) t[0]` and ``if (o[`a`]) o.a`` narrow there but not in Biome. |
| Element access with a `const` key | `o: { a: string \| number }; const k = "a";`<br>`if (typeof o[k] === "string") o[k]` | `string` | Not yet |

### Variable kinds

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Imported bindings and `var` | `var x: string \| null = get(); if (x) x` | `string` | Not yet |

### Evolving arrays

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| An unannotated `let x = []` takes the types of `push`, `unshift`, and index assignments | `let x = []; x.push(1); x` | `number[]` | Not yet |

### `unknown` and `{}`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A truthy `unknown` becomes `{}` | `x: unknown; if (x) x` | `{}` | Not yet |

### Generics

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| The constraint replaces the type parameter where a value is used | `function f<T extends string \| undefined>(x: T) { if (x) x.length }` | `x` reads as `string` | Not yet |
| A type predicate or `instanceof` on a generic gives an intersection | `function f<T>(x: T) { if (isFoo(x)) x }` | `T & Foo` | Not yet |

### `any`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Primitive `typeof`, `instanceof`, and type predicates narrow `any`; equality and `typeof` with `"object"` or `"function"` do not | `x: any; if (typeof x === "string") x` | `string` | Not yet |

### `void`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Runtime tests treat `void` as `undefined` | `x: void \| string; if (x) x` | `string` | Partial: a `void` member makes the narrowed read Unknown, because a function whose return type is `void` can return any value. |

### Other behaviour

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| `x!`, `satisfies`, or the comma operator inside a condition | `x: string \| null; if ((0, x)) x`; `if (x!) x` | `string` | Not yet |
| Paths join with a union | `x: string \| number \| null;`<br>`if (typeof x === "string") {} else if (x === null) { return; } x` | `string \| number` | Supported |
| A namespace narrows its own variables but never sees the narrowing around it | `o: string \| null; if (o === null) throw 0;`<br>`namespace N { const l: string \| null = get(); if (l) l; o }` | `string` at `l`; `string \| null` at `o` | Supported |
| Reads in unreachable code | `x: string \| null; if (x === null) { return; x }` | `string \| null`; the read is unreachable (TS7027) | Supported |
| A branch narrowed to an impossible type | `x: string; if (typeof x === "number") x` | `never` | Partial: the read has type Unknown, as the correctness contract requires, and no request reports the branch as unreachable yet. |
