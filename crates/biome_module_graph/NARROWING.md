# Narrowing reference

Biome's type inference narrows: it gives a read of a value a more specific
type than its declared type, from the conditions and statements on the paths
that reach the read. After `if (x === null) return;`, a variable `x` declared
as `string | null` reads as `string`. Lint rules see the narrowed type at
each read.

This file describes what narrows, the contract every result keeps, where
Biome differs from TypeScript on purpose, and what Biome doesn't narrow. The
reference is TypeScript 7.0.2 run with `--strict`. A pull request that
changes narrowing updates this file in the same pull request.

How narrowing is built and how to extend it is described in the flow section
of [`CONTRIBUTING.md`](./CONTRIBUTING.md#flow-sensitive-expression-reads).
The remaining work is tracked in
[#13147](https://github.com/biomejs/biome/issues/13147).

Examples write a parameter or a declared variable as `x: T` and leave out the
function around it. "Wider" means that a result has more values than
TypeScript's, and "narrower" that it has fewer.

## Terms

- **Declared type**: the type of a variable before narrowing, from its
  annotation or from inference that ignores flow. Narrowing never changes
  it. The declared type of a property path is the property's type on the
  narrowed type of its object at the read.
- **Read**: one place where the program uses the value of a variable or of a
  property path.
- **Execution root**: a body that Biome analyzes on its own: a module, a
  namespace, a function, a method, an accessor, or a constructor.
- **Inline code**: a function or arrow expression called where it is
  defined, and a class static block. Inline code runs in the flow of the
  execution root around it.
- **Closure**: a function or arrow expression, or a method or accessor of an
  object literal or of a class expression. A read in a closure continues the
  narrowing where the closure is created.
- **Condition**: an expression whose result decides which branch runs, such
  as the test of an `if` or an operand of `&&`. A **runtime test** is one
  check that a condition performs on one value, such as
  `typeof x === "string"`.
- **Subject**: what a runtime test checks: a variable or `this`, optionally
  followed by a path of property names, such as `opts.timeout` or `pair[0]`.
- **Unit value**: a value whose type has a single value, such as `"a"`, `1`,
  `null`, or `undefined`.
- **Discriminant**: a property whose literal types tell the members of a
  union apart, such as `kind` in
  `{ kind: "c"; r: number } | { kind: "s"; w: number }`.
- **Aliased condition**: a `const` whose initializer is a condition. Testing
  the `const` narrows the condition's subjects, as `ok` does in
  `const ok = x !== null; if (ok) x`.
- **Type predicate**: a return annotation such as `value is T`,
  `asserts value is T`, or `asserts value`. A **type-guard function** returns
  a type predicate. An **assertion function** is a type-guard function whose
  type predicate starts with `asserts`.
- **Statement call**: a call that is the expression of an expression
  statement, or an operand of the comma operator there, without parentheses
  around it. Only statement calls assert or end a path.

## What narrows

Biome narrows as TypeScript does wherever this file doesn't say otherwise.

### Values

A read narrows when it reads one of these values:

- A parameter, a `catch` variable, an import, or a variable declared with
  `let`, `const`, `var`, or `function`. An import narrows with the declared
  type of what it imports, and so do the paths on a namespace import, such
  as `ns.x`.
- A property path that starts at such a variable or at `this`: named
  properties and private names, written with `.` or `?.`, as in `o.v`,
  `a.b.c`, or `this.#v`, and element accesses with a literal key, as in
  `o["v"]` or `t[0]`. `o["v"]`, `o?.v`, and `(o).v` are the same path as
  `o.v`, and `t[0]` is the same path as `t["0"]`.
- `this` itself, where its paths narrow, as by a `this is T` predicate.
- An element of a destructuring pattern. With
  `s: { kind: "c"; v: number } | { kind: "s"; v: string }` and
  `const { kind, v } = s`, `if (kind === "c") v` reads `v` as `number`.
- The object that holds a discriminant. `s.kind === "c"` narrows `s`, and so
  does a test of `k` in `const k = s.kind` or `const { kind: k } = s`.

Each execution root narrows by its own tests and writes. A variable of the
code around a function, such as a module variable or an import read in a
function, starts there from its declared type, whatever the code around the
function tested. A closure continues the flow where it is created instead,
and inline code runs in the flow around it.

### Runtime tests

Conditions narrow the branches they decide: `if`, `while`, `do...while`, and
`for` statements, conditional expressions, the operands of `&&`, `||`, and
`??`, the cases of `switch`, and the right operand of a logical assignment.
Biome decomposes a condition through `!`, `&&`, `||`, and parentheses into
these runtime tests:

- **Truthiness.** The truthy branch removes `null`, `undefined`, and falsy
  literals. The falsy branch keeps broad `string`, `number`, and `bigint`,
  and `boolean` splits into `true` and `false`.
- **Equality.** `===`, `!==`, `==`, `!=`, and `case` with a unit value: a
  literal, `null`, `undefined`, `void 0`, `-1`, a template literal without
  substitutions, a `const` that holds one of these, or a member of an enum.
  `== null` and `!= undefined` match both nullish values, and a broad
  primitive becomes the literal, as `x: string` does in `x === "a"`. A
  comparison also narrows by the declared type of a variable with a type
  annotation on the other side, and a comparison of a condition with `true`
  or `false` narrows as the condition does.
- **`typeof`**, with the operands in either order, and `switch (typeof x)`.
- **`instanceof`**, and `x.constructor === C` with `===` or `==` in either
  order.
- **`in`**, with a literal key, as `"a" in x` or `0 in x`, and with a private
  name, as `#p in x`, which narrows `x` as `x instanceof C` does for the
  innermost class `C` around the test that declares `#p`.
- **Calls of type-guard functions**: local, imported, and namespace
  functions, methods, variables and parameters annotated with a function
  type, variables that hold another type-guard function, generic and
  overloaded functions, `this is T` methods, functions whose type predicate
  Biome infers from their body, and type predicates of the standard library,
  such as `Array.isArray`, `ArrayBuffer.isView`, and `every`.
- **Aliased conditions**, inlined into one another at most five levels deep,
  as in TypeScript.

### Statements and calls

- `return`, `throw`, `break`, and `continue`, also with labels, end a path.
  Paths join with a union where they meet.
- A loop narrows its body and its exit by its condition, and a read in a loop
  joins the types on entry with those at the end of each iteration and at
  each `continue`. `while (true)` and `for (;;)` exit only through `break`.
  `for...in`, `for...of`, and `for await` narrow their loop variable, its
  paths, and the elements of its destructuring pattern, and the code after
  such a loop continues from the start of an iteration and from each
  `break`. The body of a `for...in` loop whose head declares a variable sees
  the expression as not nullish, as `o` in `for (const k in o)`.
- A `switch` case narrows the discriminant, and a group of cases starts
  where one of its cases matches and where the statements before it fall
  through. A `break` leaves the `switch`, and a `continue` reaches the loop
  around it. `default` narrows by the cases that didn't match, in any
  position. `switch (true)` narrows each case as its test, and the cases
  before it as failed tests. An exhaustive `switch` without `default` ends
  the path that matches no case.
- In `try` statements, the catch clause starts from the state before `try`
  joined with every write in the try block, because an exception can leave
  the block at any point. The finally block joins every way into it. The
  code after the statement continues only from the normal ends of the try
  block and the catch clause, narrowed further by the tests in the finally
  block. `break` and `continue` reach their target without passing through
  the finally block.
- A write narrows the variable or the path it writes to the members of its
  declared union that the assigned value may have. Assignments of every
  kind, `++`, `--`, destructuring targets, the targets of `for...in` and
  `for...of`, `delete`, and a declarator with a type annotation count as
  writes, as in `const x: string | number = 1`, also through a destructuring
  pattern, as in `const { a }: { a: string | number } = { a: 1 }`. Each
  target of a destructuring write is assigned the field of the value that
  destructuring reads, or the default that supplies it when the field is
  `undefined`.
- A `let` or `var` without a type annotation whose initializer is missing,
  `null`, or `undefined` takes the type of each value assigned to it, so
  `let x; x = 1; x` reads `number`. A variable without a type annotation
  initialized with `[]` takes the types of the values added with `push`,
  `unshift`, and index assignments, so `const a = []; a.push(1); a` reads
  `number[]`.
- A statement call of an assertion function narrows the code after it, and a
  statement call of a function that returns `never` ends the path.
  `assert(false)` ends the path too, through an `asserts value` signature.
- The literal `true` or `false` itself, without parentheses, as the
  condition of an `if`, `while`, `do...while`, or `for` statement, of a
  conditional expression, or as an operand of `&&` or `||`, makes the branch
  it never takes unreachable, as in TypeScript. So `if (c && false)` never
  runs its body, and `while (c || true)` exits only through `break`. `!false`,
  `(false)`, `0`, a `const` that holds `false`, and the operands of `??` are
  ordinary conditions, so `while (1)`, `while ((true))`, and
  `do { } while (1)` can exit through their test.
- A read in unreachable code keeps its declared type, and an unreachable
  path adds nothing where it joins the others. The paths that a call that
  never returns ends are unreachable in this sense.
- A closure continues the narrowing where it is created, through any number
  of closures nested in one another, including `async` functions and
  generators. A test inside a closure narrows further from there, and an
  element of a destructuring pattern continues the narrowing of its
  pattern. Inline code runs where it is called: its writes narrow the code
  after it, and a `throw` inside it ends the path.
- In an optional chain, a test that shows that `x?.y` isn't `undefined`, as
  a truthy `x?.y`, `x?.y === 1`, or `typeof x?.y === "number"`, removes
  `null` and `undefined` from `x`, and the keys and arguments after a `?.`
  see its object as not nullish. The right operand of `??` sees the left
  operand as nullish.
- Destructuring defaults, computed keys, and parameter defaults see the
  narrowing before them. A function runs its parameters, in order, before
  its body. A default of a declaration or a parameter runs only when the
  value is `undefined`, so a test inside it narrows only the rest of the
  default: `const { a = x && f(x) } = o; x` keeps `x` as declared after the
  declaration.

### Results that follow TypeScript

These results match TypeScript's and can surprise a reader:

- `unknown` narrows as `{} | null | undefined`: a truthy `unknown` reads
  `{}`, `x != null` gives `{}`, `x !== null` gives `{} | undefined`, and a
  result that keeps all three is `unknown` again. A strict comparison with a
  value whose declared type is a single primitive or literal type gives that
  type, and one with an object type gives `object`. `unknown` stays for a
  union, for loose equality, and in the `else` branch. `typeof` gives
  `object | null` for `"object"`, `Function` for `"function"`, and the
  primitive for a primitive kind, whose `else` branch keeps `unknown`.
  `typeof x !== "object"` gives `{} | undefined`.
- `typeof` on `{}` gives the primitive. With `"object"`, it gives `object`
  only for a type literal or an object expression without members, also
  through an alias, and keeps an empty interface or class.
- `typeof` with a primitive kind replaces an object type that every value of
  the kind has with the primitive, so `x: number | { length: number }` gives
  `string` for `"string"`. `"undefined"` removes every object type. With
  `"function"`, an object type that no function can have is removed, one that
  every function has becomes `Function`, and one with a call or construct
  signature stays.
- A type parameter narrows to an intersection. Truthiness and `!= null` give
  `T & {}`, which is `NonNullable<T>`, and keep `T` when its constraint
  excludes `null` and `undefined`, as `T extends {}`. `!== null` gives
  `T & ({} | undefined)`, and `!== undefined` gives `T & ({} | null)`, or
  `T & {}` when another member of the union is the other nullish value.
  `typeof` gives `T & string`, `T & undefined`, or `T & Function`,
  `"object"` gives `(T & object) | (T & null)`, and the `else` branch of
  `"undefined"` gives `T & ({} | null)`. `T` stays when its constraint has
  only values of the kind. An intersection that narrowing gives narrows again
  by each operand, so `typeof x === "string"` after `if (x)` gives
  `T & string`, while a comparison keeps it whole or removes it. `T & boolean`
  stands for `(T & true) | (T & false)`. A property of `T & {}` is read on the
  constraint of `T`.
- `instanceof` makes `unknown` and `any` the instance type. The instance type
  of a generic class has an `any` type argument for each type parameter, as
  in `Map<any, any>`, where it replaces `unknown` or `any` or where no member
  remains. A member that is derived from the class keeps its own type
  arguments. The `else` branch keeps `unknown` and `any`.
- `in` gives the intersection with `Record<"a", unknown>` when no member can
  have the property. Biome writes it `{ a: unknown }` and merges it into the
  object types of the members, as `{ b: 1; a: unknown }` for `x: { b: 1 }`.
  The members of `Object`, such as `toString`, count as declared.
- A type-guard function, `instanceof`, and `#p in x` give an intersection
  with the target where no member relates to it, as `Bar & Foo` for `x: Bar`
  with `v is Foo`, or `T & Foo` for a type parameter without a constraint or
  whose constraint relates to the target. The `else` branch keeps the type
  parameter.
- `any` narrows only through `typeof` with a primitive kind or
  `"undefined"`, `instanceof`, and type-guard and assertion functions,
  including those of the standard library, and only in the true branch.
  Every `else` branch keeps `any`, and so do truthiness, equality, `in`,
  discriminants, and `instanceof` with `Object` or `Function`. Once a test
  narrows `any`, later tests narrow the result, so `x === "a"` after
  `typeof x === "string"` gives `"a"`. A variable whose type is `any` through
  an alias, an import, or inference, as `const data = JSON.parse(s)`, narrows
  as one annotated `any`.
- A discriminant read through an optional chain keeps the `null` and
  `undefined` members in the branch where the property can be `undefined`,
  so `s?.kind !== "c"` keeps `Square | undefined`.

## Correctness contract

Every narrowing result keeps these rules:

1. At any read, Biome's narrowed type is never narrower than TypeScript's
   narrowed type at the same read, with TypeScript run over the files that
   Biome indexes.
2. When inference cannot decide, the read keeps its declared type. When a
   work limit or a query cycle stops the computation, the read has type
   Unknown instead.
3. A read in a branch proven impossible has type Unknown, not `never`. A
   rule that needs to know whether a point is unreachable asks
   `UnreachableCodeClassificationRequest`, never the read's type. A branch
   narrowed to an impossible type, as `typeof x === "number"` for
   `x: string`, is not unreachable code, as in TypeScript.

A type wider than TypeScript's can make a rule report a union member that
TypeScript has removed. A narrower type makes rules report code that
TypeScript accepts, or suggest deleting a check that the program needs.

A few limits below describe reads that are narrower than TypeScript's, most
of them in code that TypeScript types as impossible or finds unreachable.
Each of those limits says so.

Biome doesn't read `tsconfig.json`, so it can't know which files a
project's TypeScript program contains, and rule 1 compares with TypeScript
run over the files that Biome indexes instead. The augmentations in those
files add members and bases to the types that tests compare. For a
particular `tsconfig.json`, a read can therefore be narrower than
TypeScript's when
[the program leaves out a file with an augmentation](#an-augmentation-in-a-file-that-the-program-leaves-out-still-applies),
and a call can select another overload when
[the program lists two augmenting files in another order than their paths](#overloads-that-augmentations-add-are-tried-in-the-order-of-module-paths).
A read can also be narrower than TypeScript's over the files that Biome
indexes when
[a TypeScript file without imports or exports declares global interfaces](#top-level-interfaces-become-globals-only-in-declaration-files).
These are deliberate choices.

## Deliberate choices

Some of TypeScript's narrowing is unsound: the narrowed type leaves out
values that the read can have at runtime. Biome copies such narrowing only
when the sound result loses narrowing that idiomatic code relies on and the
unsound case needs unusual code. When a sound result keeps TypeScript's
precision, Biome uses it instead, and differs from TypeScript.

Each choice below is a difference from TypeScript, an unsound result that
Biome copies, or a result that depends on the files that Biome indexes, with
the reason for it. Matching TypeScript, or making a copied result sound,
changes the trade-off that the reason describes.

### Differences from TypeScript

#### `??` as a condition narrows neither operand

```ts
x: string | null | undefined; y: boolean;
if (x ?? y) {} else { x }
// TypeScript: null | undefined. Biome: string | null | undefined.
```

`x` is `""` in the `else` branch when `x` is `""` and `y` is false, because
`""` isn't nullish. The pattern is rare, so the sound result loses little.

#### The falsy part of a broad `number` in `&&` is `number`

```ts
n: number; s: string;
n && s
// TypeScript: string | 0. Biome: number | string.
```

`-0` and `NaN` are falsy numbers too, so `0` is wrong at runtime. A `string`
or `bigint` left operand still contributes `""` or `0n`, its only falsy
value.

#### Falsy branches keep object types that a primitive can have

```ts
interface Len { length: number }
x: Len | undefined; if (!x) x
// TypeScript: undefined. Biome: Len | undefined.
y: string | Len | null; if (typeof y !== "object") y
// TypeScript: string. Biome: string | Len.
```

Every string has `length`, so a string can have type `Len`, and `x` can be
`""` in the falsy branch. A falsy branch removes `object`, functions,
tuples, object literals, instances of built-in classes, and every object
type that no falsy string, number, boolean, or bigint can have, such as
`interface Foo { a: number }`. `typeof y !== "object"` removes the object
types that no primitive and no function can have. The left operand of `&&`
keeps such types in the value of the expression in the same way. TypeScript
removes every object type there except `{}`, even empty interfaces and
classes, which every value except `null` and `undefined` has. Object types
with a member that no primitive has are far more common, so the sound result
loses little.

The positive branch of `typeof y === "object"` also keeps object types with
a call signature, such as `interface Callable { (): void }`, and the negative
branch of `typeof y === "function"` keeps object types with a call or
construct signature. TypeScript removes both.

#### A write through a key that isn't a literal writes every property

```ts
o: { v: string | null; w: string | null }; k: "v" | "w";
if (o.v) { o[k] = null; o.v }
// TypeScript: string. Biome: string | null.
```

A write through a key that isn't a string or integer literal, such as `k`,
`-0`, or a `const` with a literal type, gives every property path of the
object its declared type. A literal key writes only the property it names,
so `o["w"] = null` keeps the narrowing of `o.v`. `delete o["v"]` also gives
`o.v` its declared type. TypeScript ignores the write unless the key is a
literal or a `const` with a literal type, although `k` can be `"v"` here, and
it ignores `delete` through an element access. Such writes are rare next to
tests of a property.

#### A private name and a string key with the same text are different paths

```ts
class K {
  #x: string | null = null;
  "#x": string | null = null;
  m() { if (this.#x) this["#x"] }
}
// TypeScript: string. Biome: string | null.
```

A private name isn't a property, so `this["#x"]` reads the property named
`"#x"`, another value. TypeScript treats both as one reference. Keeping them
apart loses nothing in code without a property whose name starts with `#`.

#### A `const` that holds a property stops narrowing the object after a write

```ts
type Circle = { kind: "c"; r: number };
type Square = { kind: "s"; w: number };
s: Circle | Square;
const k = s.kind; s.kind = "s"; if (k === "c") s
// TypeScript: Circle. Biome: Circle | Square.
```

`k` holds the value of `s.kind` from before the write, so `s` matches
neither member at the read. A test of a `const` that holds a property, and an
aliased condition that tests a property, as
`const ok = s.kind === "c"`, narrow the object only when the execution root
never writes the property or the object's variable, including after the
test, as `u = t` does in `let u = s; const j = u.kind; u = t; if (j === "c") u`.
A test of the property itself, as in `if (s.kind === "c")`, still narrows
`s` after the write. Calls and callbacks that write the property don't count.
TypeScript checks a write to a discriminant against the narrowed object, so
code rarely writes a discriminant in a function that also tests it.

#### Aliased conditions narrow a property only when every member declares it readonly

```ts
o: { readonly v: string | null } | { v: string | null; w: 1 };
const ok = o.v !== null; clear(o); if (ok) o.v
// TypeScript: string. Biome: string | null.
```

TypeScript takes a property of a union as readonly when one member declares
it readonly. A function that receives `o`, such as
`function clear(x: typeof o) { if ("w" in x) x.v = null; }`, can narrow it to
the other member and assign the property, so `o.v` is `null` at the read.
Biome requires, at every step of the path, that every member of the declared
type of the object other than `null` and `undefined` declares the property
readonly. Unions that declare a property readonly in some members only are
rare.

#### A rest element in a destructuring `const` holds no property

```ts
r: { ok: true; v: number } | { ok: false; e: string };
const { ...ok } = r; if (ok) r
// TypeScript: { ok: true; v: number }. Biome: the declared type of r.
```

TypeScript takes `ok` as the property `r.ok`. `ok` holds a new object with
the other properties of `r`, which is always truthy.

#### A failed case of `switch (true)` narrows only when it gives a boolean

```ts
x: { a: 1 } | null;
switch (true) { case x: break; default: x }
// TypeScript: null. Biome: { a: 1 } | null.
```

A case matches only `true`, so `default` also runs for an object, which is
truthy but not `true`. TypeScript takes every case before `default` as falsy.
Biome narrows by a case that failed to match only when the case is `true`
whenever it is truthy: a comparison, `instanceof`, `in`, `!`, a call,
`a && b` with such a `b`, or `a || b` with both such. A case narrows where
it matches as its test does when truthy, as in TypeScript. Idiomatic
`switch (true)` code tests comparisons and type-guard functions, which give
booleans.

#### Loose `==` with a literal keeps members of other kinds

```ts
x: string | number; if (x == 1) x
// TypeScript: 1. Biome: string | 1.
y: "a" | 1 | true; if (y == "a") y
// "a" in both: a string without digits never loosely equals 1 or true.
```

`"1" == 1` holds, and an object such as `["a"]` loosely equals `"a"`. The
true branch narrows the members of the literal's kind as `===` does and
removes `null` and `undefined`, but keeps primitive types and object types
of other kinds. It removes a literal of another kind only when `==` is
certainly false for it: a boolean with a number, a bigint with a number, and
a string without any digit with a number, a boolean, or a bigint, unless the
string is empty, white space, or `Infinity` with an optional sign. A string
without digits converts to `NaN`, which equals nothing. The false branch
removes only the members strictly equal to the literal, as in TypeScript. A
loose comparison by a declared type keeps members of other kinds in the same
way. `== null`, the idiomatic use of loose equality, narrows as in
TypeScript.

#### `cond === false` narrows the other branch only when `cond` gives a boolean

```ts
x: { a: 1 } | null;
const ok = x && x.a === 1;
if (ok === false) {} else x
// TypeScript: { a: 1 }. Biome: { a: 1 } | null.
```

`x && x.a === 1` gives `null` when `x` is `null`, and `null` isn't `false`, so
the `else` branch runs with `x` set to `null`. Where a condition isn't
`false`, it is truthy only when every falsy value it gives is `false`: a
comparison, `instanceof`, `in`, `!`, a call, `a && b` with both such, or
`a || b` with such a `b`. Where it isn't `true`, or loosely equals `false`,
it is falsy only when every truthy value it gives is `true`, as for a case of
`switch (true)`. Where it is `true` or `false`, it narrows as in TypeScript,
and so does an aliased condition compared with a boolean.

#### A write in inline code reaches a `catch` clause around the call

```ts
x: string | null;
if (x) { try { (() => { x = null; throw 0; })(); } catch { x } }
// TypeScript: string. Biome: string | null.
```

The function writes `x` before it throws. The catch clause starts from every
write that can run before an exception in its try block, including the
writes in inline code there. TypeScript starts it only from the writes in the
try block's own code. The pattern is rare.

#### Writes in `async` and generator functions called in place, and through `super`, count

```ts
x: string | null;
if (x) { (async () => { x = null; })(); x }
// TypeScript: string. Biome: string | null.
```

An `async` function called where it is defined runs until its first `await`
before the code after the call, and a generator runs its body when its
iterator is consumed, which can happen later in the function. Such a write
gives the variable or path its declared type in the whole execution root.
`super.x = null` sets `x` on `this`, or calls a setter of the parent class
with `this` as its receiver, so Biome counts it as a write of `this.x` and
gives `this.x` its declared type there. TypeScript ignores both.

### TypeScript's unsound results that Biome copies

#### `eval` and the `arguments` object don't stop narrowing

```ts
x: string | null;
if (x !== null) { eval("x = null"); x }        // string
if (x !== null) { arguments[0] = null; x }     // string
```

A direct `eval` can write any variable it can see, including a type-guard
function. In a function that isn't in strict mode and has no default, rest,
or destructured parameter, `arguments[0] = null` writes the first parameter.
Stopping narrowing would lose it in every function that reads `arguments`,
even only `arguments.length`, or contains a direct `eval`, while code that
writes a narrowed variable this way is rare.

#### Calls, callbacks, and writes through other variables keep narrowing

```ts
o: { v: string | null }; reset: () => void; xs: (string | null)[];
if (o.v) { reset(); o.v }                                // string
if (xs[0]) { xs.shift(); xs[0] }                         // string
if (o.v) { const a = o; a.v = null; o.v }                // string
if (o.v) { [1].forEach(() => { o.v = null; }); o.v }     // string
```

`reset` or the callback can set `o.v` to `null`, `a.v = null` sets `o.v`
through another variable, and `xs.shift()` moves the next element into
`xs[0]`. The same holds for a variable written in a callback, for `this.x`
after `this.reset()` when `reset` sets `this.x = null`, and for an import or
a module variable that a called function can set: an import is live, and the
code around a function can write its variables between two calls. Stopping
narrowing at every call would make it useless, because most tests are
followed by a call, and rules would report code that TypeScript accepts.

A write in the execution root's own flow, including inline code, narrows at
the write. A variable or an array whose type evolves with its writes takes
the types of the writes in the root's own flow in the same way, so
`let n; n = 1; [1].forEach(() => { n = "s"; }); n` reads `number` and
`const items = []; items.push(1); [1].forEach(() => items.push("s")); items`
reads `number[]`, as in TypeScript, although the callbacks run before the
reads.

#### A getter narrows like a field

```ts
class G { get v(): string | null }
g: G; if (g.v) g.v      // string
```

A getter can return a different value each time it runs. Treating getters as
fields keeps narrowing for classes that expose their state through getters,
and getters that change between two reads close together are rare.

#### `in` treats object types as closed

```ts
function f(x: { a: 1 } | { b: 2 }) { if ("a" in x) x.a }   // 1
const v = { a: "s", b: 2 as const }; f(v);
```

`{ b: 2 }` also describes objects with other properties, so `f(v)` passes an
object whose `a` is `"s"`. Keeping `{ b: 2 }` in the true branch, as
`{ b: 2 } & Record<"a", unknown>`, would make `in` useless for telling the
members of a union apart, while code rarely passes an object whose type
leaves out a property that the code tests with `in`.

#### `instanceof` removes members that aren't derived from the class

```ts
class A { a = 1 }; class B extends A { b = 2 }
x: B | { c: number }; if (x instanceof A) x      // B
const v: { c: number } = Object.assign(new A(), { c: 1 });
```

`{ c: number }` also describes an instance of `A` with a property `c`, such
as `v`. The true branch keeps the members derived from the class and removes
object type literals, functions, and other types that aren't. Keeping them
would make `instanceof` useless for telling the members of a union apart.

#### The false branch of `instanceof` removes the class and its derived types

```ts
class A { a = 1 }
x: A | string; if (!(x instanceof A)) x      // string
const v: A = { a: 1 };
```

TypeScript accepts a plain object for a class whose members are all public,
so `x` can be such an object, not an instance of `A`. The sound result would
remove nothing in the `else` branch, while code rarely types a plain object
by a class.

#### A type-guard function removes members unrelated to its target

```ts
interface Foo { a: number }; interface Foo2 extends Foo { b: number }
interface Bar { c: number }
function isFoo(v: unknown): v is Foo;
x: Foo2 | Bar; if (isFoo(x)) x      // Foo2
```

`Bar` also describes objects with a property `a`, such as
`const v: Bar = { a: 1, c: 2 }`, which `isFoo` accepts although it isn't a
`Foo2`. The true branch keeps the members that are subtypes of the target or
that the target is a subtype of, and removes the others. Keeping them would
make type-guard functions useless for telling the members of a union apart.

#### An optional call of a function expression runs the body first

```ts
x: string | null;
if (x) { ((v?: unknown) => { x = null; })?.(x) }   // null at the argument
```

JavaScript evaluates the arguments first, so `x` is a `string` there, but
following the runtime order would give a type disjoint from TypeScript's,
which rules would report against. A plain call runs its arguments first. The
difference needs an optional call of a function that is never nullish.

#### Destructuring evaluates its pattern before its value

```ts
x: string | number; a: unknown;
if (typeof x === "string") { [a = (x = 1)] = [x] }          // number at [x]
if (typeof x === "string") { const [b = id(x = 1)] = [x] }  // string | number at [x]
```

A destructuring assignment evaluates its pattern, with the defaults and the
objects and keys of its targets, before its value, and its defaults
unconditionally, then writes its targets. A declaration evaluates its
pattern, with its defaults only for `undefined`, before its initializer.
JavaScript evaluates the value first, so `x` is a `string` where the value
reads it, but following the runtime order would give a type disjoint from
TypeScript's. The difference needs a default that writes a variable that the
value reads.

#### A `let` counts as assigned in nested functions once any code assigns it

```ts
declare const c: boolean;
let x: string | null; if (c) x = "a";
const g = () => { if (x !== null) x };     // string
function h() { if (x !== null) x }         // string
```

When `c` is false, `x` is still `undefined` when `g` or `h` runs. A `let`
without an initializer starts with `undefined` added to its declared type in
the code that declares it, but in a nested function, including a closure,
only when nothing assigns it. A variable that one function assigns and
others read is common, as in
`let server: Server; function start() { server = listen(); }`, while code
rarely runs a function before the variable it reads is assigned.

#### Code between two elements of a destructuring pattern keeps the narrowing

```ts
s: { kind: "c"; x?: number; v: number } | { kind: "s"; x?: number; v: string };
const { kind, x = reset(s), v } = s; if (kind === "c") v    // number
```

Destructuring reads `kind`, then runs the default of `x`, then reads `v`, so
`reset` can change `s` before `v` is read. A computed key between the
elements, as in `{ kind, [key()]: x, v }`, runs code at the same point and
keeps the narrowing too. Stopping the narrowing there would lose it in
patterns such as `{ kind, items = load(), v }`.

### Augmentations and the files that Biome indexes

An augmentation adds interfaces to a declaration that another file owns. A
`declare module "./shapes"` block adds them to the exports of `./shapes`,
and a `declare global` block or a declaration file without imports or
exports adds them to the globals. TypeScript merges the augmentations of the
files in its program, which `tsconfig.json` selects with options such as
`files`, `include`, and `moduleDetection`. Biome doesn't read
`tsconfig.json`, so it merges the augmentations of every file that it
indexes instead, in the order of their paths.

The choices below are where not reading `tsconfig.json` makes a read
narrower than TypeScript's for a particular `tsconfig.json`.

#### An augmentation in a file that the program leaves out still applies

```ts
// shapes.ts
export interface Cat { kind: "cat" }
export interface Dog { kind: "dog" }
// cats.ts, which the program leaves out
export {};
declare module "./shapes" { interface Cat { purrs: true } }

pet: Cat | Dog; if (!("purrs" in pet)) pet
// TypeScript: Cat | Dog. Biome: Dog.
```

A file that Biome indexes can be missing from the program, as one in
another package of a monorepo or one outside the `include` patterns of
`tsconfig.json`. Biome merges the augmentations of such a file, whether it
is a declaration file or not, so `Cat` declares `purrs` and the false branch
of `in` removes `Cat`. The false branches of `instanceof` and of type-guard
functions remove members in the same way. With the augmentation above and
`isPurring(v: unknown): v is { purrs: true }`, the false branch of
`isPurring(pet)` gives `Dog`. With a class `Animal` in `shapes.ts` and
`declare module "./shapes" { interface Cat extends Animal {} }` in such a
file, so does the false branch of `pet instanceof Animal`. TypeScript gives
`Cat | Dog` in both. A rule such as `noUnnecessaryConditions` can then
report a check in that branch as unnecessary, as `case "cat":` in
`switch (pet.kind)`, although the program needs it.

Without `tsconfig.json`, Biome can't tell such a file from one that the
program contains. Treating what augmentations add as undecided would keep
these false branches as TypeScript gives them for every `tsconfig.json`, but
`in`, `instanceof`, and type-guard functions would then narrow nothing by an
augmented member or base, even in a project whose program contains the
augmentation.

#### Top-level interfaces become globals only in declaration files

```ts
// polyfills.ts, without imports or exports
interface String { format(): string }

interface Fmt { format(): string }
declare function isFmt(v: unknown): v is Fmt;
x: string | Fmt; if (isFmt(x)) x
// TypeScript: string | Fmt. Biome: Fmt.
```

TypeScript treats a file without imports or exports as a script, whose
top-level declarations are globals, so `polyfills.ts` gives every string a
`format` method and `string` matches `Fmt`. Biome adds a project's
interfaces to the globals only from `declare global` blocks and from
declaration files without imports or exports, so it ignores the interface
in `polyfills.ts` and removes `string` from the true branch. This read is
narrower than TypeScript's even over the files that Biome indexes. A rule
such as `noUnnecessaryConditions` can then report a check in that branch as
unnecessary, although `x` can be a string there.

Whether `polyfills.ts` is a script depends on the `moduleDetection` option
of `tsconfig.json`, which Biome doesn't read. With `"force"`, TypeScript
treats every file that isn't a declaration file as a module, so the
interface stays local to `polyfills.ts` and TypeScript gives `Fmt` too.
Reading such interfaces as globals would make Biome narrower under
`"force"` instead: with `x: string | Fmt | null`, the false branch of
`isFmt(x)` would give `null`, where TypeScript gives `string | null`. An
interface in a `declare global` block of a module, or at the top level of a
declaration file without imports or exports, is a global under every
setting, and Biome reads it.

#### Overloads that augmentations add are tried in the order of module paths

```ts
// a.ts
export interface X { f(v: unknown): 1 }
// b.ts
declare module "./a" { interface X { f(v: unknown): 2 } }
export {};
// c.ts
declare module "./a" { interface X { f(v: unknown): 3 } }
export {};

x: X; x.f(0)
// TypeScript, with c.ts before b.ts in the program: 2. Biome: 3.
```

TypeScript merges augmentations in the order of the program's files, which
`tsc --listFiles` prints, and tries the overloads of a later declaration
first. Biome has no program, so it orders the augmenting modules by path,
and the overloads of `c.ts` come first. When the program lists the two files
in the order of their paths, TypeScript gives `3` too. A call selects
another overload only when overloads from two augmenting modules both
accept its arguments. A rule such as `noUnnecessaryConditions` can then
report `case 2:` in `switch (x.f(0))` as never matching, although the
program selects the overload that returns `2`.

The order of paths doesn't depend on the order in which Biome finds the
files, so every run gives the same result. Treating such a call as
undecided would lose its result even where the program lists the files in
the order of their paths.

## Limits

This section gives the rules that decide what narrows, topic by topic, and
the places where Biome narrows less than TypeScript. Where a limit makes a
read narrower than TypeScript's, it says so.

### Which reads narrow

- A variable that a destructuring default can supply, as `a` in
  `const { a = 0 } = o`, never narrows, because its declared type ignores the
  default.
- A `var` whose name another declaration of its scope also declares, as a
  second `var`, a function, or a parameter of its function, never narrows,
  because Biome resolves every reference to the last declaration while each
  declarator writes the variable. So
  `var x: string | number = 1; var x: string | number = "a"; x` reads
  `string | number`, where TypeScript gives `string`.
- A variable whose declared type includes a part that inference cannot
  determine never narrows.
- A property of `any`, as `x.v` or `a` in `const { a } = x`, never narrows,
  and Biome reads it as Unknown where TypeScript gives `any`.
- An execution root gets no narrowing at all when its own code, outside
  nested functions, contains a `with` statement, a syntax error, or a `break`
  to the label of a `try` statement, as in
  `out: try { break out; } finally {}`, or when it is very large or deeply
  nested.
- Class and enum declarations, namespaces, `import x = require()`, and
  `declare` statements don't stop narrowing. A read in a class or enum
  declaration, such as in its `extends` clause or in the initializer of an
  enum member, has the type from just before the declaration, narrowed
  further only by the rest of the expression that contains it, as in
  `x && x.length`.
- A namespace narrows its own variables but never sees the narrowing around
  it.

### Closures, inline code, and class members

- A closure continues the narrowing of a `const` that isn't an import. It
  continues the narrowing of a parameter, a `catch` variable, or a `let` only
  when the closure is created after the last assignment of the variable in
  the function that declares it. An assignment counts until the
  end of the outermost variable or expression statement, `if`, loop, `with`,
  `switch`, `try`, or class declaration around it, so
  `if (c) { x = "a"; const h = () => x; }` keeps the declared type, as in
  TypeScript. An assignment in any other function, including a closure or
  inline code, means that no closure continues.
- An import, a `var`, a function, and a class never continue in a closure.
  Neither does the narrowing of a property path or a path on `this`, as in
  `if (o.v) () => o.v`, which reads `string | null`, as in TypeScript. The
  narrowed type of `o` itself continues, so `o.v` reads its property on the
  narrowed object.
- A `let` declared at the top level of a file never continues in a closure,
  because other scripts can write a top-level `let` of a script, and Biome
  can't tell a script from a module. TypeScript continues it in a module.
- A closure created in unreachable code, as after `return`, in the branch of
  `if (false)`, or after a call that never returns, starts from the declared
  type, as in TypeScript.
- A function declaration never sees the narrowing around it, and neither
  does a closure nested in it. Neither do a method, accessor, or constructor
  of a class declaration, nor a constructor of any class. A method or
  accessor of a class expression continues the flow where the class is
  created, as in TypeScript.
- A read in a class field, including its decorators and computed key, never
  narrows, and neither does a read in the decorators or the computed key of a
  method or accessor. TypeScript narrows the decorators and computed keys of
  a field without an initializer in any class, and those of a method or
  accessor in a class expression.
- A static block of any class runs where the class is evaluated, after its
  `extends` clause and in member order.
- A function or arrow expression runs inline when it is neither `async` nor
  a generator and a call invokes it where it is defined, through
  parentheses, also with `?.()` or under `!` or `void`. Its arguments run,
  then its parameters and body, and the code after the call continues from
  the end of the body joined with each `return`. A closure created inside it
  continues the flow from there, while a write in it still counts as a write
  in another function for the closures that read the variable, as in
  TypeScript. A call through `.call` or `new`, and a function called later,
  run apart from the code around them.
- The parameters of inline code get no type from its arguments, so
  `((y) => y)(x)` reads `y` as Unknown, where TypeScript gives the type of
  `x`.
- Inline code in a field initializer, a decorator, a computed key, or an
  export declaration that isn't a statement, as in
  `export const v = (() => ...)();`, is an execution root of its own. So is
  inline code directly in an execution root that gets no narrowing at all.
- In a function nested in the code that declares it, a `let` or a `var`
  starts with `undefined` added to its declared type only when it has no
  initializer and nothing writes it. TypeScript assumes such a `var`
  assigned there, so `var w: string | null; function f() { if (w !== null) w }`
  reads `string | null`, where TypeScript gives `string`.

### Writes

- Writes in the body of another function, method, accessor, or constructor
  don't count, nor do writes through another variable, nor writes outside the
  root of a variable that an enclosing root declares.
- A variable or a path doesn't narrow anywhere in its execution root when
  code that runs with the root, but outside its flow, writes it: an `async`
  function or a generator called where it is defined, a field initializer, a
  decorator, a computed key, or a parameter default of a method, including
  inline code inside such code. So does a write in unreachable code, a write
  in the initializer of an exported declaration, which Biome doesn't
  evaluate, and a write through a target whose object Biome doesn't map to a
  variable or a path, as in `(o as T).v = x`, `o!.v = x`, or `super.v = x`.
- Every `this` and `super` in a write stands for the root's `this`, so
  `this.v = x` and `super.v = x` both write `this.v`, even where `this` is
  another object, as in a static block or in a function expression called
  where it is defined. There such a write has no place in the flow, so it
  stops `this.v` from narrowing anywhere in the root.
- The type that a write or an initializer assigns is the type of its value
  without narrowing. `y = x` after `if (x)` assigns the declared type of `x`,
  and `y` in `if (x) { const y = x }` has the declared type of `x`. An object
  or array literal keeps the literal types of its properties, as TypeScript
  does where the declared type of the target has literal types.
- A call reduces the assigned type when its callee's return type comes from
  an annotation: a function with a return type annotation, a variable or
  parameter annotated with a function type, a global, an import of such a
  function or of a declaration in a declaration file, or a method that every
  type of its object, typed without narrowing, gets from such a declaration.
  Such a method belongs to a primitive, a tuple, a global of the standard
  library that only declaration files merge interfaces into, a type that a
  declaration file declares and whose parents are such types, or a class or
  interface that declares the method with a return type annotation, or
  declares none and inherits it from such a type.
- A call also reduces when its callee is a local function without a return
  type annotation and isn't a generator: a function declaration, or a
  function or arrow expression that initializes a variable without a type
  annotation. Each value that the function returns must reduce by these
  rules and read no variable whose type evolves with its writes. Biome
  follows such calls through at most four nested functions, and infers their
  return types as TypeScript does, with `undefined` when a path reaches the
  end of the body, so `function g() { if (c) return 1; }` returns
  `1 | undefined`.
- Other calls give the variable its declared type, where TypeScript reduces
  by the return types it infers: a method without an annotation, as
  `x = k.m()` with `class K { m() { return 1; } }`, an overloaded method whose
  implementation has none, a method of an object literal, an imported
  function whose return type is inferred, inline code, and a call along an
  optional chain whose object is such a call, as `o.m()?.trim()`. So does
  `typeof`, which Biome types as one kind.
- A write keeps the declared type when it isn't a union or when the value is
  `any`, and gives `never` for a `never` value, which Biome reads as Unknown.
  `boolean` stays whole, so `let b: boolean | undefined = true` reads
  `boolean`, where TypeScript gives `true`.
- These writes keep the declared type, where TypeScript narrows: the value
  of an assignment expression, as `y` in `x = y = 1`, an empty array literal,
  a missing element of an array literal, and an element of an array that a
  destructuring target reads, which Biome types as possibly `undefined`. So
  `[x] = xs` with `xs: string[]`, `[[x]] = [["a"]]`, and `[x = 1] = []` keep
  the declared type of `x: string | number`.
- A write behind `as`, `satisfies`, or `<T>`, as `(x as T) = v`, gives the
  declared type, where TypeScript ignores the write.
- A computed key of a destructuring target that isn't a literal of the kind
  that a property path accepts gives its target the declared type.
- Compound assignments, `++`, and `--` give the primitive type of the literal
  types of the type before the write, so `x: 1 | 2` reads `number` after
  `x++`, as in TypeScript. An assignment whose right operand has a shift or
  arithmetic operator, as `x = x * 2`, gives the primitive type of the
  literal types of the declared type. The right operand is typed without
  narrowing, so `x = x + 1` with `x: number | string = 1` gives
  `number | string`, where TypeScript gives `number`.
- In a logical assignment, the right operand runs only where the target is
  nullish for `??=`, falsy for `||=`, or truthy for `&&=`, and sees the
  target tested there. A test of a discriminant narrows the object that holds
  it, as `s` in `s.kind ||= f(s)`. The right operand is typed without
  narrowing.

### Variables whose type evolves with their writes

- A `let` or `var` evolves when it is declared as a name without a type
  annotation, not exported and not `declare`. It starts at `undefined` and
  takes the type of each value assigned to it, with its literal types
  widened, so `let x = null; if (c) x = "a"; x` reads `string | null`. Paths
  join with a union, and runtime tests narrow the type that the writes give.
  A variable that nothing writes narrows as any other variable.
- An evolving array is a `let`, `const`, or `var` declared the same way and
  initialized with `[]` without parentheses. It starts with the element type
  `never`. Each argument of `push` or `unshift` on the variable, also written
  `x?.push(v)`, `(x).push(v)`, or `x!.push(v)`, and the value of an
  assignment to an element with a number literal key, as `x[0] = v`, add
  their types, with literal types widened, to the element type. A string key
  adds nothing, assigning `[]` starts again at `never`, and assigning another
  array gives its element type. Paths join the element types, and runtime
  tests don't narrow the array. Inline code adds to the array in the flow,
  and a callback adds nothing.
- Biome follows the writes only in the execution root that declares the
  variable. In other functions, including closures, a read keeps the
  declared type, where TypeScript reports an implicit `any`.
- A read keeps its declared type where a write that reaches it assigns or
  adds a value that Biome may type otherwise than TypeScript: a call whose
  return type can't reduce a write, a function, an object with a method, an
  object type or a type argument with a literal type, as `{ k: id(1) }`,
  `[]`, `any`, `unknown`, `void`, or a value that reads another such
  variable. So does a compound assignment, `++`, `--`, or an assignment whose
  right operand has a shift or arithmetic operator, as `x = x + 1`, and, for
  an array, a spread argument or a value through a key that isn't a number
  literal, as `x[i] = v` with `i: number`.
- A read keeps its declared type where the values assigned are all `null`, or
  all `undefined`, so `let x; x = null; x` keeps it, where TypeScript gives
  `null`. An evolving array that holds only `null` or only `undefined`
  values, or to which nothing was added, reads `unknown[]`, where TypeScript
  gives `null[]`, `undefined[]`, or `any[]`.
- The declared type of an evolving variable is the type of its initializer
  and ignores the writes in its own function, so a read that keeps it can be
  narrower than TypeScript: `let x = null; x = o.load(); x` reads `null`,
  where TypeScript gives `number` for `const o = { load() { return 1; } }`.
- An `async` function or a generator called where it is defined that adds to
  an array makes every read of the array in the execution root keep its
  declared type.

### Reads before a variable is assigned

- A `let` without an initializer, outside the head of a loop and without
  `!`, and a `var` without `!`, which a read can reach before its declarator,
  start with `undefined` added to their declared type, as in TypeScript. So
  does a path on `this` in a constructor, and in an arrow function that the
  constructor calls where it is defined, when its field has no initializer
  and no `!` and is neither `static`, `declare`, nor `accessor`.
- A read where `undefined` remains, although the declared type lacks it, has
  the declared type. So `let x: string | null; if (x !== null) x` reads
  `string | null`, as in TypeScript. A test that removes `undefined`, as
  `typeof x === "string"`, and an assignment narrow as usual.
- A `var` narrows as a `let` does. Each declarator with an initializer, and
  each iteration of a `for...in` or `for...of` loop that declares it, writes
  it.

### Property paths

- A path narrows only when it starts at a variable that can narrow,
  including a module variable read in a function, or at `this` where `this`
  narrows.
- An element access is part of a path only when its key is a string literal
  without escapes or a decimal integer literal from 0 to 2^53 - 1 without
  leading zeros or separators. Other keys don't narrow: a `const` key, as
  `o[k]` with `const k = "a"`, a template literal, `0x0`, or `"\u0061"`,
  which TypeScript matches to the same property.
- `o["#v"]` is never the same path as `o.#v`.
- A runtime test of a chain that starts at a parenthesized optional chain,
  such as `(x?.z)?.w` or `(o?.v)?.w === 1`, narrows neither `x` nor its
  paths. TypeScript narrows `x` through a truthiness test, and `x.z` through
  a comparison.
- A write to a prefix of a path, as `o = q` or `o.v = q` for `o.v.w`, gives
  the path its declared type at the write, and so does a write of any
  property of a prefix through a key that isn't a literal of the kind above,
  such as `o[k] = x` or `o[-0] = x`. A write through such a literal, as
  `o["v"] = x`, writes only the path it names.
- `delete o.p` assigns `undefined` to `o.p`, as in TypeScript.

### `this`

- A path on `this` narrows in a class method or accessor that isn't
  `static`, in an object literal method or accessor, in a constructor, and in
  an arrow function whose `this` comes from one of them or from the
  initializer of a field that isn't `static`. The method or accessor must
  not declare a `this` parameter.
- The class or object must have a name that Biome can find: a class
  declaration, a class expression with its own name or assigned as the
  initial value of a variable, as in `const C = class {}`, or an object
  literal assigned as the initial value of a variable, as in
  `const o = { ... }`. So a path on `this` doesn't narrow in an object literal
  passed to a call.
- A path on `this` never narrows in a `static` method, accessor, field, or
  block, where Biome types `this` as an instance instead of the class, in a
  function declaration or expression, or outside any function. Neither does
  a path on `this` in the decorators or `extends` clause of a nested class,
  or in the computed key of a nested object literal method, which Biome types
  as the nested class or object.
- `this` itself narrows where its paths do. Biome types `this` as an instance
  of its class, so a target that the class doesn't relate to gives the
  intersection, as `T1 & { tag: 1 }` for `isTagged(): this is { tag: 1 }` on
  `n: T1`, or `R & { ready: string }` after
  `asserts this is { ready: string }`, where TypeScript writes
  `this & { ... }` inside the class.

### Destructuring

- A test of an element narrows the other elements of the same pattern only
  when a `const` declares the pattern, including the head of a `for...of`
  loop, or a parameter does, including a rest parameter such as `...[k, x]`
  and a parameter with a type annotation or a default. No variable that the
  declaration binds may be written.
- The tested and the read elements must be direct elements of the same
  pattern, without a default, and not rest elements. In
  `const { a: { kind, v }, b } = w`, a test of `b` doesn't narrow `v`, and in
  `const { kind, ...rest } = s`, a test of `kind` doesn't narrow `rest`.
  Renamed elements, nested patterns, and array patterns narrow.
- An element of an array pattern from index 16 on neither narrows the
  pattern nor holds a discriminant.
- A nested pattern narrows only when no enclosing pattern supplies a
  default, as `{ a: { kind, v } = d }` does.
- The destructured value narrows as the object of a test of its property
  does, except that its `null` and `undefined` members don't count, because
  destructuring throws on them. The element's own tests then narrow the
  element further.
- Biome can't infer the types of the elements of a pattern whose initializer
  ends with `!`, as in `const { kind, v } = s!`, or of a parameter
  destructured in a function that only a variable's annotation types, as in
  `const f: (s: S) => void = ({ kind, v }) => {}`, so they don't narrow.
- A function whose parameters only a contextual type with a rest parameter
  of a union of tuples types, as
  `const f: (...args: ["a", number] | ["b", string]) => void = (k, p) => {}`,
  gets no types for `k` and `p`.
- A variable of a destructuring pattern in a `catch` clause, as `message` in
  `catch ({ message })`, is Unknown, where TypeScript gives `any` after an
  error.

### Discriminants

- A test of a property, such as `s.kind === "c"`, `s["kind"]`,
  `typeof s.kind === "string"`, or `t[0] === "a"` on a tuple, narrows the
  object that holds it. Only that
  object narrows: `o.a.kind === "c"` narrows `o.a`, not `o`. The object must
  be a variable or a property path that can narrow, not `this`.
- The object's declared type must be a union with at least two object types
  in which every member other than `null` and `undefined` declares the
  property. The property must be a discriminant: in at least one member its
  type is a literal type, `null`, `undefined`, `boolean`, or a union of them,
  and the members don't all give it the same type.
- An instance of a generic alias, as `r: Result<number>` with
  `type Result<T> = { ok: true; value: T } | { ok: false }`, isn't a union
  for this purpose, so it keeps its declared type.
- A member whose property type is an object type stays after every test that
  leaves a value other than `null` or `undefined`, because Biome doesn't
  compare object types with other types. With
  `o: { k: { a: 1 }; x: 1 } | { k: "x"; y: 2 }`, `o.k === "x"` keeps both
  members, where TypeScript keeps only the second.
- A call to a type-guard function on the property, as in `isC(s.kind)`,
  doesn't narrow the object, as in TypeScript. A write to the property, as in
  `s.kind = "b"`, keeps the object narrowed, because the written value must
  fit the property of the narrowed object.
- A comparison of a discriminant with a value other than a unit value or a
  variable with a type annotation, such as a call, doesn't narrow the
  object.
- A `default` keeps a member whose discriminant is a union of case values.
  With `s: { kind: "a" | "b"; x: 1 } | { kind: "c"; y: 2 }`, `default` after
  `case "a": case "b":` keeps both members, where TypeScript keeps the
  second.
- A `const` holds a property when its initializer reads the property, as in
  `const k = s.kind`, `s?.kind`, `s["kind"]`, or `t[0]`, or when it is an
  element of a destructuring pattern that the declarator binds directly, as
  in `const { kind } = s`, `const { kind: k } = s`, `const { "kind": k } = s`,
  or `const [k] = t`. The initializer must be a variable or a property path,
  not `s!` or `s as S`. The `const` must have no type annotation and must
  not be exported, and the element must have no default. TypeScript also
  accepts `s!.kind`, keys spelled differently from the property, such as
  `s["\u006bind"]`, `t[0x0]`, `{ "\u006bind": k }`, or
  ``{ [`kind`]: k }``, and a `const` exported by an `export { ... }` list.
- Through `s?.kind`, no test of `k` shows that `s` isn't nullish, so with
  `s: Circle | Square | undefined`, `k === "c"` gives `Circle | undefined`, as
  in TypeScript.
- A test of a `const` that holds a property, inside an aliased condition,
  narrows the object only when the object is a variable that can't change.
  A closure created after such a test sees it, and a test inside a closure
  narrows the object there, as in
  `const k = s.kind; const g = () => { if (k === "c") s }`.

### Aliased conditions

- The `const` must have an initializer and no type annotation, must declare a
  name rather than a destructuring pattern, and must not be exported.
  TypeScript also accepts a `const` exported only by an `export { ... }`
  list.
- The condition narrows only variables whose value can't change between the
  `const` and the test: parameters, `catch` variables, and variables declared
  with `const` or `let` that are never written and not exported. A `let` at
  the top level of a file doesn't count, because other scripts can write a
  top-level `let` of a script, and Biome can't tell a script from a module.
- A property path, such as `o.v`, `o.a.v`, or `this.v`, narrows only when it
  starts at such a variable or at `this` and every property along it is
  readonly in every member, with `readonly`, `Readonly<T>`, or `as const`, as
  the deliberate choices describe. The elements of a `readonly` tuple count
  as readonly. A property of an intersection is readonly only when every
  operand that declares it declares it readonly, as in TypeScript.
- A path on `this` narrows only through an aliased condition that the same
  function declares, not through one of the enclosing method when the path
  is in an arrow function.
- A test of a property in an aliased condition narrows the variable that
  holds the property, as in `const ok = s.kind === "c"; if (ok) s`, whether
  the property is readonly or not, unless the execution root writes it.
- A path keeps its declared type through an aliased condition, where
  TypeScript takes it as readonly, in these cases:
  - A property along it is a getter, because Biome doesn't collect setters,
    so the getter may have one.
  - A property along it is a `static readonly` field.
  - A property along it is an element of an array under `as const`, which
    Biome types as a tuple whose elements aren't readonly.
  - The type of the object inherits the property through `extends`,
    including the `extends` clause of an interface merged into a class, or a
    module augmentation declares it, because an augmentation in a file that
    Biome doesn't index can declare the property without `readonly`.
  - An object along it is a generic or a primitive, as for `s.length`.
  - The declared type of an object along it has a member that doesn't declare
    the property readonly, even where narrowing has removed that member at
    the read, as `if (o.kind === "a")` does in
    `o: { kind: "a"; readonly v: string | null } | { kind: "b"; v: string | null }`.
  - The execution root tests an object along the path, anywhere, with
    `instanceof`, a type-guard function, or `typeof` with a kind other than
    `"object"` and `"undefined"`. Such a test can replace a member with a
    type whose property isn't readonly, as `o instanceof Derived` does when
    `Derived` redeclares a readonly property of its parent without
    `readonly`.
- Aliased conditions inline into one another at most five levels deep, as in
  TypeScript, so in `const c2 = c1; ...; const c6 = c5; if (c6)`, only the
  truthiness of `c1` is tested.
- A comparison of an aliased condition with `true` or `false`, as
  `if (isS === true)`, applies its condition too.

### Equality and comparisons

- A unit value is one of these, written in place or held by a `const`: a
  string, number, bigint, or boolean literal, `null`, `undefined`, `void`
  with any operand, `-` or `+` directly before a number literal, as in `-1`,
  a template literal without a tag and without substitutions, and a member
  of an enum whose value Biome computes, read as `E.A` or `E["A"]` through
  the name of the enum, local or imported.
- A variable holds a unit value when it is a `const` without a type
  annotation whose initializer is one, also through at most 16 other such
  `const`s, as `K2` in `const K = "a"; const K2 = K`, or when a named or
  default import binds it to such a `const` of another module, also through
  re-exports.
- TypeScript gives `-(1)` the type `number`, and Biome gives `-1n` the type
  `bigint`, so neither is a unit value.
- A comparison narrows by any other value only through the value's declared
  type, when TypeScript gives the value that type too: a variable or a
  parameter with a type annotation, a property path that starts at one, as
  `o.k` with `o: { k: "a" }`, an `as const` expression, or an import of a
  declaration with an explicit type, as `export const K: "a" = "a"`. Where
  the two are equal, the members that may equal a value of that type stay,
  and a primitive type becomes the value's literals of its kind, so
  `x: string | number` compared with `y: "a" | 1` gives `"a" | 1`. Where they
  differ, the value narrows only when its declared type has a single value.
  Both operands narrow when each is such a value for the other, and a value
  compared with itself narrows nothing.
- TypeScript narrows by the value's type at the comparison, which can have
  fewer values. With `x: "a" | "b"` and `y: "a" | "b"`,
  `if (y === "a") { if (x !== y) x }` gives `"a" | "b"` in Biome and `"b"` in
  TypeScript.
- Other values narrow nothing, even where TypeScript gives them a unit type,
  because Biome may infer their types narrower than TypeScript does: a
  variable without a type annotation, as `let y = "a" as const`, a `const`
  without a type annotation that holds an `as const` expression, a call that
  returns a literal type, a template literal with substitutions, a member of
  a namespace, `export default "a"`, a member of an enum read as `E?.A` or
  `(E).A`, a member whose value Biome doesn't compute, and a variable named
  `undefined` or `globalThis`, which Biome types as the global of that name.
  A `case` narrows only by a unit value.
- A comparison with an imported value narrows no receiver of an optional
  chain, where TypeScript narrows it as for a local value.
- A comparison of `x?.y` narrows `x` in the branch where `x?.y` can't be
  `undefined` only for a unit value or a value whose declared type excludes
  `undefined`, and `null` for `==` and `!=`, by the rules above. TypeScript
  also narrows `x` for an imported `const` or a variable without a type
  annotation.
- A comparison of a condition with `true` or `false`, strict or loose,
  narrows nothing when the condition, without parentheses and `!`, is `&&`,
  `||`, or `??`, as in TypeScript.
- `x.constructor === C` narrows only when `C` is a dotted name whose type is
  a class, or a constructor whose `prototype` property has a type other than
  `any`, `Object`, or `Function`. The other branch keeps the type. An
  instance of a class stays only when its class is `C` itself, not a class
  derived from it, and another member stays when it is assignable to the
  type of `C.prototype`, as `string` for `String`. TypeScript removes a member
  that isn't a subtype, where Biome keeps one that is assignable, and removes
  `unknown`, which Biome keeps. The receivers of an optional chain narrow
  too, as in `x?.constructor === Foo`.
- A `const` that a condition narrows to `never`, as `D` in
  `const D = false; if (D) { if (x === D) x }`, still holds its value in
  Biome, so `x` reads `false` there, where TypeScript gives `boolean`. Biome
  is narrower than TypeScript here, in code that TypeScript types as
  impossible.

### Enums

- A member of an enum has the literal type of its value, and the enum, used
  as a type, the union of its members. TypeScript gives each member a
  literal type of its own, with the same values. So a read of `E.A` in
  `enum E { A, B }` displays as `0`, where TypeScript displays `E.A`, and
  `e: E` as `0 | 1`.
- A string literal is assignable to a string enum in Biome, as `"a"` to `S`
  in `enum S { A = "a" }`, where TypeScript reports an error. Rules that tell
  an enum's type from its values' primitive type, such as one that reports
  converting a string to a string, see a member of a string enum as a string.
- Biome computes a member's value from literals, operators, and earlier
  members of the enum. A member whose value it doesn't compute is `number`
  when TypeScript requires a number, as for `"c".length`. A computed integer
  outside 0 to 255 and a computed number that isn't an integer are `number`,
  and a concatenation or a template literal with substitutions is `string`.
  A reference to a constant outside the enum, such as a `const` or a member
  of another enum, is Unknown, because it can be a string.
- A member without an initializer of an ambient enum that isn't `const`, as
  in `declare enum D { P }`, is `number`, because the code that defines the
  enum chooses its value. Such a member, and any member whose value Biome
  doesn't compute, narrows nothing in a comparison.
- An enum whose name its scope declares more than once, as two declarations
  of the enum or an enum and a namespace, which TypeScript merges, is
  Unknown, with its members.
- Where a member initializes a mutable location, as in `let n = E.A`,
  TypeScript widens its type to the enum's type, `E`, and Biome to the
  primitive type of its literal, `number`, which is wider.

### Conditions, truthiness, and `typeof`

- `x!`, `satisfies`, and the comma operator inside a condition, as in
  `if (x!)` or `if ((0, x))`, narrow nothing.
- A falsy branch keeps object types that Biome can't compare with
  primitives, and generics, where TypeScript removes `T extends Foo`.
- A conditional expression has the types of both branches, whatever its
  test, as in TypeScript. A branch adds nothing when its type is `never`, as
  for a call to a function that never returns, or when it only reads the
  variable that the test reads and the test leaves that variable no value
  there, as `x` in `x ? 1 : x` with `x: Foo`. Another read that TypeScript
  narrows to `never` in a branch, as `x` in `x !== null ? 1 : x` with
  `x: string`, is Unknown in Biome, which makes the whole expression
  Unknown.
- A generic keeps `T` where its constraint leaves no value, as in the falsy
  branch of `T extends object`, where TypeScript gives `never`. Where the
  constraint has an `undefined` member, `!== null` gives
  `T & ({} | undefined)`, where TypeScript gives `T & {}`. Biome writes these
  intersections distributed, as `(T & {}) | (T & undefined)`.
- Falsy and `typeof` tests compare an object type with the types that give
  primitives and functions their members, such as `String` and `Function`,
  from TypeScript's ES2015 library files and the interfaces that the project
  merges into them.
- `typeof` with a primitive kind keeps an object type that Biome can't
  compare with the primitive: an instance of a class whose members the
  primitive has, because the class can declare private members; a type with
  an index signature; a type with a member whose type differs from the
  primitive's member of the same name; a member that a library newer than
  ES2015 adds to the primitive, such as `padStart` of strings or
  `description` of symbols; and `toString`, `toLocaleString`, or `valueOf`
  for `"bigint"`. With `"function"`, instances of classes derived from a
  built-in class stay too.
- A case of `switch (typeof x)` must be a string literal or a template
  literal without substitutions, as in TypeScript. A case with an invalid
  kind, such as `"strin"`, keeps the type of `x`, where TypeScript gives
  `never`. As in TypeScript, `switch (typeof x)` narrows neither the object
  that holds a property, as `o` for `typeof o.k`, nor the pattern that
  declares a variable, and narrows the receivers of an optional chain, as in
  `switch (typeof x?.y)`, only where every case of the group shows that the
  operand isn't `undefined`.
- `typeof x === "object" || typeof x === "function"` on `unknown` gives
  `object | null | Function` where TypeScript gives `object | null`, which
  has the same values.
- A `void` member makes a narrowed read Unknown, because a function whose
  return type is `void` can return any value.

### `in`

- `"a" in x` and `0 in x` narrow `x` when its declared type has an object
  type or a type parameter, or is `unknown`, and `x` is a variable, `this`, or
  a property path that can narrow. Only the value that `in` looks into
  narrows: `"a" in s.k` narrows `s.k`, not `s`, as in TypeScript.
- The key must be a string literal without escapes or a decimal integer
  literal of the kind that a property path accepts. TypeScript also narrows
  with a `const` of a literal type, a template literal, `"\u0061"`, `0x0`,
  and a key that starts with `#`, such as `"#a"`.
- The true branch keeps the members that declare the property, even as
  optional, or whose index signature accepts its name, and removes the
  others when some member does, as in TypeScript. The false branch removes
  the members that declare the property as required.
- A member stays in both branches, and no intersection is made, when Biome
  can't tell whether it has the property: a primitive, a function, a tuple,
  an array, an intersection, a generic, an instance of a class, because
  Biome doesn't collect the setters and index signatures of classes, or a
  built-in type such as `Date` or `Error`, because Biome's built-in types come
  from the ES2015 library files, which lack members that newer ones add, such
  as `at` of arrays and `cause` of errors. So does an object type with a call
  or construct signature, a setter, a spread, a computed key, or a member
  whose name is spelled with an escape or as a number other than a decimal
  integer, such as `"\u0061"` or `1.0`. So `x: { a: 1 } | Date` gives
  `{ a: 1 } | Date`, where TypeScript gives `{ a: 1 }`, and
  `x: { b: 1 } | string[]` keeps its type, where TypeScript gives
  `(string[] | { b: 1 }) & Record<"a", unknown>`. With `class P { p = 1 }` and
  `class Q { q = 1 }`, `x: P | Q` gives `P | Q` for `"p" in x` and `Q` in the
  `else` branch, where TypeScript gives `P` and `Q`.
- `#p in x` counts a class expression only when Biome types it, and never an
  anonymous default export. A static name narrows `x` to the class itself,
  as `typeof C`.
- `in` and `hasOwnProperty` don't remove a missing optional property under
  `exactOptionalPropertyTypes`.

### `instanceof`

- `x instanceof C` narrows `x` when `x` is a variable or a property path that
  can narrow, and its declared type is `unknown` or has a member other than
  `null`, `undefined`, and primitive types. Only the tested value narrows:
  `o.v instanceof C` narrows `o.v`, not `o`, and a test of an element of a
  destructuring pattern doesn't narrow the other elements. In an optional
  chain, as in `o?.v instanceof C`, the true branch also removes `null` and
  `undefined` from `o`, as in TypeScript.
- The instance type of `C` is an instance of `C` when `C` is a class, and
  otherwise the type of its `prototype` property or the return types of its
  construct signatures, as `new () => Foo` gives `Foo`. A union of
  constructors gives the union of their instance types.
- Members are compared with the instance type by their declarations, as in
  TypeScript: a class or an interface is derived from another when its
  `extends` clauses lead to that declaration, whatever their type arguments,
  so `Box<string>` is derived from `Box`, and arrays and tuples are derived
  from `Array`. An object type literal, a function type, or a primitive is
  derived from no class, and a type parameter is derived from what its
  constraint is derived from. The true branch keeps the members derived from
  the instance type, replaces a member that the instance type is derived
  from with the instance type, and removes the others. A type parameter
  without a constraint, or whose constraint `A` is derived from, becomes
  `T & A`. When no member remains, the true branch gives what a type-guard
  function gives, so `x: Bar` with `interface Bar { c: number }` becomes
  `Bar & A`. The false branch removes the members derived from the instance
  type only when `C` has a single instance type that is an object type other
  than `{}`.
- `x instanceof C` keeps the type of `x` when `C` is `Object` or `Function`,
  as in TypeScript, and, where TypeScript narrows, when the type of `C` is
  `any` or has a generic construct signature, as in `new <T>() => Box<T>`,
  and when `C` declares or inherits a static `[Symbol.hasInstance]` method,
  or a static member with a computed key that Biome can't name. TypeScript
  narrows through a `[Symbol.hasInstance]` method with a type predicate.
- Two classes with the same name and members in two modules, such as
  `class K { k = 1 }` in `./a` and in `./b`, can't be told apart from one
  class that Biome reaches through two modules, so a test of one keeps the
  type of a value of the other. Two such classes in two namespaces of one
  module are two classes, as in TypeScript, so `x: P1.K | string` keeps
  `P1.K | string` in the `else` branch of `x instanceof P2.K`. When no member
  remains in the true branch, Biome doesn't count an instance of a class as a
  subtype of an instance of another class with the same members, because a
  class can declare private members, so the true branch keeps
  `P1.K | string`, where TypeScript gives `P2.K`.

### Type-guard functions

- A call narrows through its callee only when the callee is a dotted name:
  an identifier or `this`, followed by property accesses with `.` or `?.`,
  with or without parentheses, as `isString(x)`, `u.isString(x)`, or
  `(this.check)(x)`. A call through an element access, as
  `u["isString"](x)`, or through the result of a call, as `get()(x)`,
  narrows nothing, where TypeScript narrows through any callee whose type
  has a single signature with a type predicate. The call must pass a subject
  as the tested argument, or call a method of a subject that a `this is T`
  predicate tests. A property narrows as the argument, but the receiver of
  an optional chain doesn't: `isString(x?.y)` keeps `undefined` in `x`.
- A type-guard function that the module declares with `function` or
  `declare function`, or as an arrow or function expression that
  initializes a `let` or `const` without a type annotation, narrows as its
  declaration says when nothing reassigns it and it has one signature that
  isn't generic, `async`, or a generator. The tested parameter is named,
  isn't a rest parameter, and has no default value. The call isn't optional,
  has no type arguments, passes a variable as the tested argument, and has no
  spread argument before or at the tested one.
- Any other callee, such as an import, a method, a variable or a parameter
  with a type annotation, a variable that holds a function, or a function
  that the module declares with type parameters or overloads, narrows
  through the signature of its type that TypeScript uses. The type, without
  `null` and `undefined`, has a single signature that isn't generic, or the
  call selects one of its signatures as TypeScript's overload resolution
  does, when some signature returns a type predicate, an assertion, or
  `never`. The tested parameter is named, isn't a rest or an optional
  parameter, and gets an argument before any spread argument. A callee of
  `any` and a union of functions narrow nothing. An optional call, as
  `g?.(x)`, narrows only where it is truthy, as in TypeScript.
- A target of `any` or `unknown` narrows nothing. So for `x: any`, the
  `else` branch of a type-guard function whose target is `any` keeps `any`,
  where TypeScript gives `never`.
- Neither Biome nor TypeScript checks the body of a type-guard function.
- A call selects a signature from the types of its arguments at the call,
  where a test before the call narrows them. The signatures are tried in
  order with the subtype relation, under which `any` is a subtype only of
  `any` and `unknown`, and, when none applies, with assignability; the first
  one that accepts the arguments decides, as in TypeScript. Where Biome
  can't tell whether an earlier signature accepts the arguments, as for an
  instance of a class passed for a parameter of another class, the call
  narrows nothing. A call with a spread argument narrows nothing, and
  neither does a signature with a `this` parameter among several.
- A generic signature infers a type argument from the argument for a
  parameter typed with the type parameter itself, as `sample: T`, from the
  argument without its `null` or `undefined` members for a parameter typed
  `T | null` or `T | undefined`, and from the target of the type predicate of
  a callback for a parameter typed as a callback whose type predicate
  targets the type parameter, as `predicate: (value: T) => value is S`.
  Explicit type arguments, as `isNonNull<string>(x)`, work too. The inferred
  type must satisfy the constraint, with the type arguments of an array or
  another instance whose method the call reads, as `S extends T` with `T` the
  array's element type. A literal type that TypeScript would widen there, as
  `"a"` for `v is T[]`, is widened. A type parameter that no parameter and no
  target mentions needs no type argument. Any other parameter that mentions
  a type parameter, a type argument that two arguments infer differently,
  and a constraint that Biome can't check make the call narrow nothing.
- A target that is an alias of an intersection takes the call's type
  argument, so `v is NonNullable<T>` gives `string` for `x: string | null`
  and `T & {}` for `x: T` in a generic function. With
  `function isNonNull<T>(v: T | null): v is T`, `x: T | null` gives `T`.
- A read in a loop that a call reaches again before the read gets the type
  that the calls before it in the code give it, which is never narrower than
  TypeScript's.
- A target that isn't a primitive is compared with each member of the type
  of `x` in TypeScript's subtype relation. The true branch keeps a member
  that is a subtype of a member of the target, replaces a member with a
  member of the target that is a subtype of it, as `unknown` or `object`
  becomes `Foo`, and removes the others. A type parameter is a subtype of
  what its constraint is a subtype of, and no other type is a subtype of it.
  A member of the target that no member relates to gives its intersection
  with each type parameter whose constraint it is a subtype of, or that has
  no constraint. When no member remains, the result is the target if that is
  a subtype of the type of `x`, the type of `x` if that is assignable to the
  target, the target if that is assignable to the type of `x`, and otherwise
  the intersection of the two. The false branch removes the members that the
  true branch keeps as they are, and keeps `unknown` and the type parameters
  that it intersects. A primitive is compared with an object type through
  the members of its wrapper type, so `string` is a subtype of
  `{ length: number }`.
- A type-guard function keeps the type of `x` in both branches where
  TypeScript narrows it:
  - when the target has only object types and the type of `x` has none and
    isn't `unknown`, as `string | number`, which TypeScript narrows to a
    primitive, as to `string` with `{ length: number }`, or to an
    intersection, as `(string | number) & Foo`;
  - when Biome can't tell whether one of two members is a subtype of the
    other in the way that decides the result: for an index signature,
    `unknown` in a union, instances of two different built-in classes, as
    `TypeError` and `Error`, and an instance of a class compared with a type
    that isn't derived from the class, because a class can declare private
    members. So `x: L | string` with `class L { length = 0 }` keeps its type
    in both branches of `isL(x)`. So does an instance of a class with the
    same name and members as the target class in another namespace, which is
    another class, or in another module, which Biome can't tell from the
    target class;
  - when a property of an object literal or a class field keeps a literal
    type where TypeScript widens it, because Biome can't tell that its value
    is a fresh literal: the result of a generic call, as `1` in
    `{ a: id(1) }`, a read of a `readonly` field or of a constant of a
    namespace, as `A.X`, and an `await`. Such a property is compared both
    ways. So `const o = { a: id(1) }` keeps `{ a: 1 }` in both branches of
    `isA1(o)` with `v is { a: 1 }`, where TypeScript gives `{ a: 1 }` and
    `{ a: number }`.
- The false branch of a type-guard function keeps a member of the type of `x`
  where TypeScript removes it:
  - when the member and a member of the target are subtypes of each other
    and not the same type, because TypeScript's strict subtype relation
    depends on modifiers that Biome doesn't compare, such as `readonly`. With
    `interface Same { a: number }`, `x: Same | Bar` keeps `Same` in the false
    branch of `isFoo(x)`, where TypeScript gives `Bar`, and so does an
    instance of a class that is a subtype of the target;
  - when the member is `unknown`, which TypeScript narrows to
    `null | undefined` with the target `{}`;
  - when the true branch keeps no member, as for `(string | number) & Foo`.
    The false branch is then the type of `x`, which TypeScript makes `never`
    when the type of `x` is assignable to the target.
- A literal type that the true branch keeps, together with its primitive
  type, as `"a"` and `string` from `x: "a" | { length: number }` with
  `v is string`, stays in the false branch, as in TypeScript, whose union of
  the two is the primitive type.
- A literal target on a generic narrows as strict equality with the literal
  does, so `value is "a"` keeps `T`, and gives `T & string` after
  `typeof x === "string"`, where TypeScript gives `T & "a"`. A type
  parameter whose constraint is a subtype of the target, or is derived from
  the class of `instanceof`, stays as it is.
- A library type-guard function narrows within the same rules:
  `Array.isArray` makes `unknown` and `any` `any[]`. `every` narrows an array
  or a `readonly` array, also through `xs?.every`, when the callback's
  target satisfies the constraint of its type parameter, and a tuple narrows
  to its intersection with the array, as
  `[string | number, string | number] & string[]`, whose elements keep their
  types. A callback written in place, as
  `(v): v is string => typeof v === "string"`, narrows nothing through
  `every`, because the callback's own type doesn't give its parameter the
  type that the call gives it. The DOM `matches` method doesn't narrow.
- The results of `filter` and `find` take the type argument of a callback
  with a type predicate, written or inferred, also for a callback written in
  place: `xs.filter(isString)` is `string[]`, `xs.find(isString)` is
  `string | undefined`, and `xs.filter((x) => x !== undefined)` is
  `string[]` for `xs: (string | undefined)[]`. A callback without a type
  predicate, or whose target doesn't satisfy `S extends T`, selects the
  signature that returns `T[]`, so `nums.filter(isString)` is `number[]` for
  `nums: number[]`, as in TypeScript.

### Assertion functions and calls that never return

- A call asserts or ends a path only as a statement call, so
  `(assert(x));` asserts nothing. TypeScript also asserts through an operand
  of the comma operator anywhere else, as in `return (assert(x), x)`. Calls
  inside other expressions, and calls to a type-guard function whose result
  isn't tested, don't narrow.
- An assertion function that the module declares with one signature without
  type parameters follows the rules of such a type-guard function, must be
  declared with `function` or `declare function`, and its call must not be
  optional. Any other callee follows the rules of other type-guard
  functions, including signature selection and type-argument inference.
- Every name along the callee must have an explicit type, as TypeScript
  requires for assertions: a function, a class, a namespace that `declare`
  declares, a variable, a parameter, or a property with a type annotation, a
  method, an import of one of these, a global, and `this` in a class member
  that isn't `static` have one. A variable that an arrow function without a
  type annotation, `new`, or an object literal initializes, an element of a
  destructuring pattern, and an accessor have none, and neither does a callee
  whose type may be `undefined`, as `k?.(x)` with
  `k: ((v: unknown) => asserts v) | undefined`. So
  `const a = (v: unknown): asserts v => {}` asserts nothing, as in
  TypeScript, which reports the call. TypeScript also ignores an assertion
  through an element access.
- `assert(false)` ends the path when its argument is always false: `false`,
  `a && b` with either operand such, or `a || b` with both, with or without
  parentheses. `assert(0)` and `assert(!true)` don't end the path, as in
  TypeScript.
- A statement call ends the path when the return type annotation of its
  signature's declaration is `never` and every name along the callee has an
  explicit type, as for an assertion: a function, a method, a variable or a
  parameter annotated with a function type, a method of a type literal, an
  import of one of these, or a global. So `function fail() { throw 0; }` and
  `const fail = (): never => {}` end nothing, as in TypeScript. A generic or
  overloaded signature ends the path when the call selects one that returns
  `never`, so `failOver("a")` with
  `declare function failOver(a: string): never` and
  `declare function failOver(a: number): void` ends it and `failOver(1)`
  doesn't.
- A statement call ends the path only where Biome finds the signature that
  TypeScript finds, and the statement calls of an execution root after its
  first 64 end nothing. Biome then evaluates the code after such a call as
  reachable, where TypeScript finds it unreachable and gives a read there its
  declared type, so a read there can be narrower than TypeScript. With
  `x: string | null`, `xs: number[]`, and
  `declare function fail<T>(v: T[]): never`, `x` reads `string` in Biome and
  `string | null` in TypeScript in `if (x !== null) { fail(xs); x }`. So does
  a read in a closure created after a call to a function that the module
  doesn't declare, inside a function or arrow expression that Biome doesn't
  analyze, as `g` in
  `if (x !== null) { const g = () => { log.fatal(); return () => x; }; }`.

### Inferred type predicates

- A function, method, or arrow function without a return type annotation,
  neither `async` nor a generator, has the type predicate that TypeScript
  infers when its body is a boolean test of a parameter, written as the
  arrow function's body or as a `return` that is the only statement. The
  target is the parameter's type where the test holds, when that differs
  from the declared type and the test failing leaves nothing of it. So
  `(v: string | null) => v !== null` has `v is string`, while
  `(v: string | null) => !!v` has none, because `""` fails the test.
- The test is a comparison, `instanceof`, `in`, `!`, a call, or `&&` and `||`
  of these. The parameter is the first one that the test mentions, other than
  a rest or destructured parameter and one annotated `boolean`, which
  TypeScript skips too. The test must check the parameter itself or one of
  its properties, as an operand of `typeof`, `!`, `&&`, `||`, or an equality,
  the left operand of `instanceof`, or the right operand of `in`, or pass the
  parameter itself to a call. The parameter has no default value, nothing
  assigns it, and its type comes from an annotation or, for a callback
  written in place, from the call.
- The type predicate is part of the function's type, so calls narrow both
  branches as for other type-guard functions, and `filter`, `find`, and
  `every` infer their results from a callback that has one.
- Biome infers none where TypeScript infers one through statements before
  the `return`, which can narrow the parameter, for a parameter with a
  default value, about a later parameter than the first one that the test
  mentions, or through a test of the parameter in another position, as a
  method call on it. It also infers none where Biome narrows less, as
  `typeof v === "object" && v !== null` on `unknown`, whose failing branch
  keeps functions, or `v !== undefined` on `T | undefined`.

### `switch`

- A case narrows the discriminant as `===` does when the case is a unit value
  and the discriminant is a variable or a property path, with or without
  parentheses. TypeScript also narrows by a case of any other value whose
  type is a unit type, such as a variable with a type annotation or a member
  of an enum whose value Biome doesn't compute, and with a discriminant such
  as `x!`. Every case test sees the types from just before the `switch`, as
  in TypeScript.
- Cases on a property, an element access, a tuple index, an optional chain,
  a `const` that holds a property, and an element of a destructuring
  pattern narrow the object or the pattern as `===` does.
- `default` narrows no receiver of an optional chain, as in TypeScript, so
  `switch (o?.k) { case undefined: break; default: o }` keeps `undefined` in
  `o`.
- Where no case of a `switch` on a value or on `typeof` of one matches, in
  `default` or after a `switch` without one, `unknown` stays `unknown`, as in
  TypeScript, which removes the matched values from `unknown` as a whole
  there. So `switch (x) { case null: break; default: x }` reads `unknown`,
  where `x !== null` gives `{} | undefined`.
- `switch (true)` narrows only when the discriminant is the literal `true`
  itself, as in TypeScript, which narrows nothing for `(true)` either. A
  group with `default` sees the cases outside the group fail.
- A `switch` without `default` is exhaustive when every value that its
  discriminant can have matches a case, as in TypeScript. Biome finds one
  exhaustive only when every case is a unit value, or, for
  `switch (typeof x)`, a string literal or a template literal without
  substitutions that names a kind, and when the discriminant is a variable,
  a property path, or `typeof` of one without parentheses around `typeof`.
  The path that matches no case narrows the discriminant by every case, so a
  read there is Unknown, and the code after the statement sees no value that
  matches no case, so a read of another variable there keeps the narrowing
  of every case. `UnreachableCodeClassificationRequest` reports the code
  after such a statement as unreachable when no other path reaches it.
- Each execution root counts only its first 64 `switch` statements; later
  ones are never exhaustive.
- A discriminant that TypeScript narrows to `never` before the statement,
  through a comparison that Biome doesn't narrow by, such as one with a call
  in `if (x === f()) return;`, makes Biome find the statement exhaustive
  where TypeScript doesn't, because TypeScript treats no `never` type as
  exhausted. Biome then reads another variable after the statement narrower
  than TypeScript, in code that TypeScript types as impossible.

### Optional chains

- The keys and arguments after each `?.`, and the rest of the chain after
  them, as in `x?.[x.k]` or `x?.a.f(x)`, see the object of the `?.` as not
  nullish. The object is tested as not nullish rather than as truthy, so
  `n: 0 | 1 | undefined` stays `0 | 1` in `n?.toFixed(n)`, and the code after
  the chain keeps the types from before it, as in TypeScript. When a key or
  an argument writes a variable, as in `o?.f(x = 1)`, the code after the
  chain joins the flow where each `?.` gives `undefined` with the end of the
  chain.
- Parentheses and `!` end a chain. TypeScript also narrows `x` in
  `x?.a!.b(x)`, and both `o?.v` and `o` in `(o?.v)?.w(o?.v, o)`, where Biome
  narrows only `o?.v`.
- `instanceof` removes `null` and `undefined` from the receivers of an
  optional chain in the true branch, and a group of cases of `switch (x?.y)`
  or `switch (typeof x?.y)` does where every case shows that `x?.y` isn't
  `undefined`, as in TypeScript. Type-guard functions don't narrow the
  receivers.

### Loops

- `for await` loops narrow, but Biome doesn't infer the type of the elements
  of an `AsyncIterable`, so the loop variable of `for await (const v of xs)`
  with `xs: AsyncIterable<T>` is Unknown and doesn't narrow.
- The code after a `for...in` loop doesn't see its expression as not
  nullish, and neither does an object whose property is the expression, as
  `s` in `for (const k in s.kind)`. The receivers of the expression's
  optional chain do, as `o` in `for (const k in o?.v)`. A loop whose head is
  an assignment target, as in `for (k in o)`, or a destructuring pattern,
  narrows nothing. Parentheses around the expression are skipped. TypeScript
  also narrows `o` through `for (const k in o!)`.
- A test in the body of a `for...in` or `for...of` loop never narrows the
  code after the loop, as in TypeScript:
  `for (const v of xs) { if (y === null) return; } y` keeps `y` as declared.

### `try` and `catch`

- A catch clause that completes normally starts the code after the statement
  from the state before `try`, so `try { if (x === null) return; } catch {} x`
  reads `string | null`.
- A `catch` variable has its annotation, `unknown` or `any`, or `unknown`
  without one, and narrows as other variables do.
- A `try` statement without a normal end, as `try { return; } finally {}`,
  makes the code after it unreachable.

### Declaration merging and augmentations

- Interfaces with the same name in one scope merge into one type, as in
  TypeScript, and so do a class and the interfaces with its name: the type
  has the members and the `extends` clauses of every declaration, and the
  first declaration of a property decides its type and whether it is
  readonly. A method that several declarations declare has the overloads of
  all of them, and a call tries those of a later declaration first, and those
  with a parameter of a literal type, such as `"a"`, before all others, as in
  TypeScript. Within one declaration, Biome tries the overloads in their
  order, so with `interface O { f(x: string): 2; f(x: "a"): 1 }` and `o: O`,
  `o.f("a")` gives `2`, where TypeScript gives `1`.
- Interfaces with the same name in two blocks of one namespace, as in
  `namespace N { export interface I { a: 1 } }` and
  `namespace N { export interface I { b: 2 } }`, don't merge: `N.I` is the
  first of them. A declaration in a `declare global` block merges only with
  those in other such blocks.
- Declarations whose type parameters differ, which TypeScript reports unless
  a declaration lists only the first ones of another and leaves out ones
  with defaults, give Unknown, and so do interfaces that give a class a
  construct signature. Two classes with the same name keep their own types.
- The interfaces of a module augmentation, such as
  `declare module "./shapes" { interface X { extra: 1 } }`, merge into the
  interface or class that the augmented module exports, also through
  `export *` and named re-exports. So do the interfaces of a
  `declare global` block, and the top-level interfaces of a declaration file
  without imports or exports, into the global of their name, as in
  TypeScript. Their members count after the target's own, so the target's own
  declaration of a property decides its type and whether it is readonly.
  The overloads of their methods are tried before the target's, those of the
  augmenting module with the last path first, where TypeScript follows
  [the order of the program's files](#overloads-that-augmentations-add-are-tried-in-the-order-of-module-paths).
- Augmentations merge from every module that Biome indexes, which can be
  more or fewer files than the program of a project's `tsconfig.json`, and
  the top-level interfaces of a TypeScript file without imports or exports
  that isn't a declaration file aren't globals. Both can make a read narrower
  than TypeScript's, as the
  [deliberate choices about augmentations](#augmentations-and-the-files-that-biome-indexes)
  describe.
- Only interfaces merge from these declarations. Namespaces and variables
  that a `declare global` block or a module augmentation declares, as
  `namespace NodeJS { interface ProcessEnv {} }` or `var __DEV__: boolean`,
  aren't seen, and neither are those of a TypeScript file without imports or
  exports that isn't a declaration file. An interface that augmentations add
  to a module that doesn't export its name is seen through imports of that
  module, but not through modules that re-export it or through
  `import * as`.
- A global that the standard library doesn't declare, and that more than one
  module declares, is Unknown outside the modules that declare it, and each
  of them sees only its own declarations.
