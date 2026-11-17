# Narrowing reference

This file describes what flow-sensitive narrowing in Biome's type inference
does, compared with TypeScript 7.0.2 run with `--strict`. It is the live
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
- **Aliased condition**: a `const` whose initializer is a condition, which
  narrows the condition's subjects when the `const` itself is tested, as `ok`
  does in `const ok = x !== null; if (ok) x`.
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
   that needs to know that a point is unreachable gets that answer from a
   separate request, `UnreachableCodeClassificationRequest`, never from the
   read's type.

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
| Falsy branches and `typeof x !== "object"` keep object types that a primitive can have | `interface Len { length: number }`<br>`x: Len \| undefined; if (!x) x`<br>`y: string \| Len \| null; if (typeof y !== "object") y` | `undefined`; `string` | `Len \| undefined`; `string \| Len`: an object type is removed only when no falsy string, number, boolean, or bigint can have it, or, for `!== "object"`, when no primitive and no function can have it, as for `interface Foo { a: number }`. The left operand of `&&` keeps such a type in the value of the expression in the same way. | Every string has `length`, so a string can have type `Len`, and `x` can be `""` in the falsy branch. TypeScript removes every object type there except `{}`, even empty interfaces and classes, which every value except `null` and `undefined` has. Keeping only the object types that a primitive can have is sound, and such types are rare next to object types with a member that no primitive has. |
| `eval` and the `arguments` object do not stop narrowing | `x: string \| null;`<br>`if (x !== null) { eval("x = null"); x }`<br>`if (x !== null) { arguments[0] = null; x }` | `string` | `string`, the same as TypeScript | A direct `eval` can write any variable it can see, including a type-guard function. In a function that is not in strict mode and has no default, rest, or destructured parameter, `arguments[0] = null` also writes the first parameter. Either way, `x` can be `null` at the read. Excluding them would stop narrowing in every function that reads `arguments`, even only `arguments.length`, or that contains a direct `eval`, while code that writes a narrowed variable this way is rare. |
| Calls, callbacks, and writes through other variables keep narrowing | `o: { v: string \| null }; reset: () => void;`<br>`xs: (string \| null)[]; x: string \| null;`<br>`if (o.v) { reset(); o.v }`<br>`if (xs[0]) { xs.shift(); xs[0] }`<br>`if (o.v) { const a = o; a.v = null; o.v }`<br>`if (o.v) { [1].forEach(() => { o.v = null; }); o.v }`<br>`if (x) { [1].forEach(() => { x = null; }); x }`<br>`if (this.x) { this.reset(); this.x }` in a method of a class whose `reset` method sets `this.x = null`<br>`import { y } from "./a"; let m: string \| null = get();`<br>`function f() { if (y) { reset(); y } if (m) { reset(); m } }` | `string` | `string`, the same as TypeScript. A write to `o.v` or `x` in the function's own flow narrows them at the write, and a write inside a function or arrow expression called where it is defined, a class static block, or a field initializer stops their narrowing in the whole function. A write of a variable that the code around the function declares, as `m`, counts only in the function itself. Biome also counts such a function when it is `async`, and a write through `super`, as in `super.x = null` before a read of `this.x`, where TypeScript gives `string` for both. A variable or an array whose type evolves with its writes, as the rows on unannotated `let` declarations and evolving arrays describe, takes the types of the writes in the function's own flow in the same way, so `let n; n = 1; [1].forEach(() => { n = "s"; }); n` is `number` and `const items = []; items.push(1); [1].forEach(() => items.push("s")); items` is `number[]`, as in TypeScript. | `reset` or the callback can set `o.v` or `x` to `null`, and `a.v = null` sets `o.v` through another variable, so they can be `null` at the read. `xs.shift()` moves the next element, which can be `null`, into `xs[0]`, and `this.reset()` sets `this.x` to `null`. An import is live: `reset` can call a function of `./a` that sets `y` to `null`, and it can set the module variable `m` to `null`, as can the code around `f` between two calls of `f`. Stopping narrowing at every call would make it useless, because most tests are followed by a call, and rules would report code that TypeScript accepts. An `async` function called where it is defined runs until its first `await` before the code after the call, so Biome counts its writes. `super.x = null` sets `x` on `this`, or calls a setter of the parent class with `this` as its receiver, so Biome counts it as a write of `this.x`. The callbacks that assign `"s"` to `n` or add it to `items` run before the reads, so `n` is `"s"` and `items` holds `"s"` there. |
| A write through an element access whose key is not a literal | `o: { v: string \| null; w: string \| null };`<br>`k: "v" \| "w";`<br>`if (o.v) { o[k] = null; o.v }` | `string` | `string \| null`: a write through a key that is not a string or integer literal, such as `k`, `-0`, or a `const` with a literal type, gives every property path of the object its declared type at the write. A literal key writes only the property it names, so `o["w"] = null` keeps the narrowing of `o.v`. `delete o["v"]` also gives `o.v` its declared type, where TypeScript gives `string`. | TypeScript ignores the write unless the key is a literal or a `const` with a literal type, although `k` can be `"v"` here, and it ignores `delete` through an element access. Treating such a write as a write is sound, and such writes are rare next to tests of a property. |
| A getter narrows like a field | `class G { get v(): string \| null }`<br>`g: G; if (g.v) g.v` | `string` | `string`, the same as TypeScript | A getter can return a different value each time it runs, so `g.v` can be `null` at the second read. Treating getters as fields keeps narrowing for classes that expose their state through getters, while getters that change between two reads close together are rare. |
| A private name and a string key with the same text | `class K { #x: string \| null = null; "#x": string \| null = null; m() { if (this.#x) this["#x"] } }` | `string`: TypeScript treats `this.#x` and `this["#x"]` as the same reference | `string \| null`: `this.#x` and `this["#x"]` are different paths, so neither narrows the other | A private name is not a property, so `this["#x"]` reads the property named `"#x"`, another value, which can be `null` after the test of `this.#x`. Keeping them apart is sound and loses nothing in code that has no property whose name starts with `#`. |
| A write to a property between a `const` that holds it and the test of the `const` | `type Circle = { kind: "c"; r: number };`<br>`type Square = { kind: "s"; w: number };`<br>`s: Circle \| Square; t: Circle \| Square;`<br>`const k = s.kind; s.kind = "s"; if (k === "c") s`<br>`const ok = s.kind === "c"; s.kind = "s"; if (ok) s`<br>`let u = s; const j = u.kind; u = t; if (j === "c") u` | `Circle` | `Circle \| Square`: a test of a `const` that holds a property, and an aliased condition that tests a property, narrow the object only when the code of the execution root never writes the property or the object's variable. A write after the test counts too. A test of the property itself, as in `if (s.kind === "c")`, still narrows `s` after the write. Calls and callbacks that write the property don't count, as for property narrowing. | `k` holds the value of `s.kind` from before the write, so `s.kind` is `"s"` at the read and `s` matches neither member. TypeScript checks a write to a discriminant against the narrowed object, so code rarely writes a discriminant in a function that also tests it, and the sound result loses little. |
| A property path narrows through an aliased condition only when every member declares it readonly | `o: { readonly v: string \| null } \| { v: string \| null; w: 1 };`<br>`const ok = o.v !== null; clear(o); if (ok) o.v` | `string` | `string \| null`: at every step of the path, every member of the declared type of the object other than `null` and `undefined` must declare the property readonly | TypeScript takes a property of a union as readonly when one member declares it readonly. A function that receives `o` with the same type, such as `clear(x: typeof o) { if ("w" in x) x.v = null; }`, can narrow it to the other member and assign the property, which TypeScript accepts, so `o.v` is `null` at the read. When every member declares the property readonly, code can assign it only through a value typed otherwise, and unions that declare a property readonly in some members only are rare. |
| A rest element in a destructuring `const` holds no property | `r: { ok: true; v: number } \| { ok: false; e: string };`<br>`const { ...ok } = r; if (ok) r` | `{ ok: true; v: number }`: TypeScript takes `ok` as the property `r.ok` | The declared type of `r` | `ok` holds a new object with the other properties of `r`, which is always truthy, so `r` can have `ok: false` at the read. |
| `in` treats object types as closed | `function f(x: { a: 1 } \| { b: 2 }) { if ("a" in x) x.a }`<br>`const v = { a: "s", b: 2 as const }; f(v)` | `1` | `1`, the same as TypeScript: `"a" in x` keeps `{ a: 1 }` and removes `{ b: 2 }` | `{ b: 2 }` also describes objects with other properties, so `f(v)` passes an object whose property `a` is `"s"`, and `x.a` is `"s"` at the read. Keeping `{ b: 2 }` in the true branch, as `{ b: 2 } & Record<"a", unknown>`, would make `in` useless for telling the members of a union apart, while code rarely passes an object whose type leaves out a property that the code tests with `in`. |
| The true branch of `instanceof` removes members that are not derived from the class | `class A { a = 1 }; class B extends A { b = 2 }`<br>`x: B \| { c: number }; if (x instanceof A) x`<br>`const v: { c: number } = Object.assign(new A(), { c: 1 })` | `B` | `B`, the same as TypeScript: the true branch keeps the members derived from the class and removes object type literals, functions, and other types that are not | `{ c: number }` also describes an instance of `A` that has a property `c`, such as `v`, so `x` can be an instance of `A` that is not a `B` at the read. Keeping such members would make `instanceof` useless for telling the members of a union apart, while code rarely holds an instance of a class in a variable typed by an unrelated object type. |
| The true branch of a type-guard function removes members that are not related to the target | `interface Foo { a: number }; interface Foo2 extends Foo { b: number }; interface Bar { c: number }`<br>`function isFoo(v: unknown): v is Foo;`<br>`x: Foo2 \| Bar; if (isFoo(x)) x.b`<br>`const o = { a: 1, c: 2 }; const v: Bar = o` | `Foo2` for `x`, and `number` | The same as TypeScript: the true branch keeps the members that are subtypes of the target or that the target is a subtype of, and removes the others | `Bar` also describes objects that have a property `a`, such as `v`, which `isFoo` accepts although it is not a `Foo2`, so `x.b` is `undefined` at the read when `x` is `v`. Keeping such members would make type-guard functions useless for telling the members of a union apart, while code rarely holds an object whose type leaves out a property that the type-guard function tests. |
| The false branch of `instanceof` removes the class and its derived types | `class A { a = 1 }`<br>`x: A \| string; if (!(x instanceof A)) x`<br>`const v: A = { a: 1 }` | `string` | `string`, the same as TypeScript | TypeScript accepts a plain object for a class type whose members are all public, as `v` shows, so `x` can hold such an object, which is not an instance of `A`, and still be an `A` in the `else` branch. The sound result would remove nothing in the `else` branch, while code rarely types a plain object by a class. |
| A case of `switch (true)` that fails to match narrows only when it gives a boolean | `x: { a: 1 } \| null;`<br>`switch (true) { case x: break; default: x }` | `null`: TypeScript takes every case before `default` as falsy | `{ a: 1 } \| null`: a case narrows where it fails to match only when its value is `true` whenever it is truthy: a comparison, `instanceof`, `in`, `!`, a call, `a && b` with such a `b`, or `a \|\| b` with both such. A case narrows where it matches as its test does when it is truthy, as in TypeScript. | A case matches only a value that is `true`, so `default` also runs for an object, which is truthy but not `true`, as for `x` here. Idiomatic `switch (true)` code tests comparisons and type-guard functions, which give booleans and narrow as in TypeScript, so the sound result loses little. |
| Destructuring evaluates its pattern before its value | `x: string \| number; a: unknown;`<br>`if (typeof x === "string") { [a = (x = 1)] = [x] }`<br>`if (typeof x === "string") { const [b = id(x = 1)] = [x] }` | `number` at the value of the assignment; `string \| number` at the initializer of the declaration | The same as TypeScript. A destructuring assignment evaluates its pattern, with the defaults and the objects and keys of its targets, before its value, and its defaults unconditionally, then writes its targets. A declaration evaluates its pattern, with its defaults only for `undefined`, before its initializer. | JavaScript evaluates the value first, and a default only when the field is `undefined`, so `x` is a `string` where the value reads it. Following the runtime order would give a type disjoint from TypeScript's, as `string` where TypeScript gives `number`, which rules would report against. The difference needs a default that writes a variable that the value or the initializer reads. |
| A `let` read in a nested function counts as assigned once some code assigns it | `declare const c: boolean;`<br>`let x: string \| null; if (c) x = "a";`<br>`const g = () => { if (x !== null) x };`<br>`function h() { if (x !== null) x }` | `string` at both reads | `string` at both reads, the same as TypeScript: a `let` without an initializer starts with `undefined` added to its declared type in the code that declares it, but in a function nested in that code, including a closure that continues the flow where it is created, only when nothing assigns it. | When `c` is false, `x` is still `undefined` when `g` or `h` runs, so `x !== null` holds and `x` is `undefined` at the read. A variable that one function assigns and others read is common, as `let server: Server; function start() { server = listen(); }`, and keeping `undefined` in every function that doesn't assign it would type such reads differently from TypeScript, while code rarely runs a function before the variable it reads is assigned. |
| Code between two elements of a destructuring pattern keeps the narrowing | `s: { kind: "c"; x?: number; v: number } \| { kind: "s"; x?: number; v: string };`<br>`const { kind, x = reset(s), v } = s; if (kind === "c") v` | `number` | `number`, the same as TypeScript. A computed key between the elements, as in `{ kind, [key()]: x, v }`, keeps the narrowing too. | Destructuring reads `kind`, then runs the default of `x` when `s.x` is `undefined`, then reads `v`, so `reset` can change `s` before `v` is read and make `v` a string at the read. A computed key runs code at the same point. Defaults and computed keys rarely write the value being destructured, while stopping the narrowing at every default or computed key that can run code would lose it in patterns such as `{ kind, items = load(), v }`. |

## Limits

These limits apply to every *Supported* and *Partial* row below:

- A read narrows only when it reads a parameter, a `catch` variable, an
  import, or a variable declared with `let`, `const`, `var`, or `function`.
  It narrows by the tests and writes of its own execution root. A variable
  that an enclosing execution root declares, such as a module variable or an
  import read in a function, starts there from its declared type, whatever
  the enclosing root tested, unless the read is in a closure that continues
  the flow where it is created, as the rows on closures describe: a function
  or arrow expression, or a method or accessor of an object literal or a
  class expression, which starts a `const` that is not an import, and a
  parameter, a `catch` variable, or a `let` that it reads after the last
  assignment, from its type where the closure is created. A read in a
  function declaration, in a method, accessor, or constructor of a class
  declaration, and in a constructor or static block of any class never sees
  facts from the enclosing execution root, and a read in a class field,
  including its decorators and computed key, never narrows. A variable that a destructuring default can
  supply, as in `const { a = 0 } = o`, never narrows, because its declared
  type ignores the default, and neither does a `var` whose name another
  declaration of its scope declares, as a second `var`, a function, or a
  parameter of its function does, because Biome resolves every reference to
  the last declaration while each declarator writes the variable.
- A write in the execution root's own flow narrows the variable or the
  property path that it writes, as the rows on assignments describe. A
  variable or a path doesn't narrow anywhere in its execution root when code
  that runs with the root, but outside its flow, writes it: a function or
  arrow expression called where it is defined, a class static block, a field
  initializer, a decorator, a computed key, or a parameter default of a
  method. So do a write in unreachable code and in the initializer of an
  exported declaration, which Biome doesn't evaluate, and a write through a
  target whose object Biome doesn't map to a variable or a path, as in
  `(o as T).v = x`, `o!.v = x`, or `super.v = x`. Writes in the body of any
  other function, method, accessor, or constructor don't count, and neither
  do writes through another variable, or writes outside the root of a
  variable that an enclosing root declares. A written `let` or `var` without
  a type annotation whose initializer is missing, `null`, or `undefined`, and
  a `let`, `const`, or `var` without a type annotation initialized with `[]`
  that the root writes or adds values to, take their types from those writes
  in the execution root that declares them, as the rows on unannotated `let`
  declarations and on evolving arrays describe, and keep their declared
  types in other execution roots, including closures.
- A statement call ends the path only where Biome finds the signature that
  TypeScript finds, as the rows on calls that never return describe: a
  generic or overloaded signature ends nothing, and neither do the statement
  calls of an execution root after its first 64. Biome then evaluates the
  code after such a call as reachable, where TypeScript finds it unreachable
  and gives a read there its declared type, so a read there can be narrower
  than in TypeScript: with `x: string | null` and
  `declare function fail<T>(): never`, `x` is `string` in Biome and
  `string | null` in TypeScript in `if (x !== null) { fail(); x }`. So is a
  read in a closure created after a call to a function that the module
  doesn't declare, inside a function or arrow expression that Biome doesn't
  analyze, as `g` in
  `if (x !== null) { const g = () => { log.fatal(); return () => x; }; }`.
- A property path, such as `o.v`, `a.b.c`, or `this.v`, narrows only when it
  starts at a variable that can narrow by the rule above, including a module
  variable read in a function, or at `this` where the next item allows it.
  Its declared type at a read is the property's type on the narrowed type of
  its object at that read. A path is made of named properties and
  private names, such as `v` and `#v`, written with `.` or `?.`, and of
  element accesses whose key is a string literal without escapes or a decimal
  integer literal from 0 to 2^53 - 1 without leading zeros or separators,
  such as `o["v"]` or `t[0]`. `o["v"]`, `o?.v`, and `(o).v` are the same path
  as `o.v`, and `t[0]` is the same path as `t["0"]`, but `o["#v"]` is never
  the same path as `o.#v`. Element access with any other key does not narrow
  yet. A runtime test of a chain that starts at a parenthesized optional
  chain, such as `(o?.v)?.w === 1`, narrows nothing.
- A path on `this` narrows in a class method or accessor that is not
  `static`, in an object literal method or accessor, and in an arrow function
  whose `this` comes from one of them or from the initializer of a field that
  is not `static`. The method or accessor must not declare a `this`
  parameter, and its class or object must have a name that Biome can find: a
  class declaration, a class expression with its own name or assigned as the
  initial value of a variable, as in `const C = class {}`, or an object
  literal assigned as the initial value of a variable, as in
  `const o = { ... }`. So a path on `this` does not narrow in an object
  literal passed to a call. It narrows in a constructor and in an arrow
  function created in one, where a field without an initializer may be read
  before it is assigned, as the row on such reads describes. It never narrows
  in a `static` method, accessor, field, or block, in a function declaration
  or expression, or outside any function. Neither
  does a path on `this` in the decorators or `extends` clause of a nested
  class or in the computed key of a nested object literal method, which Biome
  types as the nested class or object.
- A write to a prefix of a property path, as `o = q` or `o.v = q` for
  `o.v.w`, gives the path its declared type at the write, and so does a write
  of any property of a prefix through a key that is not a literal of the kind
  above, such as `o[k] = x` or `o[-0] = x`. A write through such a literal,
  such as `o["v"] = x`, writes only the path it names. Assignments of every
  kind, `++` and `--`, destructuring targets, the targets of `for...in` and
  `for...of`, and `delete` count as writes. A write that runs with the root
  outside its flow, as the item above lists, stops the path from narrowing
  anywhere in the root. Every `this` and `super` in such writes stands for
  the root's `this`, even where it is another object, as in a static block,
  so `this.v = x` and `super.v = x` both write `this.v`.
- The type that a write assigns is the type of its value without narrowing,
  so `y = x` after `if (x)` assigns the declared type of `x`. An object or
  array literal keeps the literal types of its properties there, as
  TypeScript keeps them where the declared type of the target has literal
  types. A call reduces when its callee's return type comes from an
  annotation: a function with a return type annotation, a variable or
  parameter annotated with a function type, a global, an import of such a
  function or of a declaration in a declaration file, or a method that every
  type of its object, typed without narrowing, gets from such a declaration.
  That is a method of a primitive, a tuple, a lib global that only
  declaration files merge interfaces into, a type that a declaration file
  declares and whose parents are such types, or a class or an interface
  that declares the method with a return type annotation, or declares none
  and inherits it from such a type. A call also reduces when its callee is a
  local function without a return type annotation, that is a function
  declaration, or a function or arrow expression that initializes a variable
  without a type annotation, and isn't a generator, when each value that the
  function returns reduces by these rules and reads no variable whose type
  evolves with its writes. Biome follows such calls through at most four
  nested functions, and infers their return types as TypeScript does, with
  `undefined` when a path reaches the end of the body, so
  `function g() { if (c) return 1; }` returns `1 | undefined`. Other calls,
  such as one to a method without an annotation, to an overloaded method
  whose implementation has none, to a method of an object literal, to an
  imported function whose return type is inferred, or to a function
  expression called where it is defined, a call along an optional chain
  whose object is such a call, as `o.m()?.trim()`, and `typeof`, which Biome
  types as one kind, give the variable its declared type, where TypeScript
  reduces by the return types that it infers.
- Testing an aliased condition applies the condition that initializes it only
  to variables whose value cannot change between the `const` and the test:
  parameters, `catch` variables, and variables declared with `const` or
  `let`, if they are never written and not exported. A `let` statement at the
  top level of a file does not count, because other scripts can write a
  top-level `let` of a script, and Biome cannot tell a script from a module.
  A property path, such as `o.v`, `o.a.v`, or `this.v`, narrows through an
  aliased condition only when it starts at such a variable or at `this`,
  and every property along it is readonly: at every step, every member of
  the declared type of the object, other than `null` and `undefined`,
  declares the property itself as readonly, with `readonly`, `Readonly<T>`,
  or `as const`. TypeScript needs this in one member only, as the
  deliberate choices above describe. A path on `this`
  narrows only through an aliased condition that the same function
  declares. A test of a property in an aliased condition still narrows the
  variable that holds the property, as in
  `const ok = s.kind === "c"; if (ok) s`, whether the property is readonly
  or not, unless the execution root writes the property, as the deliberate
  choices above describe. As in TypeScript,
  aliased conditions inline into one another at most five
  levels deep, so in `const c2 = c1; ...; const c6 = c5; if (c6)`, only the
  truthiness of `c1` is tested.
- A property path whose properties are readonly in TypeScript keeps its
  declared type through an aliased condition in these cases:
  - A property along it is a getter, because Biome doesn't collect setters,
    so a getter may have one; a `static readonly` field; an element of an
    array under `as const`, which Biome types as a tuple whose elements
    aren't readonly; or a property that the type of the object inherits
    through `extends`, including the `extends` clause of an interface
    merged into a class, or that a module augmentation declares, because an
    augmentation in a file that Biome doesn't index can declare the property
    on the type itself without `readonly`.
  - An object along it is a generic, or a primitive, as for `s.length`.
  - The declared type of an object along it has a member that doesn't
    declare the property readonly, even where narrowing has removed that
    member at the read, as `if (o.kind === "a")` does in
    `o: { kind: "a"; readonly v: string | null } | { kind: "b"; v: string | null }`.
    Biome decides from the declared type, while TypeScript uses the type at
    the read.
  - The execution root tests an object along the path, anywhere, with
    `instanceof`, a type-guard function, or `typeof` with a kind other than
    `"object"` and `"undefined"`. Such a test can replace a member of the
    object's type with a type whose property TypeScript doesn't take as
    readonly, as `o instanceof Derived` does for `class Derived extends Base`
    when `Derived` redeclares a readonly property of `Base` without
    `readonly`.
  - The path starts at `this` in an arrow function and the aliased
    condition is declared in the enclosing method.

  The elements of a `readonly` tuple count as readonly, so `t[0]` in
  `t: readonly [string | null]` narrows through an aliased condition. A
  property of an intersection is readonly only when every operand that
  declares it declares it readonly, as in TypeScript.
- A runtime test of a property, such as `s.kind === "c"`, `s["kind"]`, or
  `t[0] === "a"` on a tuple, also narrows the object that holds the property,
  as TypeScript narrows a discriminated union. Only that object narrows:
  `o.a.kind === "c"` narrows `o.a`, not `o`. The object must be a variable or
  a property path that can narrow by the rules above, not `this`, and its
  declared type must be a union with at least two object types in which
  every member other than `null` and `undefined` declares the property. The
  property must be a discriminant: in at least one member, its type is a
  literal type, `null`, `undefined`, `boolean`, or a union of them, and the
  members don't all give it the same type. An instance of a generic alias,
  such as `r: Result<number>` with
  `type Result<T> = { ok: true; value: T } | { ok: false }`, is not a union
  yet, so it keeps its declared type where TypeScript narrows it. A member
  whose property type is an object type stays after every test that leaves a
  value other than `null` or `undefined`, because Biome does not compare
  object types with other types, so with
  `o: { k: { a: 1 }; x: 1 } | { k: "x"; y: 2 }`, `o.k === "x"` keeps both
  members, where TypeScript keeps only the second. A call to a type-guard
  function on the property, as in `isC(s.kind)`, does not narrow the object,
  as in TypeScript. A write to the property, as in `s.kind = "b"`, keeps the
  object narrowed, because the written value must fit the property of the
  narrowed object. A test of a `const` that holds the property, as in
  `const k = s.kind; if (k === "c")` or `const { kind } = s; if (kind === "c")`,
  narrows the object in the same way, as the row on discriminants held in a
  `const` describes.
- A runtime test of an element of a destructuring pattern, such as `kind` in
  `const { kind, v } = s` or in `function f({ kind, v }: S)`, also narrows
  the value that the pattern destructures, as a test of `s.kind` narrows `s`,
  and a read of another element of the same pattern reads its property on
  the narrowed value, as TypeScript does. So with
  `s: { kind: "c"; v: number } | { kind: "s"; v: string }`,
  `if (kind === "c") v` reads `v` as `number`. The pattern must be declared
  by a `const`, including the head of a `for...of` loop, or by a parameter,
  including a rest parameter such as `...[k, x]`, and no variable that the
  declaration binds may be written. The tested and the read elements must be
  direct elements of the same pattern that have no default and are not rest
  elements, so in `const { a: { kind, v }, b } = w`, a test of `b` doesn't
  narrow `v`, and in `const { kind, ...rest } = s`, a test of `kind` doesn't
  narrow `rest`. An element of an array pattern from index 16 on neither
  narrows the pattern nor holds a discriminant, where TypeScript has no such
  limit. A nested pattern narrows only when no enclosing pattern
  supplies a default, as `{ a: { kind, v } = d }` does. The destructured
  value narrows as the object of a test of its property does, by the rules
  above, except that its `null` and `undefined` members don't count, because
  destructuring throws on them. The element's own tests then narrow the
  element further, as a property path's own tests do.
- `"a" in x` and `0 in x` narrow `x` when its declared type is a union with
  at least two object types, and `x` is a variable, `this`, or a property
  path that can narrow by the rules above. The key must be a string literal
  without escapes or a decimal integer literal of the kind that a property
  path accepts. As in TypeScript, the true branch keeps the members that
  declare the property, even as optional, or whose index signature accepts
  its name, and removes the others only when some member does, and the false
  branch removes the members that declare the property as required. A member
  stays in both branches when Biome can't tell whether it has the property,
  as the row on `in` lists. Only the value that `in` looks into narrows:
  `"a" in s.k` narrows `s.k`, not `s`, as in TypeScript. The deliberate
  choices above describe why object types count as closed.
- `x instanceof C` narrows `x` when `x` is a variable or a property path that
  can narrow by the rules above, and its declared type is `unknown` or has a
  member other than `null`, `undefined`, and primitive types, because no
  primitive is an instance of a class. Only the tested value narrows:
  `o.v instanceof C` narrows `o.v`, not `o`, and a test of an element of a
  destructuring pattern doesn't narrow the other elements. In an optional
  chain, as in `o?.v instanceof C`, the true branch also removes `null` and
  `undefined` from `o`, as in TypeScript. The instance type of `C` is an
  instance of `C` when `C` is a class, and otherwise the type of its
  `prototype` property or the return types of its construct signatures, as
  `new () => Foo` gives `Foo`. A union of constructors gives the union of
  their instance types. As in TypeScript, the members of the type of `x` are
  compared with the instance type by their declarations: a class or an
  interface is derived from another when its `extends` clauses lead to that
  declaration, whatever their type arguments, so `Box<string>` is derived
  from `Box`, and arrays and tuples are derived from `Array`. An object type
  literal, a function type, or a primitive is derived from no class. The true
  branch keeps the members derived from the instance type, replaces a member
  that the instance type is derived from with the instance type, and removes
  the others. When no member remains, it gives the instance type if that is a
  subtype of the type of `x`, as `A` is of `{ a: number } | string` with
  `class A { a = 1 }`, and keeps the type of `x` otherwise, where TypeScript
  gives an intersection such as `(string | number) & A`. `unknown` becomes
  the instance type. The false branch removes the members derived from the
  instance type when `C` has a single instance type that is an object type
  other than `{}`. The deliberate choices above describe why each branch
  removes members that a value reaching it can have.
- `x instanceof C` keeps the type of `x`, where TypeScript narrows it, when
  `C` is `Object` or `Function`, when its type is `any` or has a generic
  construct signature, as in `new <T>() => Box<T>`, when `C` declares or
  inherits a static `[Symbol.hasInstance]` method, or a static member with a
  computed key that Biome can't name, and when a member of the type of `x` is
  a type parameter, as in `x: T | C`. An instance of a generic class has no
  type arguments in Biome, where TypeScript gives it `any` ones, as in
  `Map<any, any>`, so `unknown` stays `unknown` for such a class, a member
  that its instance type is derived from stays as it is, and the true branch
  keeps the type of `x` when no member remains. Two classes with the same
  name and members in two modules, such as `class K { k = 1 }` in `./a` and
  in `./b`, can't be told apart from one class that Biome reaches through two
  modules, so a test of one of them keeps the type of a value of the other.
  Two such classes in two namespaces of one module, as in
  `namespace P1 { export class K { k = 1 } }` and
  `namespace P2 { export class K { k = 1 } }`, are two classes, as in
  TypeScript, so `x: P1.K | string` keeps `P1.K | string` in the `else`
  branch of `x instanceof P2.K`. When no member remains in the true branch,
  Biome doesn't count an instance of a class as a subtype of an instance of
  another class with the same members, because a class can declare private
  members, so the true branch keeps `P1.K | string`, where TypeScript gives
  `P2.K`.
- A call narrows through its callee only when the callee is a dotted name: an
  identifier or `this`, followed by property accesses with `.` or `?.`, in
  parentheses or not, as `isString(x)`, `u.isString(x)`, or
  `(this.check)(x)`. A call through an element access, as
  `u["isString"](x)`, or through the result of a call, as `get()(x)`,
  narrows nothing, where TypeScript narrows a call in a condition through
  any callee whose type has a single signature with a type predicate. The
  call must pass a subject as the tested argument, or call a method of a
  subject that a `this is T` predicate tests.
- A call to a type-guard function whose target is not a primitive type, as
  `isFoo(x)` with `x is Foo`, compares each member of the type of `x` with
  each member of the target in TypeScript's subtype relation, as TypeScript
  does. The true branch keeps a member that is a subtype of a member of the
  target, such as `Foo2` with `interface Foo2 extends Foo`, replaces a member
  with a member of the target that is a subtype of it, as `unknown` or
  `object` becomes `Foo`, and removes the others. When no member remains, it
  gives the target if that is a subtype of the type of `x`, and keeps the
  type of `x` otherwise, where TypeScript gives an intersection such as
  `(string | number) & Foo`. The false branch removes the members that the
  true branch keeps as they are, and keeps `unknown`. A primitive is compared
  with an object type through the members of its wrapper type, so `string`
  is a subtype of `{ length: number }`.
- A type-guard function keeps the type of `x` in both branches where
  TypeScript narrows it in these cases:
  - The target has only object types and the type of `x` has none and isn't
    `unknown`, as `string | number`, which TypeScript can narrow to a
    primitive, as to `string` with `{ length: number }`, or to an
    intersection.
  - Biome can't tell whether one of two members is a subtype of the other in
    the way that decides the result: for a generic, an index signature,
    `unknown` in a union, instances of two different built-in classes, as
    `TypeError` and `Error`, and an instance of a class that is compared with
    a type that isn't derived from the class, because a class can declare
    private members. So `x: L | string` with `class L { length = 0 }` keeps
    its type in both branches of `isL(x)`. So does an instance of a class
    with the same name and members as the target class in another namespace,
    which is another class, or in another module, which Biome can't tell
    from the target class.
  - A property of an object literal or a class field keeps a literal type
    where TypeScript widens it, because Biome can't tell that its value is a
    fresh literal: the result of a generic call, as `1` in `{ a: id(1) }`, a
    read of a `readonly` field or of a constant of a namespace, as `A.X`, and
    an `await`. Biome widens the literal types of other values, as
    TypeScript does. Such a property is compared both ways, and Biome can't
    tell when the results differ. So `const o = { a: id(1) }` keeps its type
    `{ a: 1 }` in both branches of `isA1(o)` with `v is { a: 1 }`, where
    TypeScript gives `{ a: 1 }` and `{ a: number }`.
- The false branch of a type-guard function also keeps a member of the type
  of `x` where TypeScript removes it in these cases:
  - The member and a member of the target are subtypes of each other and not
    the same type. TypeScript's strict subtype relation, which depends on
    modifiers that Biome doesn't compare, such as `readonly`, decides whether
    the true branch keeps the member as it is. So with
    `interface Same { a: number }`, `x: Same | Bar` keeps `Same` in the false
    branch of `isFoo(x)`, where TypeScript gives `Bar`, and so does an
    instance of a class that is a subtype of the target, as `A` with the
    target `Foo`.
  - The member is `unknown`, which TypeScript narrows to `null | undefined`
    with the target `{}`.
  - The true branch keeps no member, as for `(string | number) & Foo`. The
    false branch is then the type of `x`, which TypeScript makes `never` when
    the type of `x` is assignable to the target.
- A literal type that the true branch of a type-guard function keeps, and
  whose primitive type the true branch also gets, as `"a"` and `string` from
  `x: "a" | { length: number }` with `v is string`, stays in the false
  branch, as in TypeScript, whose union of the two is the primitive type. A
  target of `any` or `unknown` narrows nothing.
- Interfaces with the same name in one scope merge into one type, as in
  TypeScript, and so do a class and the interfaces with its name: the type
  has the members and the `extends` clauses of every declaration, and the
  first declaration of a property decides its type and whether it is
  readonly. A method that several declarations declare has the overloads of
  all of them, and a call tries those of a later declaration first, and
  those with a parameter of a literal type, such as `"a"`, before all others,
  as in TypeScript. Within one declaration, Biome tries the overloads in
  their order, so with `interface O { f(x: string): 2; f(x: "a"): 1 }` and
  `o: O`, `o.f("a")` gives `2`, where TypeScript gives `1`. Interfaces with
  the same name in two blocks of
  one namespace, as in `namespace N { export interface I { a: 1 } }` and
  `namespace N { export interface I { b: 2 } }`, don't merge yet: `N.I` is
  the first of them. A declaration in a `declare global` block merges only
  with those in other such blocks. Declarations whose type parameters
  differ, which TypeScript reports (TS2428) unless a declaration lists only
  the first ones of another and leaves out ones with defaults, give
  Unknown, and so do interfaces that give a class a construct signature.
  Two classes with the same name (TS2300) keep their own types.
- The interfaces of a module augmentation, such as
  `declare module "./shapes" { interface X { extra: 1 } }`, merge into the
  interface or class that the augmented module exports, also through
  `export *` and named re-exports, and so do the interfaces of a
  `declare global` block, and the top-level interfaces of a declaration file
  (`.d.ts`) without imports or exports, into the global of their name, as in
  TypeScript. Their members count after the target's own, so the target's
  own declaration of a property decides its type and whether it is readonly,
  and the overloads of their methods are tried before the target's, those
  of the augmenting module with the last path first, where TypeScript
  follows the order of the program's files. They merge from every module
  that Biome indexes, which can be more or fewer files than the program of a
  project's `tsconfig`. An augmentation in a file that the program leaves
  out, such as a file outside its `include`, can make the false branch of
  `instanceof` and of a type-guard function narrower than that program's
  TypeScript: with `declare module "./shapes" { interface X extends Base {} }`
  in such a file, `x: X | C` gives `C` in the false branch of
  `x instanceof Base`, where TypeScript gives `X | C`.
- Only interfaces merge from these declarations: namespaces and variables
  that a `declare global` block or a module augmentation declares, as
  `namespace NodeJS { interface ProcessEnv {} }` or `var __DEV__: boolean`,
  are not seen, and neither are those of a TypeScript file without imports
  or exports that isn't a declaration file. An interface that augmentations
  add to a module that doesn't export its name is seen through imports of
  that module, but not through modules that re-export it or through
  `import * as`. A global that the standard library doesn't declare, and
  that more than one module declares, is Unknown outside the modules that
  declare it, and each of them sees only its own declarations.
- Falsy and `typeof` tests compare an object type with the types that give
  primitives and functions their members, such as `String` and `Function`,
  from TypeScript's ES2015 library files and the interfaces that the project
  merges into them.
- Variables whose declared type is `any`, or includes a part that inference
  cannot determine, never narrow.
- An execution root gets no narrowing at all when its own code, outside nested
  functions, contains a `with` statement, a syntax error, or a `break` to the
  label of a `try` statement, as in `out: try { break out; } finally {}`, or
  when it is very large or deeply nested.
- Class and enum declarations, namespaces, `import x = require()`, and
  `declare` statements do not stop narrowing. A read in a class or enum
  declaration, such as in its `extends` clause or in the initializer of an
  enum member, has the type from just before the declaration, narrowed
  further only by the rest of the expression that contains the read, as in
  `x && x.length`.
- A comparison with `===`, `!==`, or a `case` narrows by a unit value, a
  value whose type TypeScript gives a single value, as its `isUnitType`
  decides: a string, number, bigint, or boolean literal, `null`,
  `undefined`, `void` with any operand, `-` or `+` directly before a number
  literal, as in `-1`, and a template literal without a tag and without
  substitutions, as `` `a` ``. A variable holds a unit value when it is a
  `const` without a type annotation whose initializer is one, also through
  other such `const`s, as `K2` in `const K = "a"; const K2 = K`, at most 16
  of them, or when a named or default import binds it to such a `const` of
  another module, also through re-exports. TypeScript gives `-(1)` the type
  `number`, and Biome gives `-1n` the type `bigint`, so neither is a unit
  value in Biome. Loose `==` and `!=` narrow only by `null`, `undefined`,
  and `void`, and a comparison with an imported value narrows no receiver of
  an optional chain, where TypeScript narrows it as for a local value. Other
  values narrow nothing, even where TypeScript gives them a unit type, such
  as a variable with a type annotation, a call, `"a" as const`, a member of
  a namespace, or an enum member. A `const` that a condition narrows to
  `never`, as `D` in `const D = false; if (D) { if (x === D) x }`, still
  holds its value in Biome, so `x` reads as `false` there, where TypeScript
  gives `boolean`: Biome is narrower than TypeScript, in code that
  TypeScript types as impossible.
- A `switch` statement without `default` is exhaustive when every value that
  its discriminant can have at the statement matches a case, as in
  TypeScript. Biome finds one exhaustive only when every case is a unit
  value, as the item above describes, or, for `switch (typeof x)`, a string
  literal or a template literal without substitutions that names a kind of
  value, and when the discriminant is a variable or a property path, or
  `typeof` of one without parentheses around `typeof`. A case that compares
  with an imported value counts only when that value is a unit value. Then
  the code after the statement sees no value that matches no case, so a read
  of another variable there keeps the narrowing of every case. A
  discriminant that TypeScript narrows to `never` before the statement,
  through a comparison that Biome doesn't narrow by, such as one with an
  enum member in `if (x === E.A) return;`, makes Biome find such a statement
  exhaustive where TypeScript doesn't, because no type that TypeScript
  checks for exhaustiveness is `never`. Biome then reads another variable
  after the statement narrower than TypeScript does, in code that TypeScript
  types as impossible. Each execution root counts only its first 64 such
  statements; later ones never count as exhaustive.

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
| `boolean` splits into `true` and `false` | `x: boolean \| undefined; if (!x) x`<br>`y: boolean; if (y !== true) y` | `false \| undefined`; `false` | Supported |
| The falsy branch removes object types | `interface Foo { a: number }`<br>`x: Foo \| undefined; if (!x) x` | `undefined` | Deliberately different: removes `object`, functions, tuples, object literals, instances of built-in classes, and every interface, object type literal, and class instance that no falsy string, number, boolean, or bigint can have, such as `Foo`. Keeps those that one can have, such as `{ length: number }`, empty interfaces, and empty classes, where TypeScript keeps only `{}`; see the deliberate choices above. Also keeps object types that Biome cannot compare with primitives, as listed for `typeof` with a primitive kind, and generics, where TypeScript removes `T extends Foo`. |
| `!`, `!!`, and parentheses | `x: string \| null; if (!!x) x` | `string` | Supported |
| `&&` and `\|\|` in conditions and as operands | `x: string \| null; y: number \| undefined;`<br>`if (x && y) x`; `x && x.length` | `string` at both reads | Supported |
| The branches of a conditional expression | `x: string \| null; x ? f(x) : g(x)` | `string` in `f(x)`, `string \| null` in `g(x)` | Supported |
| A conditional expression has the types of both branches, whatever its test | `interface Foo { a: number }; x: Foo;`<br>`x ? 1 : "s"`; `true ? 1 : "s"`; `x ? 1 : x` | `1 \| "s"` at the first two; `1` at the third, whose `x` is `never` | Supported: a branch adds nothing to the type when its type is `never`, as a call to a function that never returns, or when it only reads the variable that the test reads and the test leaves that variable no value there, as `x` in `x ? 1 : x`. Another read that TypeScript narrows to `never` in a branch, as `x` in `x !== null ? 1 : x` with `x: string`, is Unknown in Biome, which makes the whole expression Unknown. |
| Truthiness of a property narrows the property | `o: { v: string \| null }; if (o.v) o.v` | `string` | Supported |
| Truthiness of a generic gives `NonNullable<T>` | `function f<T>(x: T) { x && x }` | `NonNullable<T>` at the right operand | Not yet |
| A constant condition makes a branch unreachable | `x: string \| null;`<br>`if (x === null) { if (true) return; } x` | `string`; the branch of `if (false)` is unreachable (TS7027) | Supported: as TypeScript's binder decides, only the literal `true` or `false` itself, without parentheses, makes the branch that it never takes unreachable, in `if`, `while`, `do...while`, `for`, a conditional expression, and an operand of `&&` or `\|\|`, so `if (c && false)` never runs its body and `while (c \|\| true)` exits only by `break`. A read there keeps its declared type, and the branch adds nothing where it joins the others. `!false`, `(false)`, `0`, a `const` that holds `false`, and the operands of `??` are ordinary conditions, so `while (1)`, `while ((true))`, and `do...while (1)` can exit by their test, as `for (;;)` can't. |

### Equality

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| `=== null` and `!== null` | `x: string \| null; if (x !== null) x` | `string` | Supported |
| `=== undefined` and `!== undefined` | `x: string \| undefined; if (x === undefined) return; x` | `string` | Supported |
| `== null` and `!= undefined` match both nullish values | `x: string \| null \| undefined; if (x != null) x` | `string` | Supported |
| Strict equality with a string, number, bigint, or boolean literal | `x: "a" \| "b"; if (x === "a") x` | `"a"`; `"b"` in the `else` branch | Supported |
| A broad primitive becomes the literal | `x: string; if (x === "a") x` | `"a"` | Supported |
| Equality with a negative number, `void 0`, a template literal, or a `const` of a literal type | `x: number; if (x === -1) x`<br>`x: string \| undefined; if (x !== void 0) x`<br>`const K = "a"; x: "a" \| "b"; if (x === K) {} else x` | `-1`; `string`; `"b"` | Partial: a comparison narrows by every value that the limits above list as a unit value, as by a literal, in both branches, including a `const` imported from another module. TypeScript narrows by any value whose type is a unit type, where Biome keeps the declared type: a variable with a type annotation, as `k` in `k: "a"`, `"a" as const`, a call that returns a literal type, a template literal with substitutions, a member of a namespace, `export default "a"`, and `-1n`, which Biome types as `bigint`. |
| Equality with an enum member | `enum E { A, B }; e: E; if (e === E.A) e`<br>`x: 0 \| 1; if (x === E.A) x` | `E.A`; `0` | Not yet: Biome doesn't infer the types of enums, so a variable of an enum type is unknown, and an enum member is no unit value. |
| Loose `==` with a literal | `x: "a" \| "b" \| null; if (x == "a") x` | `"a"` | Not yet: loose equality narrows only with `null`, `undefined`, `void`, or a `const` that holds one of them. |
| Equality with another variable | `x: string \| number; y: "a"; if (x === y) x` | `"a"` | Not yet: Biome compares only with the unit values that the limits above list, which leave out a variable with a type annotation, such as `y`. Where the operands are equal, TypeScript also narrows by a variable whose type is not a unit type, keeping the members that the other operand's type can equal. |
| Equality with the result of a runtime test | `x: string \| number;`<br>`if ((typeof x === "string") === true) x` | `string` | Not yet |
| `unknown` compared with a literal | `x: unknown; if (x === "a") x` | `"a"` | Supported |
| `unknown` compared with `null` loosely | `x: unknown; if (x != null) x` | `{}` | Not yet |
| `unknown` compared with an object | `x: unknown; o: { a: number }; if (x === o) x` | `object` | Not yet |

### `typeof`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Primitive kinds, in both branches | `x: string \| number; if (typeof x === "string") x` | `string`; `number` in the `else` branch | Supported |
| Reversed operands | `x: string \| number; if ("string" === typeof x) x` | `string` | Supported |
| A primitive kind removes object types | `interface Foo { a: number }`<br>`x: string \| Foo; if (typeof x === "string") x` | `string` | Partial: removes the object types that no value of the kind can have, such as `Foo`, and, as TypeScript does, replaces an object type that every value of the kind has with the primitive, so `x: number \| { length: number }` gives `string`. `"undefined"` removes every object type. An object type stays when Biome cannot compare it with the primitive: a class instance whose members the primitive has, because the class can declare private members; a type with an index signature; a member whose type differs from the primitive's member of the same name; a member that a TypeScript library newer than ES2015 adds to the primitive, such as `padStart` of strings or `description` of symbols; and `toString`, `toLocaleString`, or `valueOf` for `"bigint"`. Generics stay unchanged. |
| `"object"` keeps object types and `null`; its negation removes them | `x: string \| Foo \| null; if (typeof x !== "object") x` | `string` | Deliberately different: the negative branch removes the object types that no primitive and no function can have, such as `Foo`, but keeps those that one can have, such as `{ length: number }`, where TypeScript removes them; see the deliberate choices above. The positive branch keeps object types with a call signature, such as `interface Callable { (): void }`, which TypeScript removes. |
| `typeof` of `unknown` | `x: unknown; if (typeof x === "object") x`<br>`if (typeof x === "function") x` | `object \| null`; `Function` | Partial: only `"string"`, `"number"`, `"bigint"`, `"boolean"`, `"symbol"`, and `"undefined"` narrow `unknown`. |
| `typeof` of `{}` gives the primitive | `x: {}; if (typeof x === "string") x` | `string` | Supported |
| `"function"` keeps functions and removes object types that no function can have | `interface Foo { a: number }`<br>`interface Len { length: number }`<br>`x: Foo \| (() => void); if (typeof x === "function") x`<br>`y: Len \| string; if (typeof y === "function") y` | `() => void`; `Function` | Partial: in the positive branch, as in TypeScript, an object type that no function can have is removed, one that every function has becomes `Function`, and one with a call or construct signature stays as it is. Object types that Biome cannot compare stay too, as listed for primitive kinds, and so do instances of classes derived from a built-in class. The negative branch keeps object types with a call or construct signature, which TypeScript removes, so `x: CallFoo \| string`, with `interface CallFoo { (): void; foo: string }`, gives `CallFoo \| string` where TypeScript gives `string`. |
| `typeof` of a generic gives an intersection | `function f<T>(x: T) { if (typeof x === "string") x }` | `T & string` | Not yet |
| `typeof` of a property | `o: { v: string \| number }; if (typeof o.v === "string") o.v` | `string` | Supported |
| `typeof` of a discriminant narrows the object | `o: { k: "a"; a: 1 } \| { k: 1; b: 2 };`<br>`if (typeof o.k === "string") o` | `{ k: "a"; a: 1 }` | Supported |
| `typeof x?.y` removes nullish types from `x` | `x: { y: number } \| undefined;`<br>`if (typeof x?.y === "number") x` | `{ y: number }` | Supported |
| `switch (typeof x)` | `x: string \| number;`<br>`switch (typeof x) { case "string": x }` | `string` | Partial: as in TypeScript, every case must be a string literal or a template literal without substitutions, such as `` `string` ``, and a comparison of `typeof x` with such a template literal narrows as one with the string literal does. A case narrows `x` as `typeof x === "string"` does, and the receivers of an optional chain, as in `switch (typeof x?.y)`, only where every case of the group shows that the operand is not `undefined`. As in TypeScript, it narrows neither the object that holds a property, as `o` for `typeof o.k`, nor the pattern that declares a variable. A case with an invalid kind, such as `"strin"`, keeps the type of `x`, where TypeScript gives `never`. |

### `instanceof`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| The true branch | `x: Date \| string; if (x instanceof Date) x` | `Date` | Supported |
| The false branch removes the class and its derived types | `x: Date \| string; if (!(x instanceof Date)) x` | `string` | Supported |
| `unknown` and `any` become the instance type | `x: unknown; if (x instanceof Map) x` | `Map<any, any>` | Partial: `unknown` becomes an instance of a class that is not generic, so `x instanceof Date` gives `Date`, but the example keeps `unknown`, as the limits above describe. `any` never narrows. |
| `Symbol.hasInstance` with a type predicate | `class Checker { static [Symbol.hasInstance](v: unknown): v is string }`<br>`x: unknown; if (x instanceof Checker) x` | `string` | Not yet |
| `x.constructor === C` | `class Foo { a = 1 }; class Bar { b = 2 }`<br>`x: Foo \| Bar; if (x.constructor === Foo) x` | `Foo` | Not yet |

### `in`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Keeps the union members that declare the property | `x: { a: 1 } \| { b: 2 }; if ("a" in x) x` | `{ a: 1 }`; `{ b: 2 }` in the `else` branch | Partial: the limits above describe the keys and subjects. A member stays in both branches when Biome can't tell whether it has the property: a primitive, a function, a tuple, an array, an intersection, a generic, an instance of a class, because Biome doesn't collect the setters and index signatures of classes, or of a built-in type such as `Date` or `Error`, because Biome's built-in types come from the ES2015 library files, which lack members that newer ones add, such as `at` of arrays and `cause` of errors. So does an object type with a call or construct signature, with a setter, a spread, or a computed key, or with a member whose name is spelled with an escape or as a number other than a decimal integer, such as `"\u0061"` or `1.0`. So `x: { a: 1 } \| Date` gives `{ a: 1 } \| Date`, where TypeScript gives `{ a: 1 }`, and `x: P \| Q`, with `class P { p = 1 }` and `class Q { q = 1 }`, gives `P \| Q` for `"p" in x` and `Q` in the `else` branch, where TypeScript gives `P` and `Q`. TypeScript also narrows with a key spelled otherwise, such as a `const` of a literal type, a template literal, `"\u0061"`, or `0x0`, with a key that starts with `#`, such as `"#a"`, and a declared type with a single object type, which can make the `else` branch impossible. |
| An undeclared property adds a `Record` | `x: object; if ("a" in x) x` | `object & Record<"a", unknown>` | Not yet: Biome keeps the declared type when no member declares the property or has an index signature that accepts it. |
| `#brand in x` | `class C { #p = 1 }`<br>`x: C \| Date; if (#p in x) x` (inside `C`) | `C` | Not yet |
| `in` and `hasOwnProperty` remove a missing optional property | `x: { a?: string }; if ("a" in x) x.a`<br>(with `exactOptionalPropertyTypes`) | `string`; `string \| undefined` without `exactOptionalPropertyTypes` | Not yet |

### Library type predicates

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| `Array.isArray` | `x: string \| string[]; if (Array.isArray(x)) x` | `string[]` | Supported, with the restrictions below the table of type predicates, except for `any`, which the row on `any` describes. Other type-guard functions of the lib, such as `ArrayBuffer.isView`, narrow too. |
| `every` with a type predicate callback | `isString: (v: unknown) => v is string;`<br>`xs: (string \| number)[]; if (xs.every(isString)) xs` | `string[]` | Not yet |
| DOM `matches` | `el: Element; if (el.matches("a")) el` (with the `dom` library) | `HTMLAnchorElement` | Not yet |

### Discriminated unions

The rows use `type Circle = { kind: "c"; r: number }` and
`type Square = { kind: "s"; w: number }`.

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Equality of a discriminant | `s: Circle \| Square; if (s.kind === "c") s` | `Circle` | Partial: `===` and `!==` with a unit value, as the limits above list them, such as a literal, `null`, `undefined`, `-1`, or a `const` of a literal type, and `== null` and `!= undefined`, narrow the object, as they narrow a variable, so `const k = "c"; if (s.kind === k) s` gives `Circle`. Loose `==` and `!=` with a literal, and comparisons with other values, such as an enum member, don't narrow the object yet. |
| Truthiness of a discriminant | `r: { ok: true; v: number } \| { ok: false; e: string };`<br>`if (r.ok) r` | `{ ok: true; v: number }` | Supported |
| Destructured `const` or parameter | `s: { kind: "c"; v: number } \| { kind: "s"; v: string };`<br>`const { kind, v } = s; if (kind === "c") v` | `number` | Partial: a test of an element narrows the other elements of the same pattern, as the limits above describe, including renamed elements, nested and array patterns, parameters with a type annotation or a default, and tests inside an aliased condition. The test of `kind` also narrows `s` itself, as the row on discriminants held in a `const` describes. Loose `==` and `!=` with a string, number, bigint, or boolean literal don't narrow the pattern yet, as for a test of a property. TypeScript also narrows the elements of a pattern under a default of an enclosing pattern, as in `const { a: { kind, v } = d } = w`, where Biome keeps their declared types because those ignore the default. Biome can't infer the types of the elements of a pattern whose initializer ends with `!`, as in `const { kind, v } = s!`, or of a parameter destructured in a function that only a variable's annotation types, as in `const f: (s: S) => void = ({ kind, v }) => {}`, so they don't narrow. |
| Dependent rest parameters | `(...[k, p]: ["a", number] \| ["b", string]) => { if (k === "a") p }` | `number` | Partial: a rest parameter destructured by an array pattern narrows as a destructured parameter does. TypeScript also narrows the parameters of a function whose contextual type has a single rest parameter typed with a union of tuples, as in `const f: (...args: ["a", number] \| ["b", string]) => void = (k, p) => { if (k === "a") p }`, where Biome can't infer the types of `k` and `p` yet. |
| Discriminant held in a `const` | `s: Circle \| Square;`<br>`const k = s.kind; if (k === "c") s`<br>`const { kind } = s; if (kind === "c") s` | `Circle` at both reads | Partial: as in TypeScript, a `const` holds a property when its initializer reads the property, as in `const k = s.kind`, `s?.kind`, `s["kind"]`, or `t[0]`, or when it is an element of a destructuring pattern that the declarator binds directly, as in `const { kind } = s`, `const { kind: k } = s`, `const { "kind": k } = s`, or `const [k] = t`, whose initializer is a variable or a property path, not `s!` or `s as S`. The `const` must have no type annotation and must not be exported, and the element must have no default. Tests of the `const` narrow the object as tests of the property do, also inside an aliased condition, where the object must be a variable that cannot change. Through `s?.kind`, no test of `k` shows that `s` is not nullish, so with `s: Circle \| Square \| undefined`, `k === "c"` gives `Circle \| undefined`, as in TypeScript. TypeScript also accepts `s!.kind`, keys spelled differently from the property, such as `s["\u006bind"]`, `t[0x0]`, `{ "\u006bind": k }`, or ``{ [`kind`]: k }``, and a `const` exported by an `export { ... }` list, which keep the declared type in Biome. A read in a closure created after the test sees it, as the rows on closures describe, and a test inside a closure narrows the object there, as in `const k = s.kind; const g = () => { if (k === "c") s }`. Loose `==` doesn't narrow the object yet, as for a test of the property. The deliberate choices above describe writes to the property and rest elements. |
| Discriminant read through optional chaining | `s: Circle \| Square \| undefined; if (s?.kind === "c") s` | `Circle` | Supported: as in TypeScript, the `null` and `undefined` members stay in the branch where the property can be `undefined`, so `s?.kind !== "c"` keeps `Square \| undefined`. |

### `switch`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Literal cases | `x: "a" \| "b" \| "c"; switch (x) { case "a": x }` | `"a"` | Partial: a case narrows the discriminant as `===` does when it is a unit value, as the limits above list them, such as a literal, `null`, `undefined`, `-1`, `void 0`, a template literal without substitutions, or a `const` of a literal type, local or imported, and the discriminant is a variable or a property path, in parentheses or not. TypeScript also narrows by a case of any other value whose type is a unit type, such as an enum member, and a discriminant such as `x!`, where Biome keeps the declared type. Every case test sees the types from just before the `switch`, as in TypeScript. |
| Grouped cases and fallthrough | `x: "a" \| "b" \| "c"; switch (x) { case "a": case "b": x }` | `"a" \| "b"` | Supported: the statements of a group start where one of its cases matches and where the statements before them fall through. A `break` leaves the `switch`, and a `continue` reaches the loop around it. |
| `default` removes the handled values | `x: "a" \| "b" \| "c";`<br>`switch (x) { case "a": break; case "b": break; default: x }` | `"c"` | Partial: `default` narrows in any position, by every case that Biome narrows by, and a group that contains it also starts where one of its cases matches. A member whose discriminant is a union of case values stays: with `s: { kind: "a" \| "b"; x: 1 } \| { kind: "c"; y: 2 }`, `default` after `case "a": case "b":` keeps both members, where TypeScript keeps `{ kind: "c"; y: 2 }`. As in TypeScript, `default` narrows no receiver of an optional chain, so `switch (o?.k) { case undefined: break; default: o }` keeps `undefined` in `o`. |
| `switch (true)` | `x: string \| number;`<br>`switch (true) { case typeof x === "string": x }` | `string` | Partial: a case narrows as its test does when it is truthy, after every case before its group failed to match, and a group with `default` sees the cases outside the group fail, as in TypeScript. The discriminant must be the literal `true` itself, as in TypeScript, which narrows nothing for `(true)` either. A case that fails to match narrows only when it gives a boolean, as the deliberate choices above describe. |
| `switch` on a discriminant | `s: Circle \| Square; switch (s.kind) { case "c": s }` | `Circle` | Supported: cases on a property, an element access, a tuple index, an optional chain, a `const` that holds a property, and an element of a destructuring pattern narrow the object or the pattern as `===` does, within the limits above. |
| An exhaustive `switch` ends the path that matches no case | `x: "a" \| "b";`<br>`switch (x) { case "a": return 1; case "b": return 2; } x` | `never`; the read is unreachable (TS7027) | Supported: the path that matches no case narrows the discriminant by every case, so the read is Unknown, and an exhaustive `switch` leaves that path out for every other variable, as the limits above describe. `UnreachableCodeClassificationRequest` reports the code after such a statement as unreachable when no other path reaches it. |

### Type predicates

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A local, non-generic type-guard function with a primitive target | `function isString(v: unknown): v is string;`<br>`x: string \| number; if (isString(x)) x` | `string` | Supported, with the restrictions below the table |
| The false branch excludes the target | `x: string \| number; if (!isString(x)) x` | `number` | Supported |
| An object, interface, class, or union target | `function isFoo(v: unknown): v is Foo;`<br>`x: Foo \| string; if (isFoo(x)) x` | `Foo`; `string` in the `else` branch | Partial: both branches narrow by TypeScript's subtype relation, as the limits above describe. Biome keeps the type of `x` where TypeScript narrows a type without an object type by a target with only object types, or gives an intersection, and when two members can't be compared, as for instances of classes, which can declare private members, and the literal types that Biome keeps for properties whose values it can't tell are fresh literals, as `{ a: id(1) }`. The `else` branch also keeps a member that is a subtype of a member of the target and the other way around, as `{ a: number }` with the target `Foo`, and `unknown` with the target `{}`, where TypeScript gives `null \| undefined`. |
| An imported, namespace, or method type-guard function | `import { isString } from "./guards"`; `u.isString(x)` | `string` | Supported, with the restrictions below the table |
| A generic type-guard function | `function isNonNull<T>(v: T \| null): v is T;`<br>`x: string \| null; if (isNonNull(x)) x` | `string` | Not yet |
| An overloaded type-guard function | `function isS(v: string \| number): v is string;`<br>`function isS(v: boolean): boolean;`<br>`x: string \| number; if (isS(x)) x` | `string` | Not yet |
| `this is T` | `class TreeNode { isLeaf(): this is Leaf }`<br>`n: TreeNode; if (n.isLeaf()) n` | `Leaf` | Partial: the object of the method narrows as the argument of a type-guard function does, so `n: Leaf \| Branch` gives `Leaf` and `Branch` in the two branches, and so does a property path, as `o.n` in `if (o.n.isLeaf())`. Biome keeps the type of the example, because it can't compare an instance of `TreeNode` with one of a class derived from it, as the limits above describe, and keeps the declared type where TypeScript gives an intersection, as `this & { tag: 1 }` for `isTagged(): this is { tag: 1 }`. |
| The argument is a property or an optional chain | `o: { v: string \| number }; if (isString(o.v)) o.v`<br>`x: { y: string \| number } \| undefined; if (isString(x?.y)) x` | `string`; `{ y: string \| number }` | Partial: the property narrows, but the receiver of an optional chain does not, so the second example gives `{ y: string \| number } \| undefined`. |
| Inferred type predicates | `const isStr = (v: unknown) => typeof v === "string";`<br>`x: string \| number; if (isStr(x)) x` | `string` | Not yet |
| A variable annotated with a function type that returns a type predicate | `const g: (v: unknown) => v is string = ...;`<br>`if (g(x)) x` | `string` | Supported, with the restrictions below the table. So are a parameter annotated in the same way and a variable that holds another type-guard function, as `const alias = isString`. |

A type-guard function that the module declares with `function` or
`declare function`, or as an arrow or function expression that initializes a
`let` or `const` without a type annotation, narrows as its declaration says:
nothing reassigns it, and it has one signature that is not generic, `async`,
or a generator. The tested parameter is named, is not a rest parameter, and
has no default value. The target is not `any` or `unknown`. The call is not
optional, has no type arguments, passes a variable as the tested argument,
and has no spread argument before or at the tested one.

Any other callee, such as an import, a method, a variable or a parameter
with a type annotation, or a variable that holds a function, narrows through
the signature of its type, as TypeScript's `getEffectsSignature` finds it,
within the limits above on callees: the type, without `null` and `undefined`,
must have a single signature that is not generic. The tested parameter is
named, is not a rest or an optional parameter, and gets an argument before
any spread argument, and the target is not `any` or `unknown`. A callee of
`any`, a union of functions, and an overloaded function narrow nothing. The
call has no type arguments, and an optional call, as `g?.(x)`, narrows only
where it is truthy, as in TypeScript. Neither Biome nor TypeScript checks the
body of a type-guard function.

### Assertions

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| `asserts value` | `function assert(c: unknown): asserts c;`<br>`x: string \| null; assert(x !== null); x` | `string` | Supported, with the restrictions below the table |
| `asserts value is T` with a primitive target | `function assertString(v: unknown): asserts v is string;`<br>`x: string \| number; assertString(x); x` | `string` | Supported |
| `asserts value is T` with an object or union target | `function assertFoo(v: unknown): asserts v is Foo;`<br>`x: Foo \| string; assertFoo(x); x` | `Foo` | Partial: as the true branch of a type-guard function with the same target. |
| `asserts this` and `asserts this is T` | `class R { ready?: string; assertReady(): asserts this is { ready: string } }`<br>`this.assertReady(); this` (inside `R`) | `this & { ready: string }` | Partial: `asserts this is T` narrows the object of the method as the true branch of a `this is T` predicate does, so Biome keeps the declared type where TypeScript gives an intersection, as in the example. |
| A method, imported, or annotated assertion function | `x: string \| null; assert.ok(x); x`<br>`a: (v: unknown) => asserts v; a(x); x` | `string` | Supported, with the restrictions below the table |
| An assertion function declared as a variable without a type annotation is ignored (TS2775) | `const a = (v: unknown): asserts v => {};`<br>`x: string \| null; a(x); x` | `string \| null`, with error TS2775 | Supported |
| An assertion call as an operand of the comma operator | `x: string \| null; (assert(x), 0); x` | `string` | Partial: an operand of the comma operator in an expression statement asserts, as in the example, also for the operands after it. TypeScript also asserts through such an operand anywhere else, as in `return (assert(x), x)`, where it takes the signature from the callee's type even when a name along the callee has no explicit type; Biome keeps the declared type there. |
| `assert(false)` ends the path | `x: string \| null; if (x === null) assert(false); x` | `string`; code after `assert(false)` is unreachable (TS7027) | Supported: an `asserts value` signature never returns when its argument is always false as TypeScript's `isFalseExpression` reads it: `false`, `a && b` with either operand such, or `a \|\| b` with both, in parentheses or not. `assert(0)` and `assert(!true)` don't end the path, as in TypeScript. |

A call asserts only as a statement call, as TypeScript's binder adds calls to
the flow: the expression of an expression statement, or an operand of the
comma operator there, not in parentheses, so `(assert(x));` asserts nothing.
Calls inside other expressions, and calls to a type-guard function whose
result is not tested, do not narrow.

An assertion function that the module declares follows the restrictions of
such a type-guard function, must be declared with `function` or
`declare function`, and its call must not be optional. Any other callee
follows the restrictions of other type-guard functions and TypeScript's rule
for assertions (TS2775): every name along the callee has an explicit type. A
function, a class, a namespace that `declare` declares, a variable, a
parameter, or a property with a type annotation, a method, an import of one
of these, a global, and `this` in a class member that is not `static` have
one. A variable that an arrow function without a type annotation, `new`, or
an object literal initializes, an element of a destructuring pattern, and an
accessor have none, and neither does a callee whose type may be `undefined`,
as `k?.(x)` with `k: ((v: unknown) => asserts v) | undefined`. TypeScript
also ignores an assertion through an element access (TS2776).

### Calls that never return

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A call to a function returning `never` ends the path | `declare function fail(): never;`<br>`x: string \| null; if (x === null) fail(); x` | `string`; code after `fail()` is unreachable (TS7027) | Partial: a statement call, as the assertions above describe it, ends the path when the return type annotation of its signature's declaration is `never` and every name along the callee has an explicit type, as for an assertion: a function, a method, a variable or a parameter annotated with a function type, a method of a type literal, an import of one of these, or a global, so `function fail() { throw 0; }` and `const fail = (): never => {}` end nothing, as in TypeScript. The paths that it ends add nothing where they join the others, and a read that only such paths reach keeps its declared type, as in TypeScript. A generic or overloaded signature ends nothing yet, as the limits above describe. |
| `return` and `throw` end the path | `x: string \| null; if (!x) throw new Error(); x` | `string` | Supported |

### Assignments

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| An assignment narrows to the matching members of the declared type | `let x: string \| number; x = "a"; x` | `string` | Partial: as in TypeScript, a write keeps the members of a declared union that the assigned value may have, keeps the declared type when it isn't a union or when the value is `any`, and gives `never` for a `never` value, which Biome reads as Unknown. A write of a prefix of a property path gives the path its declared type, and the code after `try`, `if`, loops, and labeled blocks joins the writes on each path. The assigned value is typed without narrowing, as the limits above describe, so `x = y` after `if (y)` keeps the declared type of `y`, where TypeScript uses its narrowed type. A call whose return type the limits above don't let a write use gives the declared type, so `x: number \| undefined` keeps `undefined` after `x = k.m()` with `class K { m() { return 1; } }`, where TypeScript gives `number`. `boolean` stays whole, so `let b: boolean \| undefined = true` gives `boolean`, where TypeScript gives `true`. Biome types the value of an assignment expression, as `y` in `x = y = 1`, an empty array literal, a missing element of an array literal, and the element of an array that a destructuring target reads, which may be `undefined`, so that these writes keep the declared type, where TypeScript narrows. A write behind `as`, `satisfies`, or `<T>`, as `(x as T) = v`, gives the declared type, where TypeScript ignores the write. |
| An initializer narrows an annotated union | `const x: string \| number = 1; x` | `number` | Supported: a declarator with a type annotation writes the variables it declares, also through a destructuring pattern, as `const { a }: { a: string \| number } = { a: 1 }`. |
| Compound assignments and `++` | `let x: number \| string = 0; x += 1; x` | `number` | Partial: `+=` and the other compound assignments, `++`, and `--` give the primitive type of the literal types of the type before the write, as TypeScript's `getBaseTypeOfLiteralType` does, so `x: 1 \| 2` is `number` after `x++`. An assignment whose right operand has a shift or arithmetic operator, as `x = x * 2`, reduces the primitive type of the literal types of the declared type. Its right operand is typed without narrowing, so `x = x + 1` after `x: number \| string = 1` gives `number \| string`, where TypeScript gives `number`. |
| Logical assignments | `x: string \| undefined; x ??= "d"; x` | `string` | Partial: the right operand runs only where the target is nullish for `??=`, falsy for `\|\|=`, or truthy for `&&=`, so it sees the target tested there, and a test of a discriminant narrows the object that holds it, as `s` in `s.kind \|\|= f(s)`, as in TypeScript. The target is written there, and the code after the assignment joins that path with the one that skipped the right operand. The right operand is typed without narrowing, as for other assignments. |
| Destructuring assignments | `let x: string \| number; [x] = ["a"]; x` | `string` | Partial: each target of a destructuring assignment, of the head of a `for...in` or `for...of` loop, and each variable of a declarator with a type annotation is assigned the field of the value that destructuring reads, with the default that supplies it when the field is `undefined`, as TypeScript's `getAssignedType` does. Biome types the element of an array as possibly `undefined`, and a missing element of an array literal as unknown, so `[x] = xs` with `xs: string[]`, `[[x]] = [["a"]]`, and `[x = 1] = []` keep the declared type of `x: string \| number`, where TypeScript narrows. A computed key that is not a literal of the kind that a property path accepts gives its target the declared type. The deliberate choices above describe the order in which the pattern and the value run. |
| An unannotated `let` takes the type of its assignments | `let x; x = 1; x` | `number` | Partial: a `let` or a `var` declared as a name without a type annotation, not exported and not `declare`, without an initializer or with `null` or `undefined` as its initializer, starts at `undefined` and takes the type of each value assigned to it, with its literal types widened, as TypeScript's `autoType` does, so `let x = null; if (c) x = "a"; x` is `string \| null`. Paths join with a union, and runtime tests narrow the type that the writes give. Biome follows the writes only in the execution root that declares the variable: TypeScript gives a read in another function, as a closure, an implicit `any` (TS7005), and Biome keeps the declared type there. A variable that nothing writes narrows as any other variable. A read keeps its declared type where a write that reaches it assigns a value that Biome may type otherwise than TypeScript: a call whose return type the limits above don't let a write use, a function, an object with a method, an object type or a type argument with a literal type, as `{ k: id(1) }`, `[]`, `any`, `unknown`, `void`, or a value that reads another such variable, and where the write is a compound assignment, `++`, `--`, or an assignment whose right operand has a shift or arithmetic operator, as `x = x + 1`. It also keeps it where the values assigned are all `null`, or all `undefined`, so `let x; x = null; x` keeps the declared type, where TypeScript gives `null`. The declared type is the type of the initializer and ignores the writes in the variable's own function, so a read that keeps it can be narrower than in TypeScript: `let x = null; x = o.load(); x` is `null`, where TypeScript gives `number` for `const o = { load() { return 1; } }`. |
| `delete` counts as an assignment | `o: { p?: string }; if (o.p) { delete o.p; o.p }` | `undefined` | Supported: `delete o.p` assigns `undefined`. `delete o["p"]` gives `o.p` its declared type, where TypeScript ignores it, as the deliberate choices above describe. |
| A variable or field read before it is assigned keeps its declared type | `let x: string \| null; if (x !== null) x`<br>`class K { x: string \| null; constructor() { if (this.x !== null) this.x; this.x = null; } }` | `string \| null` at both reads (TS2454, TS2565) | Supported: a `let` without an initializer, outside the head of a loop and without `!`, a `var` without `!`, which a read can reach before its declarator, and in a constructor, and in an arrow function that it calls where it is defined, a path on `this` whose field has no initializer and no `!` and is neither `static`, `declare`, nor `accessor`, start with `undefined` added to their declared type, as in TypeScript. In a function nested in the code that declares it, a `let` or a `var` starts that way only when it has no initializer and nothing writes it; TypeScript assumes such a `var` assigned there, so Biome keeps `var w: string \| null; function f() { if (w !== null) w }` at `string \| null`, where TypeScript gives `string`. A closure that continues the flow where it is created starts such a `let` from its type there, without `undefined` when some code assigns it, as the deliberate choices above describe. A read where `undefined` remains, although the declared type lacks it, has the declared type. A test that removes `undefined`, as `typeof x === "string"`, and an assignment narrow as usual. |

### Aliased conditions

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A `const` holding a runtime test, up to five levels deep | `x: string \| number; const isS = typeof x === "string"; if (isS) x` | `string` | Partial: the `const` must have an initializer and no type annotation, must declare a name rather than a destructuring pattern, and must not be exported, and the condition narrows only the variables and the readonly property paths that the limits above allow, as `o.v` in `o: { readonly v: string \| null }; const ok = o.v !== null; if (ok) o.v`. TypeScript also narrows a variable declared by a `let` statement at the top level of a module, a `const` exported only by an `export { ... }` list, which it also accepts as the aliased condition, a property path that it takes as readonly in the cases that the limits above list, and a property that is readonly in one member of a union, as the deliberate choices above describe. A comparison such as `if (isS === true)` does not narrow `x` yet, as the row on equality with the result of a runtime test describes. |
| A `const` holding `&&` or `\|\|` of runtime tests | `a: string \| null; b: string \| null;`<br>`const ok = a !== null && b !== null; if (ok) a` | `string` | Supported |

### Closures

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A `const` read in an arrow or function expression keeps the outer narrowing | `const x: string \| null = get(); if (x) () => x` | `string` | Supported: a read in a function or arrow expression, including an `async` function and a generator, or in a method or accessor of an object literal or of a class expression, continues the flow where the closure is created, as TypeScript's `checkIdentifier` does, through any number of such closures nested in one another. A test inside a closure narrows further from there, and an element of a destructuring pattern continues the narrowing of its pattern. An import, a `var`, a function, and a class never continue, and neither do the narrowing of a property path and a path on `this`, as in `if (o.v) () => o.v`, which keeps `string \| null`, as in TypeScript; the type of `o` itself continues, so `o.v` reads its property on the narrowed object. A closure created in unreachable code, as after `return`, in the branch of `if (false)`, or after a call that never returns, starts from the declared type, as in TypeScript. |
| A parameter or `let` read in a closure created after its last assignment | `function f(x: string \| null) { if (x) [1].forEach(() => x) }` | `string` | Partial: as TypeScript's `isPastLastAssignment` decides, a parameter, a `catch` variable, or a `let` continues when the closure is created after its last assignment in the function that declares it. An assignment counts until the end of the outermost variable or expression statement, `if`, loop, `with`, `switch`, `try`, or class declaration around it, so `if (c) { x = "a"; const h = () => x; }` keeps the declared type, as in TypeScript. An assignment in any other function, such as a closure or a function called where it is defined, means that no closure continues. TypeScript also continues a `let` declared at the top level of a module, where Biome keeps the declared type, because it can't tell a module from a script, whose top-level `let` other scripts can write. |
| A function declaration never sees the outer narrowing | `x: string \| null; if (x) { function g() { x } }` | `string \| null` | Supported: a closure nested in a function declaration continues the flow of the declaration, which starts from the declared type. |
| An immediately invoked function expression continues the outer flow | `x: string \| null; if (x) (() => x)()` | `string` | Partial: a read inside it continues the flow where it is created, as in any closure. A write inside it keeps the variable at its declared type in the whole execution root, where TypeScript narrows the code after the call, as `null` in `if (x) { (() => { x = null; })(); x }`, and a `let` assigned after the call keeps its declared type inside it, where TypeScript narrows it as code that runs in place. |

### Class members

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A method or accessor of a class expression keeps the outer narrowing as an arrow function does, and a static block of any class continues the outer flow | `x: string \| null;`<br>`if (x) { const C = class { m() { x } }; class D { static { x } n() { x } } }` | `string` in `m` and in the static block; `string \| null` in `n`, because a method of a class declaration never sees the outer narrowing | Partial: a method or accessor of a class expression continues the flow where the class is created, as a closure does, and a method of a class declaration doesn't, as in TypeScript. Biome analyzes a static block on its own, so a read of an outer variable there keeps its declared type. |
| The decorators and computed keys of class members | `x: string \| null;`<br>`if (x) { const C = class { @dec(x) m() {} @dec(x) f?: number; @dec(x) g = 1 } }` | `string` in the decorators of `m` and `f`, `string \| null` in the decorator of `g`. In any class, the decorators and computed keys of a field without an initializer narrow, and those of a field with an initializer do not. Those of a method or accessor narrow in a class expression but not in a class declaration. | Not yet: Biome evaluates no class member in the enclosing flow, so these reads keep their declared type. |

### Loops

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| The condition of `while`, `do...while`, and `for` narrows the body and the exit | `x: string \| null; while (x) x` | `string` | Supported |
| A read in a loop joins the entry and the back edge | `x: string \| null; while (flag) { x; if (!x) return; }` | `string \| null` at `x;` | Supported |
| `while (true)` and `for (;;)` exit only through `break` | `x: string \| null; while (true) { if (x) break; } x` | `string` | Supported |
| `for...in` and `for...of` | `xs: (string \| null)[]; for (const v of xs) { if (v !== null) v }` | `string` | Supported: conditions in the body narrow the loop variable, its property paths, and the elements of its destructuring pattern, and the narrowing from before the loop applies in the body and after the loop. As in TypeScript, the code after the loop continues from the start of an iteration and from each `break`, so a test in the body never narrows it: `for (const v of xs) { if (y === null) return; } y` keeps `y` as declared. The expression runs once, before the loop, and a read at the start of the body joins the types from before the loop with those at the end of the body and at each `continue`. `for await` loops narrow too, but Biome doesn't infer the type of the elements of an `AsyncIterable` yet, so the loop variable of `for await (const v of xs)` with `xs: AsyncIterable<T>` is unknown and doesn't narrow. |
| `for (k in o)` removes nullish types from `o` | `o: { a: 1 } \| undefined; for (const k in o) o` | `{ a: 1 }` | Partial: as in TypeScript, the body of a `for...in` loop whose head declares a variable, with `const`, `let`, or `var`, sees the expression as not nullish, and the receivers of its optional chain too, as `o` in `for (const k in o?.v)`. The code after the loop doesn't, and neither does an object whose property is the expression, as `s` in `for (const k in s.kind)`. A loop whose head is an assignment target, as in `for (k in o)`, or a destructuring pattern, which TypeScript rejects (TS2491), narrows nothing. Parentheses around the expression are skipped. TypeScript also narrows `o` through `for (const k in o!)`, where Biome keeps the declared type. |
| Labeled `break` and `continue` | `x: string \| null; out: { if (x === null) break out; x }` | `string` | Supported |

### `try`, `catch`, and `finally`

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| `catch` starts from the state before `try` | `x: string \| null; try { if (x === null) throw 0; x } catch { x }` | `string` in the try block, `string \| null` in the catch clause | Supported: an exception can leave the try block at any point, so a test in the try block never narrows the catch clause. |
| The code after a `try` sees its tests when the catch clause ends the path | `x: string \| null; try { if (x === null) return; f(); } catch { return; } x` | `string` | Supported: the code after the statement continues from the ends of the try block and of the catch clause. A catch clause that completes normally starts from the state before `try`, so `try { if (x === null) return; } catch {} x` gives `string \| null`. |
| `finally` joins every way into it; the code after it only the normal exits | `x: string \| null; try { if (x === null) return; } finally { x } x` | `string \| null` in the finally block, `string` after it | Supported: the finally block sees the ends of the try block and of the catch clause, the exceptions, and each `return` in them. The code after the statement runs only when the try block or the catch clause ended normally, and the tests in the finally block narrow it too: `try { f(); } finally { if (x === null) throw 0; } x` gives `string`. Without a normal end, as in `try { return; } finally {}`, the code after the statement is unreachable. As in TypeScript, `break` and `continue` reach their target without passing through the finally block. |
| `catch` and `finally` see every assignment in `try` | `let x: string \| number = "a"; try { x = 1; f(); } catch { x }` | `string \| number` | Supported: each write in a try block or catch clause reaches the clause or block that handles its exceptions, as TypeScript's `createFlowMutation` does. |
| A `catch` variable is `unknown`, or its annotation | `try { f(); } catch (e) { if (e instanceof Error) e.message }` | `e` is `Error` in `e.message` | Supported: a `catch` variable has its annotation, `unknown` or `any`, or `unknown` without one, and narrows as other variables do. A variable of a destructuring pattern, as `message` in `catch ({ message })`, is Unknown, where TypeScript gives `any` after an error. |

### Optional chaining

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| A truthy `x?.y` removes nullish types from `x` | `x: { y: string } \| undefined; if (x?.y) x` | `{ y: string }` | Supported |
| Equality of `x?.y` with a value | `x: { y: number } \| undefined; if (x?.y === 1) x` | `{ y: number }` | Partial: comparisons with a unit value, as the limits above list them, such as a literal, `null`, `undefined`, `-1`, `void 0`, or a local `const` of a literal type, with `===`, `!==`, `==`, or `!=`, narrow `x` in the branch where `x?.y` cannot be `undefined`. TypeScript also narrows `x` for any other value whose type excludes `undefined`, such as an imported `const` or a variable of type `number`. |
| A chain that starts at a parenthesized optional chain | `x: { z?: { w: string } } \| undefined;`<br>`if ((x?.z)?.w) x` | `{ z?: { w: string } }` | Not yet: Biome narrows neither `x` nor its paths. In a comparison, such as `(x?.z)?.w === "a"`, TypeScript narrows `x.z` but not `x`, so a read of `x?.z` adds `undefined` back, which Biome cannot express yet. |
| The receiver is not nullish inside the chain | `x: { f(v: unknown): void } \| undefined; x?.f(x)` | `{ f(v: unknown): void }` at the argument | Partial: the key and the arguments after each `?.`, and the rest of the chain after them, as in `x?.[x.k]` or `x?.a.f(x)`, see the object of the `?.` as not nullish. As in TypeScript, the object is tested as not nullish rather than as truthy, so `n: 0 \| 1 \| undefined` stays `0 \| 1` in `n?.toFixed(n)`, and the code after the chain keeps the types from before it. When a key or an argument writes a variable, as in `o?.f(x = 1)`, the code after the chain joins the flow where each `?.` gives `undefined` with the end of the chain. Parentheses and `!` end a chain: TypeScript also narrows `x` in `x?.a!.b(x)`, and both `o?.v` and `o` in `(o?.v)?.w(o?.v, o)`, where Biome narrows only `o?.v`. |
| `instanceof`, `switch`, or a type-guard function applied to `x?.y` | `x: { y: unknown } \| undefined; if (x?.y instanceof Date) x` | `{ y: unknown }` | Partial: `instanceof` removes `null` and `undefined` from the receivers in the true branch, and a group of cases of `switch (x?.y)` or `switch (typeof x?.y)` does where every case shows that `x?.y` is not `undefined`, as in TypeScript. Type-guard functions don't narrow the receivers yet. |

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
| `this.x`, including assignments in a constructor | `class K { x: string \| null = null; m() { if (this.x) this.x } }` | `string` | Partial: paths on `this`, including private names such as `this.#x`, narrow in methods and accessors, in constructors, in object literal methods, and in arrow functions inside them or inside a field initializer, within the limits above, and an assignment such as `this.x = "a"` narrows them. They do not narrow yet in a `static` method or static block, where Biome types `this` as an instance instead of the class, in a member with a `this` parameter, or in a member of an object literal or class without a name that Biome can find, such as an object literal passed to a call. |
| Element access with a literal key | `o: { a: string \| null }; if (o["a"]) o["a"]` | `string` | Partial: `o["a"]` reads the same property as `o.a`, and `t[0]` the same as `t["0"]`, but only a string literal without escapes or a decimal integer from 0 to 2^53 - 1 without leading zeros or separators is a key. TypeScript also matches other spellings of the same key, so `if (t[0x0]) t[0]` and ``if (o[`a`]) o.a`` narrow there but not in Biome. |
| Element access with a `const` key | `o: { a: string \| number }; const k = "a";`<br>`if (typeof o[k] === "string") o[k]` | `string` | Not yet |

### Variable kinds

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| Imported bindings and `var` | `var x: string \| null = get(); if (x) x`<br>`import { y } from "./a"; if (y) y` | `string` | Partial: an import narrows with the declared type of what it imports, in its module and in the functions that read it, and so do the paths on a namespace import, as `ns.x`. A `var` narrows as a `let` does, and starts with `undefined` added to its declared type, as the row on reads before an assignment describes: each declarator with an initializer and each iteration of a `for...in` or `for...of` loop that declares it writes it. A `var` whose name another declaration of its scope declares keeps its declared type, as the limits above describe, so `var x: string \| number = 1; var x: string \| number = "a"; x` gives `string \| number`, where TypeScript gives `string`. |
| A variable that the code around a function declares narrows by the function's own tests | `const config: { v: string \| null } = get();`<br>`function f() { if (config.v) config.v }` | `string` | Supported: a function, method, or arrow function narrows a variable, a parameter, or an import of the code around it, and its paths, by its own tests and writes, starting from the declared type, as TypeScript does. Writes elsewhere don't change it there, as the deliberate choices above describe. A test outside a function declaration never reaches a read inside it, while a closure continues the flow where it is created, as the rows on closures describe. |

### Evolving arrays

| Behaviour | Example | TypeScript | Status |
| --- | --- | --- | --- |
| An unannotated `let x = []` takes the types of `push`, `unshift`, and index assignments | `let x = []; x.push(1); x` | `number[]` | Partial: a `let`, a `const`, or a `var` declared as a name without a type annotation, not exported and not `declare`, and initialized with `[]` without parentheses, starts with the element type `never`, as TypeScript's `autoArrayType` does. Each argument of a call of `push` or `unshift` on the variable, also written `x?.push(v)`, `(x).push(v)`, or `x!.push(v)`, and the value of an assignment with `=` to an element with a number literal key, as `x[0] = v`, add their types, with literal types widened, to the element type. A string key adds nothing, assigning `[]` starts again at `never`, and assigning another array gives its element type. Paths join the element types, and a read gives an array of the element type, which runtime tests don't narrow. A read keeps its declared type, `unknown[]`, where nothing was added yet, where TypeScript gives `any[]` (TS7005), in another function, as a closure, and where a write that reaches it adds a value that Biome may type otherwise than TypeScript, as the row on unannotated `let` declarations lists, a spread argument, a value through a key that is not a number literal, as `x[i] = v` with `i: number`, or only `null` or only `undefined` values, where TypeScript gives `number[]`, `null[]`, or `undefined[]`. A function or arrow expression called where it is defined that adds to the array makes every read of the array in the execution root keep its declared type, while one that runs only when called, as a callback, adds nothing, as the deliberate choices above describe. |

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
| Destructuring defaults, computed keys, and parameter defaults see the narrowing before them | `x: string \| null; if (x) { const { a = x } = o }`<br>`function f(x: string \| null, y = x && x.length) {}` | `string` at `a = x` and in `x.length` | Supported: expressions run in the order JavaScript runs them, except that a declaration runs its pattern before its initializer and a destructuring assignment its pattern before its value, as TypeScript's binder does and the deliberate choices above describe. A function runs its parameters, in order, before its body. A default of a declaration or a parameter runs only when the value is `undefined`, so a test inside it narrows only the rest of the default, as in TypeScript: `const { a = x && f(x) } = o; x` keeps `x` as declared after the declaration. |
| Paths join with a union | `x: string \| number \| null;`<br>`if (typeof x === "string") {} else if (x === null) { return; } x` | `string \| number` | Supported |
| A namespace narrows its own variables but never sees the narrowing around it | `o: string \| null; if (o === null) throw 0;`<br>`namespace N { const l: string \| null = get(); if (l) l; o }` | `string` at `l`; `string \| null` at `o` | Supported |
| Reads in unreachable code | `x: string \| null; if (x === null) { return; x }` | `string \| null`; the read is unreachable (TS7027) | Supported |
| A branch narrowed to an impossible type | `x: string; if (typeof x === "number") x` | `never` | Partial: the read has type Unknown, as the correctness contract requires. As in TypeScript, such a branch is not unreachable code, so `UnreachableCodeClassificationRequest` reports it as reachable. |
