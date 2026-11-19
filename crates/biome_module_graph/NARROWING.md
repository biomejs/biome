# Narrowing reference

Narrowing gives a read of a variable a more specific type than its declared
type, from the checks and statements on the paths that reach the read. After
`if (x === null) return;`, a parameter `x: string | null` reads as `string`.
Biome's type-aware lint rules, such as `noUnnecessaryConditions` and
`useExhaustiveSwitchCases`, see the narrowed type at each read, so narrowing
decides which checks they find necessary and which code they report.

This reference compares Biome with TypeScript 7.0.2 run with `--strict`.
Examples write a parameter or a declared variable as `x: T` and leave out the
function around it. Several examples use these declarations:

```ts
type Circle = { kind: "c"; r: number };
type Square = { kind: "s"; w: number };
declare const s: Circle | Square;
interface Foo { a: number }
interface Bar { c: number }
declare function isFoo(v: unknown): v is Foo;
declare function isString(v: unknown): v is string;
declare function isNumber(v: unknown): v is number;
```

A result is wider when it has more values than TypeScript's, and narrower
when it has fewer. Unknown is the type that Biome gives a value whose type it
can't tell. How narrowing is built is described in
[`CONTRIBUTING.md`](./CONTRIBUTING.md#flow-sensitive-expression-reads).

## Correctness contract

A narrowed read is never narrower than TypeScript's narrowed type at the same
read, with TypeScript run over the files that Biome indexes. When Biome can't
decide a result, the read keeps its declared type. When a work limit or a
query cycle stops the computation, or a check leaves a branch no possible
value, the read is Unknown instead of `never`. Reads in code that TypeScript
treats as unreachable, impossible, or erroneous can be narrower than
TypeScript's, as the [deliberate choices](#differences-from-typescript) list.

A wider type can make a rule report a union member that TypeScript has
removed. A narrower type makes a rule report code that TypeScript accepts, or
suggest deleting a check that the program needs.

## What narrows

Biome narrows as TypeScript does wherever this reference doesn't say
otherwise. Each result is the type of the last read in its example.

### Values

| Value | Example | Result |
| --- | --- | --- |
| A parameter, a `catch` variable, or a `let`, `const`, `var`, or `function` | `x: string \| null; if (x) x` | `string` |
| An import, and a path on a namespace import | `import * as ns from "./a"; if (ns.x) ns.x`, with `x: string \| null` | `string` |
| A property path | `o: { v: string \| null }; if (o.v) o.v` | `string` |
| An element access with a literal key | `t: [string \| null, number]; if (t["0"]) t[0]` | `string` |
| A private name on `this` | `if (this.#y) this.#y`, with `#y: string \| null` | `string` |
| The object that holds a discriminant | `if (s.kind === "c") s` | `Circle` |
| A `const` that holds a discriminant | `const k = s.kind; if (k === "c") s` | `Circle` |
| An element of a destructuring pattern | `p: { kind: "c"; v: number } \| { kind: "s"; v: string }; const { kind, v } = p; if (kind === "c") v` | `number` |
| An aliased condition, a `const` that holds a condition | `const ok = x !== null; if (ok) x` | `string` |

Each function narrows by its own checks and writes. A module variable read in
a function starts from its declared type, whatever the code around the
function checked. A closure, a function or arrow expression, continues the
narrowing where it is created, and a function or arrow expression called where
it is defined runs in the flow around it.

### Runtime tests

Conditions narrow `if`, `while`, `do...while`, and `for` statements,
conditional expressions, `&&`, `||`, `??`, the cases of `switch`, and the
right operand of a logical assignment. Biome splits a condition through `!`,
`&&`, `||`, and parentheses into these checks.

| Check | Example | Result |
| --- | --- | --- |
| Truthiness | `x: string \| null; if (x) x` | `string` |
| Falsiness | `x: Foo \| undefined; if (!x) x` | `undefined` |
| `===` with a literal | `x: "a" \| "b"; if (x === "a") {} else x` | `"b"` |
| `== null` | `x: string \| number \| null \| undefined; if (x == null) return; x` | `string \| number` |
| `===` with an enum member | `enum E { A, B }; x: 0 \| 1; if (x === E.A) x` | `0` |
| `===` with an annotated variable | `x: string \| number; y: "a" \| 1; if (x === y) x` | `"a" \| 1` |
| A check compared with `true` | `x: string \| number; if ((typeof x === "string") === true) x` | `string` |
| `typeof` | `x: string \| number; if (typeof x !== "string") return; x` | `string` |
| `instanceof` | `x: Date \| string; if (x instanceof Date) x` | `Date` |
| `x.constructor === C` | `class K1 { k1 = 1 }; class K2 { k2 = 2 }; x: K1 \| K2; if (x.constructor === K1) x` | `K1` |
| `in` | `interface A { a: 1 }; interface B { b: 2 }; x: A \| B; if ("a" in x) x` | `A` |
| `#p in x` | `class P { #p = 1; m(x: P \| Date) { if (#p in x) x } }` | `P` |
| A type-guard function | `x: Foo \| string; if (isFoo(x)) {} else x` | `string` |
| A generic type-guard function | `isNonNull<T>(v: T \| null): v is T; x: string \| null; if (isNonNull(x)) x` | `string` |
| An overloaded type-guard function | `isS(v: string \| number): v is string; isS(v: boolean): boolean; x: string \| number; if (isS(x)) x` | `string` |
| A `this is T` method | `class TreeNode { isLeaf(): this is LeafNode }; class LeafNode extends TreeNode {}; n: TreeNode; if (n.isLeaf()) n` | `LeafNode` |
| An inferred type predicate | `const notNull = (v: string \| null) => v !== null; x: string \| null; if (notNull(x)) x` | `string` |
| `Array.isArray` | `x: string \| string[]; if (Array.isArray(x)) x` | `string[]` |
| `every` | `xs: (string \| number)[]; if (xs.every(isString)) xs` | `string[]` |
| `filter` and `find` | `xs: (string \| undefined)[]; xs.filter((x) => x !== undefined)` | `string[]` |
| An optional chain | `x: { y: number } \| undefined; if (x?.y === -1) x` | `{ y: number }` |
| The key and the arguments after `?.`, also after `!` | `x: { a?: { b(v: unknown): void } } \| undefined; x?.a!.b(x)` | `{ a?: { b(v: unknown): void } }` at the argument |

A type-guard function selects its overload by the types of the arguments at
the call, which a check before the call can narrow. Literal type arguments
widen as TypeScript widens them, and a constraint such as `S extends T` in
`filter` is checked against the receiver at the call: after
`xs.every(isNumber)`, `xs.filter(isString)` is `number[]`.

### `unknown`, `any`, and type parameters

These results match TypeScript's and can surprise a reader.

| Check | Example | Result |
| --- | --- | --- |
| Truthiness of `unknown` | `x: unknown; if (x) x` | `{}` |
| `!== null` on `unknown` | `x: unknown; if (x !== null) x` | `{} \| undefined` |
| `typeof` on `unknown` | `x: unknown; if (typeof x === "object") x` | `object \| null` |
| `switch` on `unknown` | `x: unknown; switch (x) { case null: case undefined: break; default: x }` | `unknown` |
| `instanceof` on `unknown` | `x: unknown; if (x instanceof Map) x` | `Map<any, any>` |
| `in` where no member declares the key | `x: { b: 1 }; if ("a" in x) x` | `{ b: 1 } & Record<"a", unknown>` |
| A type-guard function with an unrelated target | `x: Bar; if (isFoo(x)) x` | `Bar & Foo` |
| A type-guard function for `{}` on `unknown` | `isObj(v: unknown): v is {}; x: unknown; if (isObj(x)) {} else x` | `null \| undefined` |
| `typeof` that replaces an object type | `interface Len { length: number }; x: number \| Len; if (typeof x === "string") x` | `string` |
| `typeof` with a string that names no kind of value | `x: string \| Foo; if (typeof x === "strin") x` | `Foo` |
| `any` with `typeof` | `x: any; if (typeof x === "string") x` | `string` |
| `any` with other checks | `x: any; if (x === "a") x` | `any` |
| A property of `any` | `x: any; if (typeof x.v === "string") x.v` | `any` |
| `any` from inference | `const y = JSON.parse(t); if (typeof y === "string") y` | `string` |
| Truthiness of a type parameter | `x: T; if (x) x` | `T & {}` |
| `typeof` on a type parameter | `x: T; if (typeof x === "string") x` | `T & string` |
| `switch` on a type parameter | `x: T; switch (x) { case null: case undefined: break; default: x }` | `T` |
| `#s in x` on a class | `class P { static #s = 2; static s1(x: typeof P \| Date) { if (#s in x) {} else x } }` | `Date \| typeof P` |
| A discriminant through an optional chain | `x: Circle \| Square \| null; if (x?.kind !== "c") x` | `Square \| null` |

Biome writes `{ b: 1 } & Record<"a", unknown>` as `{ b: 1; a: unknown }`, and
`T & {}` where TypeScript writes `NonNullable<T>`.

### Statements and calls

| Statement | Example | Result |
| --- | --- | --- |
| `return`, `throw`, `break`, and `continue` | `x: string \| null; if (x === null) return; x` | `string` |
| `for...in` | `o: Circle \| undefined; for (const k in o) { o }` | `Circle` |
| A `switch` case | `switch (s.kind) { case "c": s }` | `Circle` |
| A `switch` case on `x!` | `x: "a" \| "b" \| "c"; switch (x!) { case "a": x }` | `"a"` |
| `default` | `switch (s.kind) { case "c": break; default: s }` | `Square` |
| `switch (true)` | `x: string \| number \| null; switch (true) { case typeof x === "string": break; case x === null: break; default: x }` | `number` |
| An exhaustive `switch` | `x: boolean \| null \| undefined; y: string \| null; switch (x) { case true: case false: case null: case undefined: if (y === null) return; } y` | `string` |
| An assignment | `x: string \| number; [x] = ["a"]; x` | `string` |
| A `true` or `false` literal assigned to a union with `boolean` | `let b: boolean \| undefined = true; b` | `true` |
| A declarator with a type annotation | `const { a }: { a: string \| number } = { a: 1 }; a` | `number` |
| A `let` without a type annotation | `let x = null; if (c) x = 1; x` | `null \| number` |
| A `let` without an initializer, before a write | `let x; x` | `undefined` |
| An array without a type annotation | `let a = []; a.push(1); a` | `number[]` |
| An assertion function | `assertString(v: unknown): asserts v is string; x: string \| number; (assertString(x), x)` | `string` at the last `x` |
| A function that returns `never` | `fail(): never; x: string \| null; if (x === null) fail(); x` | `string` |
| A function that returns `never` through a type alias | `type Never = never; fail(): Never; x: string \| null; if (x === null) fail(); x` | `string` |
| A closure | `const x: string \| null = f(); if (x) () => x` | `string` |
| Inline code | `x: string \| null; (() => { while (true) { if (x !== null) break; } })(); x` | `string` |
| A branch that a check makes impossible | `x: Date; if (x instanceof Date) {} else x` | Unknown |

An impossible branch, as in the last row, is still reachable code, as in
TypeScript. Only the literal `true` or `false` itself, without parentheses,
makes a branch unreachable, so `while (1)` and `while ((true))` can exit
through their test. A read in unreachable code keeps its declared type.

## Deliberate choices

Some of TypeScript's narrowing is unsound: the narrowed type leaves out
values that the read can have at runtime. Biome copies such narrowing only
when the sound result loses narrowing that idiomatic code relies on and the
unsound case needs unusual code. When a sound result keeps TypeScript's
precision, Biome uses it instead and differs from TypeScript.

### Differences from TypeScript

| Case | Example | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- | --- |
| `??` as a condition | `x: string \| null \| undefined; y: boolean; if (x ?? y) {} else x` | `null \| undefined` | `string \| null \| undefined` | `x` is `""` there when `y` is false, so rules keep `string` in that branch. |
| The falsy part of `number` in `&&` | `n: number; t: string; n && t` | `string \| 0` | `number \| string` | `-0` and `NaN` are falsy too, so rules don't take the falsy result as `0`. |
| Falsiness and object types | `interface Len { length: number }; x: Len \| undefined; if (!x) x` | `undefined` | `Len \| undefined` | `""` is a falsy `Len`, so `noUnnecessaryConditions` doesn't call a check of `x` there unnecessary. |
| `typeof` and object types | `x: string \| Len \| null; if (typeof x !== "object") x` | `string` | `string \| Len` | A string can be a `Len`, so rules keep `Len` in that branch. |
| A failed `typeof` test with a string that names no kind of value | `x: string \| Foo; if (typeof x === "strin") {} else x` | `string` | `string \| Foo` | A `Foo` has the `typeof` `"object"`, so rules keep `Foo` in that branch. |
| A write through a key that isn't a literal | `o: { v: string \| null }; k: "v"; if (o.v) { o[k] = null; o.v }` | `string` | `string \| null` | `o[k]` can write `o.v`, so `noUnnecessaryConditions` doesn't call a later check of `o.v` unnecessary. |
| A private name and a string key | `if (this.#x) this["#x"]` | `string` | `string \| null` | `this["#x"]` is another value, so rules don't apply the check of `this.#x` to it. |
| A `const` that holds a property, after a write | `const k = s.kind; s.kind = "s"; if (k === "c") s` | `Circle` | `Circle \| Square` | `k` holds the value from before the write, so rules don't take `s` as a `Circle`. |
| An aliased condition on a property that one member declares `readonly` | `o: { readonly v: string \| null } \| { v: string \| null }; const ok = o.v !== null; if (ok) o.v` | `string` | `string \| null` | Code can write `v` through the other member, so `noUnnecessaryConditions` doesn't call a check of `o.v` unnecessary. |
| A rest element in a destructuring `const` | `r: { ok: true; v: number } \| { ok: false; e: string }; const { ...ok } = r; if (ok) r` | `{ ok: true; v: number }` | both members | `ok` is a new object, always truthy, so rules don't narrow `r` by it. |
| A failed case of `switch (true)` that isn't a boolean | `x: { a: 1 } \| null; switch (true) { case x: break; default: x }` | `null`, with an error | `{ a: 1 } \| null` | An object reaches `default`, so `noUnnecessaryConditions` doesn't call a check of `x` there unnecessary. |
| Loose `==` with a literal | `x: string \| number; if (x == 1) x` | `1` | `string \| 1` | `"1" == 1` holds, so rules keep `string` in that branch. |
| A condition compared with `false` | `x: { a: 1 } \| null; const ok = x && x.a === 1; if (ok === false) {} else x` | `{ a: 1 }` | `{ a: 1 } \| null` | `ok` is `null` when `x` is `null`, so rules keep `null` in that branch. |
| A write in inline code that throws | `x: string \| null; if (x) { try { (() => { x = null; throw 0; })(); } catch { x } }` | `string` | `string \| null` | The function writes `x` before it throws, so `noUnnecessaryConditions` doesn't call a check of `x` in `catch` unnecessary. |
| A write in an `async` function called in place | `x: string \| null; if (x) { (async () => { x = null; })(); x }` | `string` | `string \| null` | The write runs before the read, so `noUnnecessaryConditions` doesn't call a later check of `x` unnecessary. |
| A write through `super` | `if (this.x) { super.x = null; this.x }`, with `x: string \| null` | `string` | `string \| null` | `super.x = null` sets `this.x`, so `noUnnecessaryConditions` doesn't call a later check of it unnecessary. |
| Literals whose freshness Biome can't trace | `const o = { a: id(1) }; o` | `{ a: number }` | `{ a: 1 }` | Rules keep the literal types that `useExhaustiveSwitchCases` and `noUnnecessaryConditions` check against. |
| A guard passed to `every`, `filter`, or `find` | `xs: (string \| number)[]; if (xs.every(isNumber)) { const r = xs.filter(isString); r }` | `number[]` | `string[]` | Rules keep this narrowing on arrays of unions, and the result still describes the runtime values. |
| A string literal assigned to a string enum | `enum S { A = "a", B = "b" }; let x: S \| number = "a"; x` | `number \| S`, with an error | `"a"` | Biome gives each member the literal type of its value, so rules see the assigned value in code that TypeScript reports. |
| Reads in code that TypeScript treats as unreachable, impossible, or erroneous | `x: boolean; const D = false; if (D) { if (x === D) x }` | `boolean` | `false` | `noUnnecessaryConditions` can call a check there unnecessary, in code that runs only with values outside TypeScript's types. |
| An augmentation in a file that the program leaves out | `pet: Cat \| Dog; if (!("purrs" in pet)) pet` | `Cat \| Dog` | `Dog` | Biome can't tell which files `tsconfig.json` includes, so `noUnnecessaryConditions` can call a needed check unnecessary. |
| A top-level interface in a script that isn't a declaration file | `x: string \| Fmt; if (isFmt(x)) x` | `string \| Fmt` | `Fmt` | Biome can't tell a script from a module without `tsconfig.json`, so `noUnnecessaryConditions` can call a needed check unnecessary. |
| Overloads that augmentations add | `x: X; x.f(0)` | `2` | `3` | Biome has no program file order, so `noUnnecessaryConditions` can report `case 2:` as never matching. |

Falsy branches remove only the object types that no falsy primitive can
have, such as `interface Foo { a: number }`. Loose `==` removes a literal of
another kind only when `==` is certainly false for it, as for a string without
digits compared with a number. `delete o["v"]` gives `o.v` its declared type,
where TypeScript ignores it.

TypeScript widens a literal type when the value is fresh, as `1` in
`let n = 1`. Biome keeps the literal type where it can't tell whether the
value is fresh: the result of a generic call, as `{ a: id(1) }`; a
`static readonly` field, as `let l1 = A.X` with `static readonly X = 1`; a
`const` of a namespace; and `await` of a literal. Widening wherever freshness
is unknown would also widen literal types that TypeScript keeps, such as
`let kind = shape.kind`, and `useExhaustiveSwitchCases` and
`noUnnecessaryConditions` would miss real problems. Where nothing assigns such
a variable another value, rules see its exact value.

A type-guard function passed to `every`, `filter`, or `find` narrows by the
receiver that Biome sees. Where TypeScript has narrowed the receiver through a
check that Biome doesn't narrow by, the result can be narrower than
TypeScript's, but it still describes the runtime values, because every element
passed the guard. Matching TypeScript would mean dropping this narrowing on
arrays whose elements are a union, which rules rely on.

Reads can also be narrower than TypeScript's in these places, which
TypeScript treats as unreachable, impossible, or erroneous:

- after a `switch` that is exhaustive because TypeScript narrowed its
  discriminant to `never` through a check that Biome doesn't narrow by, as
  after `if (x === f()) return;`;
- after a call that never returns that Biome doesn't recognize, as
  `failAll(xs)` with `failAll<T>(v: T[]): never`, or `fail()` when `never`
  comes from a generic type alias, as in `fail(): G<string>` with
  `type G<T> = never`, from a type alias that a declaration file declares
  globally, or from a type alias in the signature of a generic or overloaded
  function of the module, or that comes after the first 64 statement calls of
  a function, and in closures created there;
- after a call that never returns that Biome drops when a function exceeds its
  work budget, or that is part of a query cycle.

TypeScript merges the augmentations of the files in its program, which
`tsconfig.json` selects with options such as `include` and
`moduleDetection`. Biome doesn't read `tsconfig.json`, so it merges the
augmentations of every file that it indexes, in the order of their paths, and
reads top-level interfaces as globals only from `declare global` blocks and
from declaration files without imports or exports. In the examples above,
`cats.ts` adds `purrs: true` to `Cat` and the program leaves it out; the
script `polyfills.ts` declares `interface String { format(): string }`, as
`Fmt` does; and two augmenting modules each add an overload of `f` to `X`.
Treating what augmentations add as undecided would lose narrowing by
augmented members in every project.

### TypeScript's unsound results that Biome copies

| Case | Example | Result | Effect on rules |
| --- | --- | --- | --- |
| `eval` and `arguments` | `x: string \| null; if (x) { (function () { arguments[0] = null; })(); x }` | `string` | Rules keep narrowing in functions that use them, and miss a write through them. |
| Calls that can write a value | `o: { v: string \| null }; reset: () => void; if (o.v) { reset(); o.v }` | `string` | Most checks are followed by a call, so rules keep the narrowing, as TypeScript does. |
| A write through another variable | `o: { v: string \| null }; if (o.v) { const a = o; a.v = null; o.v }` | `string` | Rules keep the narrowing, so `noUnnecessaryConditions` can call a later check of `o.v` unnecessary. |
| A callback that writes a variable | `x: string \| null; if (x) { [1].forEach(() => { x = null; }); x }` | `string` | Rules keep the narrowing, so `noUnnecessaryConditions` can call a later check of `x` unnecessary. |
| A getter | `class G { get v(): string \| null }; g: G; if (g.v) g.v` | `string` | Rules keep narrowing on classes that expose their state through getters. |
| `in` on object types | `x: { a: 1 } \| { b: 2 }; if ("a" in x) x` | `{ a: 1 }` | `{ b: 2 }` can have an `a`, but rules need `in` to tell union members apart. |
| `instanceof` and unrelated members | `class A { a = 1 }; class B extends A { b = 2 }; x: B \| { c: number }; if (x instanceof A) x` | `B` | Rules need `instanceof` to tell union members apart, although an `A` can be a `{ c: number }`. |
| The false branch of `instanceof` | `x: Date \| string; if (x instanceof Date) {} else x` | `string` | Rules keep this narrowing, although a plain object can have type `Date`. |
| A type-guard function and unrelated members | `interface Foo2 extends Foo { b: number }; x: Foo2 \| Bar; if (isFoo(x)) x` | `Foo2` | Rules need type-guard functions to tell union members apart, although a `Bar` can have an `a`. |
| An optional call of a function expression | `x: string \| null; if (x) { ((v?: unknown) => { x = null; })?.(x) }` | `null` at the argument | Following JavaScript's order would give a type disjoint from TypeScript's, which rules would report. |
| The order of destructuring | `x: string \| number; if (typeof x === "string") { [a = (x = 1)] = [x] }` | `number` at `[x]` | Following JavaScript's order would give a type disjoint from TypeScript's, which rules would report. |
| A `let` assigned in another function | `let x: string \| null; if (c) x = "a"; const g = () => { if (x !== null) x }` | `string` | Rules keep narrowing a variable that one function assigns and others read. |
| Code between destructured elements | `x: { kind: "c"; d?: number; v: number } \| { kind: "s"; d?: number; v: string }; const { kind, d = mutate(), v } = x; if (kind === "c") v` | `number` | Rules keep this narrowing, although `mutate` can change `x` before `v` is read. |
| A `switch (true)` group with `default` | `x: string \| null; c: boolean; switch (true) { case c: default: x; break; case x === null: }` | `string` | Rules agree with TypeScript, although `x` can be `null` when `case c:` matches. |

A write in the function's own flow, including inline code, still narrows at
the write.

## Limits

Each table lists code where Biome's result differs from TypeScript's. Biome's
result is wider unless the effect says otherwise, so a rule can miss a report.

### Which reads narrow

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `var x: string \| number = 1; var x: string \| number = "a"; x` | `string` | `string \| number` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |
| `x: any; if (typeof x["v"] === "string") x["v"]` | `string` | `any` | A rule can miss a report on `x["v"]`. |
| `x: any; const [b] = x; b` | `any` | Unknown | A rule can miss a report on `b`. |
| `xs: AsyncIterable<string \| null>; for await (const v of xs) { if (v) v }` | `string` | Unknown | A rule can miss a report on `v`. |
| `o: { a?: string \| number }; const { a = 0 } = o; if (typeof a === "string") a` | `string` | `string \| number` | `noUnnecessaryConditions` can miss an unnecessary check of `a`. |
| `x: string; x !== null ? 1 : x` | `1` | Unknown | A rule can miss a report on the conditional. |

A variable that a destructuring default can supply never narrows, because
its declared type ignores the default. A conditional expression is Unknown
when a check leaves one of its branches no possible value, because a read in
that branch is Unknown. A function gets no narrowing when it contains a
`with` statement or a syntax error, or exceeds a work limit, so rules can miss
reports in it.

### Closures, inline code, and class members

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `let top: string \| null = f(); if (top) () => top`, at the top level of a module | `string` | `string \| null` | `noUnnecessaryConditions` can miss an unnecessary check of `top` in the closure. |
| `x: string \| number; ((y) => { y })(x)` | `string \| number` | Unknown | A rule can miss a report on `y`. |

### Writes

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `x: string \| null; let y: string \| null = f(); if (x) { y = x; y }` | `string` | `string \| null` | `noUnnecessaryConditions` can miss an unnecessary check of `y`. |
| `class K { m() { return 1; } }; k: K; let x: number \| undefined = k.m(); x` | `number` | `number \| undefined` | `noUnnecessaryConditions` can miss an unnecessary `?.` on `x`. |
| `let b: boolean = true; b` | `true` | `boolean` | `noUnnecessaryConditions` can miss that `b` is always `true`. |
| `const T = true; let b: boolean \| undefined = T; b` | `true` | `boolean` | `noUnnecessaryConditions` can miss that `b` is always `true`. |
| `let x: number \| string = 1; x = x + 1; x` | `number` | `number \| string` | A rule can miss that `x` is a number. |
| `xs: string[]; let x: string \| number = 1; [x] = xs; x` | `string` | `string \| number` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |

A write assigns the type of its value without narrowing. Only a `true` or
`false` literal, in parentheses or not, narrows a `boolean` member of the
declared union, because Biome gives some comparisons, as `1 === 1`, the type
`true`, where TypeScript gives `boolean`. A destructuring target keeps its
declared type when it reads an element of an array, as in `[x] = xs` and
`[[x]] = [["a"]]`, or when its default supplies the value, as in
`[x = 1] = []`. A call reduces the assigned type only when its callee has a
return type annotation or is a local function whose returned values Biome can
type. A value that a declaration file types as `typeof` a function, as
`declare const f: typeof impl`, has a return type annotation only when that
function has one.

### Variables whose type evolves with their writes

A `let` or `var` without a type annotation whose initializer is missing,
`null`, or `undefined`, and a variable initialized with `[]`, take the types of
their writes, as in TypeScript. The code below uses
`const store = { load() { return 1; } }`.

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `let x = null; x = store.load(); x` | `number` | Unknown | A rule can miss a report on `x`. |
| `let x = null; x = 0; x++; x` | `number` | Unknown | A rule can miss a report on `x`. |
| `let a = []; a.push(1); if (c) a = null; a` | `any[]`, with an error | Unknown | A rule can miss a report on `a`. |
| `let a = []; a.push(1); a.push(store.load()); a` | `number[]` | `unknown[]` | A rule can miss a report on the elements of `a`. |
| `var v = null; function read() { return v; } v = "a";`, the read in `read` | `any`, with an error | Unknown | A rule can miss a report on `v`. |
| `let w = null; function read() { w } function write() { w = 1; }`, the read in `read` | `any`, with an error | `null \| 1` | Rules see the values that the writes assign, where TypeScript reports an implicit `any`. |

A read is Unknown after a write that Biome can't type as TypeScript does: a
call whose return type Biome infers, an object with a method, a function,
`[]`, `any`, a compound assignment, `++`, or a value that reads another such
variable. A write in a nested block, as in
`let x = null; if (c) { x = store.load(); } x`, gives `null | number`, as in
TypeScript. A read in another function keeps the declared type only when every
write is an `=` or a logical assignment outside the scope of the declaration,
as in the last row. A read that a path reaches without passing the
declarator of such a `var`, as in `x; var x = null;`, is `undefined` there, as
in TypeScript, and an array is Unknown there.

### Property paths

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `x: { z?: { w: string } } \| undefined; if ((x?.z)?.w) x` | `{ z?: { w: string } }` | `{ z?: { w: string } } \| undefined` | `noUnnecessaryConditions` can miss an unnecessary `?.` on `x`. |
| `o: { v: string \| null }; if (o.v) o.v; o!.v = null;` | `string` | `string \| null` | `noUnnecessaryConditions` can miss an unnecessary check of `o.v`. |

A write through `o!.v` or `(o as T).v` stops `o.v` from narrowing anywhere in
the function, also before the write. Only a string literal or a template
literal without substitutions, neither with escapes, or a decimal integer
literal from 0 to 2^53 - 1 is a key of a path, as `o["v"]`, ``o[`v`]``, or
`t[0]`.

### `this`

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `class S { static s: string \| null; static m() { if (this.s) this.s } }` | `string` | Unknown | A rule can miss a report on `this.s`. |
| `use({ x: null, n() { if (this.x) this.x } })`, with `use(o: { x: string \| null; n(): void })` | `string` | Unknown | A rule can miss a report on `this.x`. |

A path on `this` narrows in methods, accessors, and constructors of a class or
of an object literal assigned to a variable, in a function or method whose
`this` parameter has a type annotation, and in arrow functions inside them.

### Destructuring

These rows use `type S = { kind: "c"; v: number } | { kind: "s"; v: string }`.

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `const f: (s: S) => void = ({ kind, v }) => { if (kind === "c") v }` | `number` | Unknown | A rule can miss a report on `v`. |
| `const f: (...args: ["a", number] \| ["b", string]) => void = (k, x) => { if (k === "a") x }` | `number` | Unknown | A rule can miss a report on `x`. |

### Discriminants

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `type Result<T> = { ok: true; value: T } \| { ok: false; error: string }; r: Result<number>; if (r.ok) r` | `{ ok: true; value: number }` | `Result<number>` | A rule can miss a report on `r`. |
| `o: { k: { a: 1 }; x: 1 } \| { k: "x"; y: 2 }; if (o.k === "x") o` | `{ k: "x"; y: 2 }` | both members | A rule can miss a report on `o`. |
| `interface AB { kind: "a" \| "b"; x: 1 }; interface C { kind: "c"; y: 2 }; s: AB \| C; switch (s.kind) { case "a": case "b": break; default: s }` | `C` | `AB \| C` | A rule can miss a report on `s` in `default`. |
| `kindC(): "c"; if (s.kind === kindC()) s` | `Circle` | `Circle \| Square` | A rule can miss a report on `s`. |

### Aliased conditions

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `let m = get(); const ok = m !== null; if (ok) m`, with `get(): string \| null`, at the top level of a module | `string` | `string \| null` | `noUnnecessaryConditions` can miss an unnecessary check of `m`. |
| `class Base { readonly v: string \| null = null }; class Sub extends Base {}; o: Sub; const ok = o.v !== null; if (ok) o.v` | `string` | `string \| null` | `noUnnecessaryConditions` can miss an unnecessary check of `o.v`. |
| `class G { get v(): string \| null }; g: G; const ok = g.v !== null; if (ok) g.v` | `string` | `string \| null` | `noUnnecessaryConditions` can miss an unnecessary check of `g.v`. |

An aliased condition narrows a property path only when every member of the
declared type of each object along it declares the property `readonly`.

### Equality and comparisons

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `x: "a" \| "b"; let y = "a" as const; if (x === y) x` | `"a"` | `"a" \| "b"` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |
| `make(): "a"; x: string \| number; const y = make(); if (x === y) x` | `"a"` | `string \| number` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |
| `x: "a" \| "b"; y: "a" \| "b"; if (y === "a") { if (x !== y) x }` | `"b"` | `"a" \| "b"` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |
| `x: "a" \| "b"; if (x! === "a") x` | `"a"` | `"a" \| "b"` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |

A comparison narrows by a literal, `null`, `undefined`, `void 0`, `-1`, a
template literal without substitutions, an enum member, a `const` that holds
one of these, and the declared type of an annotated variable. Only the
discriminant of a `switch` narrows through `!`, and only the operand of `!`
itself, not the object that holds it, as in TypeScript. Other checks of `x!`,
such as `if (x!)` and `typeof x! === "string"`, narrow nothing.

### Enums

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `enum E { A, B }; E.A` | `E.A`, which is `0` | `0` | none |
| `let wn = E.A; wn` | `E` | `number` | `useExhaustiveSwitchCases` can miss the missing cases of a `switch (wn)`. |
| `enum H { X } enum H { Y = 1 }; H.X` | `0` | Unknown | A rule can miss a report on `H.X`. |
| `enum NE { A } namespace NE {}; NE.A` | `0` | Unknown | A rule can miss a report on `NE.A`. |
| `const K = "k"; enum F { X = K }; F.X` | `"k"` | Unknown | A rule can miss a report on `F.X`. |
| `enum G { C = "c".length }; G.C` | `G.C` | `number` | A rule can miss a report on `G.C`. |
| `declare enum D { P, Q = 2 }; d: D; d` | `D` | `number` | `useExhaustiveSwitchCases` can miss the missing cases of a `switch (d)`. |

Biome gives each enum member the literal type of its value, so a rule that
tells an enum from its primitive type sees a member of a string enum as a
string.

### Truthiness and `typeof`

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `x: T \| undefined, with T extends Foo; if (!x) x` | `undefined` | `T \| undefined` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |
| `x: T, with T extends string \| null \| undefined; if (x !== null) x` | `T & {}` | `(T & {}) \| (T & undefined)` | `noUnnecessaryConditions` can miss an unnecessary check of `x` against `undefined`. |
| `class L { length = 0 }; x: L \| number; if (typeof x === "string") x` | `string` | `L` | A rule can miss a report on `x`. |
| `interface Pad { padStart(n: number): string }; x: Pad \| number; if (typeof x === "string") x` | `string` | `Pad` | A rule can miss a report on `x`. |
| `x: string \| Foo; switch (typeof x) { case "": x }` | `never` | `Foo` | A rule can miss that the case never matches. |
| `x: any; if (typeof x === "strin") x` | `object` | `any` | A rule can miss a report on `x`. |
| `x: string \| void; if (typeof x === "string") x` | `string` | Unknown | A rule can miss a report on `x`. |

A narrowed read of a type with a `void` member is Unknown, because a function
whose return type is `void` can return any value. A class can declare private
members, so Biome doesn't count a primitive as an instance of a class. Biome
compares object types with the members that primitives have in TypeScript's
ES2015 library files, which lack members that newer libraries add, such as
`padStart`.

### `in` and `instanceof`

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `interface A { a: 1 }; interface B { b: 2 }; x: A \| B; const k = "a"; if (k in x) x` | `A` | `A \| B` | A rule can miss a report on `x`. |
| `class P { p = 1 }; class Q { q = 1 }; x: P \| Q; if ("p" in x) x` | `P` | `P \| Q` | A rule can miss a report on `x`. |
| `x: { b: 1 } \| string[]; if ("a" in x) x` | `(string[] \| { b: 1 }) & Record<"a", unknown>` | `{ b: 1 } \| string[]` | A rule can miss a report on `x`. |
| `class Checker { static [Symbol.hasInstance](v: unknown): v is string }; x: string \| Date; if (x instanceof Checker) x` | `string` | `string \| Date` | A rule can miss a report on `x`. |
| `class Box<T> { v!: T }; class C { c = 3 }; GenCtor: new <T>() => Box<T>; x: Box<number> \| C; if (x instanceof GenCtor) x` | `Box<number>` | `Box<number> \| C` | A rule can miss a report on `x`. |

`in` keeps a member in both branches when Biome can't tell whether it has the
property: a primitive, a function, a tuple, an array, an instance of a class,
a built-in type, or an object type with a call signature, a setter, a spread,
or a computed key.

### Type-guard functions

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `get: () => (v: unknown) => v is string; x: string \| number; if (get()(x)) x` | `string` | `string \| number` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |
| `xs: (string \| number)[]; if (xs.every((x) => typeof x === "string")) xs` | `string[]` | `(string \| number)[]` | A rule can miss a report on the elements of `xs`. |
| `q(v: "a"): v is "a"; q(v: string): v is "b"; x: string; if (q(x)) x` | `"b"` | `string` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |
| `isOf<T>(v: unknown, sample: T): v is T; x: string \| number \| boolean; y: string \| number; if (isString(y) && !isOf(x, y)) x` | `number \| boolean` | `string \| number \| boolean` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |
| `class P1 { a = 1 }; class P2 { a = 1 }; k(v: unknown, c: P1): v is string; k(v: unknown, c: unknown): v is number; x: string \| number; c: P2; if (k(x, c)) x` | `string` | `string \| number` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |
| `x: string \| number; if (isFoo(x)) x` | `(string \| number) & Foo` | `string \| number` | A rule can miss a report on `x`. |
| `interface Same { a: number }; x: Same \| Bar; if (isFoo(x)) {} else x` | `Bar` | `Same \| Bar` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |
| `interface Empty {}; isEmpty(v: unknown): v is Empty; x: unknown; if (isEmpty(x)) {} else x` | `null \| undefined` | `unknown` | `noUnnecessaryConditions` can miss an unnecessary check of `x`. |
| `x: { y: string \| number } \| undefined; if (isString(x?.y)) x` | `{ y: string \| number }` | `{ y: string \| number } \| undefined` | `noUnnecessaryConditions` can miss an unnecessary `?.` on `x`. |

Biome skips an earlier overload only when no narrowing could make it accept
the arguments, and narrows only the true branch where TypeScript might infer
a type argument differently, as from `y` in the fourth row. A call asserts or
ends a path only as a statement or as an operand of the comma operator, as
`return (assertString(x), x)`, when every name along the callee has an
explicit type, as TypeScript requires.

### Inferred type predicates

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `const isObj = (v: unknown) => typeof v === "object" && v !== null; x: unknown; if (isObj(x)) x` | `object` | `unknown` | A rule can miss a report on `x`. |
| `function isDefined<T>(v: T \| undefined) { return v !== undefined; }; x: string \| undefined; if (isDefined(x)) x` | `string` | `string \| undefined` | `noUnnecessaryConditions` can miss an unnecessary `?.` on `x`. |

Biome infers a type predicate only when the body is a single boolean check of
a parameter, written as the arrow's body or as the only statement, a
`return`. A parameter with a default value also needs a type annotation.

### `switch`, optional chains, and overloads

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `x: T, with T extends null; switch (x) { case null: break; default: x }` | `never` | `T` | A rule can miss that `default` never runs. |
| ``interface U { f(x: string): 2; f(x: `a`): 1 }; u: U; u.f("a")`` | `1` | `2` | Disjoint: `noUnnecessaryConditions` can report `case 1:` on the result as never matching. |

Biome tries the overloads with a parameter of a literal type first, as
TypeScript does. It doesn't select an overload for a string argument through
a parameter with a template literal type, as in the second row.

### Declarations and globals

| Code | TypeScript | Biome | Effect on rules |
| --- | --- | --- | --- |
| `namespace N { export interface I { a: 1 } } namespace N { export interface I { b: 2 } }; i: N.I; i.b` | `2` | Unknown | A rule can miss a report on `i.b`. |
| `declare global { var __DEV__: string \| null }` in one module, `if (__DEV__) __DEV__` in another | `string` | Unknown | A rule can miss a report on `__DEV__`. |
| `declare global { namespace App { interface Config { v: string \| null } } }` in one module, `c: App.Config; if (c.v) c.v` in another | `string` | Unknown | A rule can miss a report on `c.v`. |

Interfaces of one name in two blocks of a namespace don't merge, and the
first of them declares the type. Biome sees the interfaces of a
`declare global` block, but not its variables and namespaces.

### Work limits

| Limit | Value | Beyond it | Effect on rules |
| --- | --- | --- | --- |
| Syntax nodes in a function, including nested functions | 16,384 | the function's reads keep their declared types | Rules can miss reports in the function. |
| Nesting depth in a function | 128 | the function's reads keep their declared types | Rules can miss reports in the function. |
| Statement calls per function that can end a path | 64 | later calls end nothing | Narrower after such a call, in code that TypeScript finds unreachable. |
| `switch` statements per function that can be exhaustive | 64 | later ones are never exhaustive | `noUnnecessaryConditions` can miss an unnecessary check after them. |
| `const` declarations followed to find a compared value | 16 | the comparison narrows nothing | `noUnnecessaryConditions` can miss an unnecessary check. |
| Nested calls of local functions followed to type a write | 4 | the write keeps the declared type | `noUnnecessaryConditions` can miss an unnecessary check. |
| Elements of an array pattern that narrow the pattern | indices 0 to 15 | later elements don't narrow it | A rule can miss a report on the other elements. |
| Aliased conditions inlined into one another | 5, as in TypeScript | only the innermost is tested | none |
