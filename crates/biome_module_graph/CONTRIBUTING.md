# Contributing to type inference

This guide explains how to extend the Salsa-backed JavaScript and TypeScript
inference engine in `biome_module_graph`. It focuses on choosing request and
query boundaries that preserve incremental reuse and make inference work
observable.

For the representation of inferred types, also read
[`../biome_js_type_info/CONTRIBUTING.md`](../biome_js_type_info/CONTRIBUTING.md).

## Mental model

Type inference has five distinct layers:

```text
syntax and semantic collection
    -> raw module tables
    -> analyzer-facing requests
    -> tracked Salsa queries
    -> resolver helpers
```

### Collection and resolution

During a module-graph scan, `resolve_js_module` processes each parsed JavaScript
or TypeScript file with `TypeInferenceMode::RawTypesOnly`. `JsModuleInfoCollector`
uses the syntax tree and semantic model to record three inputs:

- `raw_types`: declaration and structural type data owned by the module;
- `raw_expressions`: expression ranges mapped to type references;
- `raw_binding_types`: binding ranges mapped to type references.

A raw reference identifies another entry in the module table, a global type, or
a name or import that still needs resolution. It is not an inferred type.

For analyzer consumers, resolution starts when a request invokes a tracked
query. A lookup query selects the raw reference for the requested range or table entry,
and `ResolutionCtx` converts it to `InferredTypeData`. It follows module-table
references, resolves globals, and uses export and lookup queries for imports.
Named declarations remain symbolic `LocalTypeHandle` values until a consumer
normalizes or inspects their structure. In this code, "local" means owned by one
module's type table; it does not mean block- or function-scoped source code.

Start at `src/module_graph.rs`, `src/js_module_info/collector.rs`, and
`src/db/type_inference/resolver.rs`.

### Flow-sensitive expression reads

An expression lookup can give a variable a more specific type at a particular
read. For example, after `if (value === null) return`, a variable typed
`string | null` can be read as `string`. The type stored for the variable's
declaration does not change.

[`NARROWING.md`](./NARROWING.md) is the reference for what narrowing does: its
correctness contract, which runtime tests narrow and which do not yet, its
limits, and where Biome deliberately differs from TypeScript. A change to
narrowing updates the rows it affects there. This section describes how
narrowing is built.

An execution root is a body analyzed independently, such as a module,
function, or method. Each root gets its own graph, and a read never sees facts
from an enclosing root. Biome uses its semantic model to distinguish variables
with the same name.

Flow is split like types: collection gathers syntax-only facts, and tracked
queries do everything that needs a resolved type.

#### Collection

When type inference is enabled, collection (`src/js_module_info/flow.rs`)
builds a graph of the possible paths through each execution root that tests
one of its own variables or a path on its own `this`. Its parts live in
`src/js_module_info/flow/`:

- `scanner.rs` selects roots while the module visitor walks the syntax tree,
  so collection never walks the whole module again. A root is selected when
  its own code contains a condition or, in a module with an `asserts`
  annotation, a standalone call. It also records whether the root's own code
  uses `this`, so a root whose only subjects are paths on `this` still gets a
  graph. The scanner rejects a root when its own code contains a `with`
  statement or bogus syntax, and a root over its node or depth limit. Declaration files (`.d.ts`, `.d.mts`, `.d.cts`, in any letter
  case) contain no code that runs, so the visitor does not look for roots in
  them.
- `builder.rs` builds the graph of one selected root from start, join, and
  condition nodes. It records the incoming flow point of every reachable read
  of a subject and, for each condition node, the subjects its guard may
  narrow. A read of a property path is recorded after its object and key are
  evaluated. The builder skips class members: methods, accessors,
  constructors, and static blocks are roots of their own, and TypeScript
  analyzes a class field that has an initializer apart from the code around
  the class. It also skips `declare` statements, which never run.
- The builder models the control flow of blocks, `if`, `while`, `do...while`,
  `for`, labeled statements, `break`, `continue`, `return`, and `throw`. Any
  other statement, such as `switch`, `try`, `for...in`, or `for...of`, is an
  opaque region. The builder evaluates the expressions of a region in source
  order from the flow point before it, without modelling the branches and
  jumps inside it, and the next statement continues from the result. For each
  `break` or `continue` inside the region that targets a statement outside it,
  the builder adds the point before the region as a predecessor of that
  target, once per target. Regions keep narrowing sound because only subjects
  that the root never writes narrow: the state before a region is never
  narrower than the state at any point inside or after it. Replacing a region
  with a model of its statement only makes narrowing more precise.
- `subjects.rs` maps an expression to the subject it reads or tests: a
  variable or `this`, or a property path that starts at one, such as `o.v`,
  `a.b.c`, `t[0]`, or `this.#v`. A static member access with a named property
  or a private name extends a path, and so does an element access whose key
  spells its property's name: a string
  literal without escapes, or a decimal integer literal from 0 to 2^53 - 1
  without leading zeros or separators. JavaScript converts a number key to a
  string, and such an integer converts to its own digits, so `o["v"]` is the
  same subject as `o.v`, and `t[0]` the same as `t["0"]`. Any other key, such
  as `k`, `-0`, or `0x0`, ends the subject. A private name gets a key of its
  own, so `o.#v` is never the same subject as `o["#v"]`. Parentheses are
  skipped, and `o?.v` is the same subject as `o.v`. Each root stores its
  subjects in a table, a path after its prefixes, so the builder and the
  decomposer give a read and a test of the same subject the same id.
- `this` is a subject only in a root where `this_can_narrow` in `subjects.rs`
  holds: where Biome types `this` as TypeScript does and TypeScript narrows
  its properties as those of a variable. That is a class method or accessor
  that is not `static` and has no `this` parameter, an object literal method
  or accessor, or an arrow function whose `this` comes from one of them or
  from a field initializer, when Biome can name the class or object. A
  constructor has no `this` subject, and neither does an arrow function whose
  `this` comes from a constructor. There, TypeScript reads a field without an
  initializer at its declared type after a test that leaves `undefined`
  possible (TS2565), and an arrow function called where it is defined
  continues the constructor's flow. Each root has its own `this`, so a test
  of `this.v` never reaches a read in a nested function.
- `guards.rs` decomposes every condition into guards on subjects, combined
  the way `!`, `&&`, and `||` combine them. For example,
  `typeof value === "string"` becomes a runtime test of `value`, and
  `o.v !== null` a runtime test of `o.v`. When the tested operand is part of
  an optional chain and the test's result shows that the operand is not
  `undefined`, as the true branch of `x?.y === 1` does, the guard also tests
  each receiver of the chain that is a subject as not nullish, such as `x`
  and `x.y` in `x?.y?.z`, combined with the main test as `&&` combines its
  operands. Which branches narrow the receivers follows TypeScript's
  `narrowTypeByOptionalChainContainment`: neither outcome of `??` does, for
  example. Parentheses end a chain, and a test of a chain that starts at a
  parenthesized optional chain, such as `(x?.y)?.z === 1`, narrows nothing:
  TypeScript narrows `x.y` there but not `x`, so a read of `x?.y` adds
  `undefined` back from `x`, while a narrowed path in Biome would keep its
  narrowed type at that read. A call to a local type-guard
  function becomes a runtime test of its argument: collection selects the
  tested argument from the function's collected signature, and inference
  later resolves only the target of its type predicate. Neither step infers
  the function's body. For a standalone call to an assertion function,
  the builder adds a condition node after the call, so the call and its
  arguments still use the types from before it. `asserts value is T` becomes a
  runtime test of the argument, as `value is T` does, while `asserts value`
  decomposes the argument as a condition that holds, so
  `assert(value !== null)` excludes `null` from `value`.
  A truthiness test of an aliased condition, a `const` that is not exported,
  has no type annotation, and binds a name to an initializer, as `ok` in
  `const ok = x !== null; if (ok)`, also decomposes that initializer and
  combines it with the test of `ok` as `&&` combines its operands. Inside an
  inlined initializer, only a test of a variable whose value cannot change
  between the `const` and the test is kept, following TypeScript's
  `isConstantReference`: a test of a property path, of `this`, of an exported
  variable, or of a variable declared by a `let` statement at the top level
  of a file becomes `Keep`. Inlining stops
  once five aliased conditions enclose a test, where TypeScript stops too, so
  a deeper `const` tests only its own truthiness. When decomposition stops
  early, the subjects that inlined initializers read are mentioned too.
- `writes.rs` finds the property paths that a root may write. The graph has
  no write points, so a written path does not narrow anywhere in its root.
  The scan visits the root's own code, including function and arrow
  expressions called where they are defined and class static blocks, but not
  the bodies of other functions, methods, accessors, and constructors, which
  run only when called. A write cancels the written path and every path that
  extends it. A write through an element access whose key `subjects.rs`
  accepts, as in `o["v"] = x`, writes the path that the key names, and a
  write through any other key, as in `o[k] = x`, cancels every path below its
  object. Every `this` and `super` along a written target stands for the
  root's `this`, so `super.v = x` cancels `this.v`.

Collection then keeps only the reads that a runtime test of their subject can
reach, and only for subjects that can narrow. A variable can narrow when it is
declared in the root, never written, neither untyped nor annotated `any`, and
not supplied by a destructuring default, which the declared type ignores. A
property path can narrow when its variable can, or when it starts at a `this`
that is a subject, and the write scan finds no write that cancels it; the scan
runs only when the root reads a tested path.
Those reads, plus the enclosing expressions that can use a narrowed operand,
become candidates. A test inside a nested function never reaches reads in the
enclosing one, and an array or object literal ends the chain of enclosing
candidates. A root that declares no variable that can narrow and has no `this`
subject gets no graph.
Other variable types are resolved only when a candidate read is queried,
because resolving them can be costly.

#### Tracked solve

Lookups skip flow for any expression that is not a candidate. Guards, the
subjects each condition may narrow, and reads refer to a subject by its id in
its root's subject table, so a test and a read of the same subject match. For
a candidate read of a variable, `infer_flow_binding_baseline` resolves the
variable's declared type once. For a read of a property path, the declared
type is the type of the member access at that read, computed from the
narrowed type of its object when the object has one, as TypeScript types
property reads. Facts about the variable therefore reach its
paths only through that member lookup. `infer_flow_subject_states` then
applies the guards on every path to all reads of the subject in one root in a
single solve, and `infer_flow_subject_type` takes the result at the read's
incoming flow point. Both queries take the starting type as part of their
key, so reads of a subject share one solve when they start from the same
type.

Complete module inference evaluates flow candidates in its own resolution
context instead of through `infer_flow_expression_type` and
`infer_flow_binding_baseline`. When it reaches expressions, that context has
resolved every raw type of the module, so a variable's declared type, and the
member access that gives a property path its declared type, read types it
already holds, while the queries would resolve them again in a fresh context.
It still solves through `infer_flow_subject_states` and
`infer_flow_subject_type`. Cycle fallback and the import-depth fallback read
imports differently from the flow queries, so they keep using the queries.

`src/db/type_inference/flow.rs` applies each runtime test with `narrow_type`
from `biome_js_type_info`. Where branches meet, or a loop returns to an earlier
point, it keeps every type still possible on a reachable path. A read on a path
that no value can take gets type Unknown, not `never`. `flow_expressions.rs`
uses narrowed operands when inferring member accesses, calls, logical and
conditional expressions, `await`, the comma operator, and `+`. Complete module
inference narrows each read as an individual expression query does, from the
declared type its own context resolved.

#### Caching

Collected flow belongs to the module input, so an edit replaces it together
with the raw type tables, even when variable declarations and references stay
the same. Salsa caches the type work: all reads of a variable share the type
that narrowing starts from, its reads in one root share one narrowing
calculation, and reads at the same flow point share their result. Lookups and
complete module inference share the calculation and the results when they
start from the same type. These caches recompute when the module input
changes; they do not track edits separately for each function.

#### Work limits

Collection, condition decomposition, and the solve each have work limits. A
root that exceeds a limit while its flow is collected gets no graph, so its
reads keep their declared types. The limit on finding the reads that a test
can reach is spent on variables before property paths; when it runs out on
paths, the root keeps its graph and only its path reads lose narrowing. If decomposing a condition cannot finish
within its work limit, the guard makes the result unknown rather than assuming
the expression is unaffected. When the shared solve exceeds its work limit or
meets a query cycle, `infer_flow_subject_type` solves the read's point on its
own, so a read never loses precision because other reads share its root. If
that solve also exhausts its work limit or a query cycle occurs, the result is
unknown. Callers must respect that result rather than replace it with a more
confident answer from raw type information.

### Analyzer-facing requests

A request is a typed operation used by analyzer rules. It stores module and
source-range inputs, composes tracked queries through
`TypeInferenceRequestContext`, and returns one defined result, such as a
normalized expression type or the expected type of a call argument.

Requests live under `src/type_inference/requests/`. Every concrete request
implements `TypeInferenceRequestMetadata` beside its execution implementation,
which gives the request a stable profile ID and human label.

### Tracked queries

Tracked queries are the incremental computation units. Expression, binding,
module type, and export queries resolve one entry from a raw table.
Normalization, call matching, and whole-module inference have separate query
boundaries.

The query facade is `src/db/queries/type_inference.rs`. Files under
`src/db/queries/type_inference/` are organized by subject: `lookups`, `exports`,
`promises`, `calls`, `normalization`, and `module_types`. Compound Salsa keys
live in the `interned` module. Algorithms used by one family may live in a child module;
resolution helpers shared by multiple families live under
`src/db/type_inference/`.

### Resolver helpers

Cross-module relationships must remain references to data owned by the source
module. Do not copy or clone inferred data from another module into the current
module, including behind `Arc`.

`ResolutionCtx` protects itself against endless recursion in two ways: it
remembers which types it is currently resolving, and it gives up after a fixed
number of nested steps. It also remembers the types it has already resolved.
All of this is stored inside one context, so a newly created context starts
with none of it.

For this reason, do not create a new context in the middle of a resolution and
keep resolving through it. The new context does not know what the old one was
working on. Two declarations that refer to each other can then keep resolving
each other until the program runs out of stack space and crashes.

Create a new context only where something outside the context still limits the
work:

- at the start of a tracked query. If the same query is requested again before
  it finishes, Salsa returns the query's fallback result instead of running it
  again;
- when following an import, because each import uses up part of a limited
  import budget;
- when sharing the declaration evaluator. It keeps one record of which
  declarations depend on which, across all modules, and uses it to detect
  loops;
- when looking up a member of a namespace import, because that lookup has its
  own step limit that decreases with each step.

To resolve a type that belongs to another module, use a helper that follows one
of these paths, such as `ResolutionCtx::resolve_foreign_type_id`. For example,
in these two files each interface's type parameter refers to the other file's
interface:

```ts
// a.ts
import type { B } from "./b";
export interface A<T extends B<any>> {}

// b.ts
import type { A } from "./a";
export interface B<U extends A<any>> {}
```

Resolving `A`'s type parameter requires `B`, and resolving `B`'s type parameter
requires `A`. If each step creates a new context, no context notices the loop.

## Vocabulary

| Term | Meaning |
| --- | --- |
| Raw table | Unresolved type descriptions and references collected for one module |
| Module type handle | `LocalTypeHandle` reference to a declaration in one module's raw table |
| Request | Typed analyzer operation that computes one inference result from module and range inputs |
| Classification | Explicit `Match`, `NoMatch`, or `Indeterminate` result for a requested condition |
| Lookup query | Query for one expression, binding, export, or module type entry |
| Normalization | Bounded flattening pass that resolves nested module type handles and removes redundant wrappers |
| Namespace expansion | Discovery and resolution of all visible namespace exports |
| Whole-module inference | Resolution of every raw type, expression, and binding in one module |
| Bottom-up scheduling | Untracked dependency-first preparation for whole-module inference |
| Cycle result | Result or recovery path used when recursive tracked queries form a cycle |

"On demand" describes the normal import strategy, not a constant-work
guarantee. A request can still traverse imports, recursively normalize a large
type, or expand a namespace.

## Request architecture

Every production analyzer flow enters through
`execute_type_inference_request`. This boundary associates one caller, request
identity, source origin, and implementation reference with the entire flow.

A request implementation:

1. stores module inputs, source ranges, and static options;
2. implements `TypeInferenceRequestMetadata` with one stable ID and label;
3. declares its implementation reference;
4. returns an exact source origin;
5. composes operations through `TypeInferenceRequestContext`;
6. does not start timers or emit profiling events directly.

`TypeInferenceRequest` extends `TypeInferenceRequestMetadata`, so a request
cannot compile without its own static identity. Runtime inputs cannot select a
different identity. If an option changes the result contract, model the
alternatives as distinct request types and share private input-resolution
helpers between them.

`TypedService` converts syntax nodes and semantic bindings into request inputs.
It should not orchestrate raw query calls.

### Adding a request

Before adding a request, search existing implementations of
`TypeInferenceRequestMetadata`. Reuse a request when it has the same output and
uncertainty behavior. Caller identity, syntax wrappers, input extraction, and
the exact source origin do not create request boundaries.

Add a request only for a new result contract. State the result and each form of
uncertainty precisely, and give the request one canonical `execute`
implementation. Do not encode a tri-state classification as `Option<bool>`;
return `TypeInferenceClassification` so matches, conclusive non-matches, and
indeterminate results remain explicit. If only input extraction differs, reuse
the request and share an input helper.

Add the request to the matching subject file under
`src/type_inference/requests/`. Implement `Sealed`,
`TypeInferenceRequestMetadata`, and `TypeInferenceRequest`; see
`NormalizedExpressionTypeRequest` and `ExpectedCallArgumentTypeRequest` for
complete examples. Then:

1. re-export the request from `requests/mod.rs` and `type_inference/mod.rs`;
2. add a thin `TypedService` method that constructs and executes it;
3. add a flow test that asserts the request uses the intended queries;
4. verify that the flow does not execute whole-module inference unless its
   contract requires it.

Do not add one request per lint rule. The rule is the caller; the request is a
reusable inference operation.

### Request metadata

Like an analyzer rule's metadata trait, `TypeInferenceRequestMetadata` is
implemented by each request type and required by `TypeInferenceRequest`. A
request therefore cannot compile without its static identity.

Request IDs are stable identities used to aggregate records for the same
request contract. Use lowercase words separated by hyphens after the
`request.` prefix:

```text
request.expected-call-argument-type
```

Labels are concise human names for the result. IDs and labels must never contain
source text, symbol names, inferred-type strings, import specifiers, or paths.
Explain output, uncertainty, and widening behavior in the request's rustdoc.

## Adding a tracked query

A request usually composes existing queries. Add a tracked query only when the
result needs an independently memoized invalidation boundary.

### Query file structure

Within a subject file, keep tracked queries, cycle callbacks, and ordinary
helpers in separate regions, in that order. A query region contains only
`#[salsa::tracked]` functions.

### Query acceptance criteria

A new query must satisfy all of these criteria:

1. **Smallest semantic result**: it returns no more information than its
   consumer needs. Prefer a classification or projection query over a
   normalized type when the consumer asks one property.
2. **Two-parameter signature**: the database is the first parameter and one
   Salsa input or interned value is the second. If the query needs multiple
   logical values, enclose them in a new type under `interned`; syntax nodes are
   never query parameters.
3. **Minimal dependencies**: the query reads only inputs needed for its result.
   A Salsa execution test proves that an unrelated edit reuses the result and an
   edit to a consumed input recomputes it.
4. **Defined uncertainty**: missing input, unsupported shapes, ambiguity,
   cycles, and exhausted budgets have documented behavior.
5. **Cycle behavior**: recursive queries define a `cycle_result` callback. The
   callback either runs an explicit recovery algorithm, such as the
   whole-module SCC fallback, or preserves uncertainty. Type queries return
   `Unknown`; classification queries return `Indeterminate`; they do not invent
   a definite type or match result.
6. **Bounded traversal**: recursive shape, overload, import, and export walks
   have an explicit work budget. Do not turn incomplete work into a definite
   negative. Return the request's documented indeterminate result unless the
   helper explicitly documents partial-result semantics.
7. **Observable boundary**: the query belongs to one registered query family
   and has one implementation reference.
8. **Independent validation**: tests prove both correctness and selective Salsa
   execution.

Interning an input gives equal values a shared identity. It does not memoize the
query result by itself.

### Query families

`TypeInferenceQueryKind` identifies the subject module that owns a tracked
query: lookups, exports, promises, calls, normalization, or module types. Add a
query to its existing family without adding an enum variant. Add a family only
when the query needs a new subject module with a distinct responsibility.

### Graph discovery and type resolution

Keep graph-only export discovery separate from type resolution. Origin and
namespace-name queries should inspect module graph inputs without caching
provisional inferred types. The owning expression, binding, or export can then
be resolved with a lookup query.

This separation prevents graph discovery from unnecessarily joining inference
cycles.

### Tracked and untracked entry points

Tracked queries may call other tracked queries directly. New tracked queries
must not call the public bottom-up scheduler. The guarded deep-import fallback
in `imports.rs` is the existing exception.

Code outside a tracked query may use bottom-up scheduling when it deliberately
needs whole-module inference for an import graph. The scheduler is untracked
because it prepares dependencies iteratively and then invokes the tracked
`infer_module_types` query.

## Work that widens inference

Review every new request and query for the following transitions.

### Recursive normalization

Normalization rewrites a type into the directly inspectable form required by a
consumer. It resolves module type handles throughout nested unions,
intersections, tuples, instances, and `typeof` wrappers, then removes wrappers
that no longer add information. It preserves identities such as classes and
interfaces when later operations depend on them.

Do not normalize when a targeted classifier can produce the required result.
Analyzer convenience methods must state whether they normalize their result.
Review the entire request flow, not only its first lookup query.

### Namespace expansion

A namespace import or re-export may discover and resolve every visible export,
including exports reached through blanket re-exports. Treat namespace work as
proportional to the visible namespace, not as an individual lookup.

### Deep import fallback

On-demand import resolution has a recursion limit. Crossing the limit invokes
bottom-up whole-module inference to avoid consuming the Rust stack. A request
that normally stays lookup-oriented can therefore become graph-wide on a
sufficiently deep chain.

The transition is implemented in `src/db/type_inference/imports.rs`.

### Whole-module APIs

`infer_module_types` resolves every raw type-table entry, expression, and
binding in a module. Do not call it from a new request unless the request
contract requires all three tables.

`infer_module_types_bottom_up` additionally prepares static imports and
re-exports dependency-first. Do not call it from a new tracked query.

### Cycle recovery

Lookup query cycles return `Unknown` for type results or `Indeterminate` for
classifications. Whole-module cycles use an SCC fallback that blocks members of
the active component while preserving resolvable dependencies outside it.

Raw declaration recursion is different from a Salsa query cycle. Preserve
symbolic module type handles for recursive declarations rather than eagerly
collapsing them to `Unknown`.

### Call inference

Expected-argument inference may resolve the callee and every sibling argument
before matching signatures. Count this as one request with multiple tracked
query executions; do not describe it as a single constant-cost projection.

## Result semantics

Do not hide distinct outcomes behind `Option<bool>`. A classification request
returns `TypeInferenceClassification`:

- `Match` means available type information proves the requested condition.
- `NoMatch` means available type information proves the condition does not hold.
- `Indeterminate` means inference cannot decide because information is missing,
  unsupported, ambiguous, cyclic, or beyond a work budget.

`None` remains appropriate when a request has no result to return and that
absence has one documented meaning. `Unknown` is an inferred type value whose
structure cannot be resolved. Missing raw entries indicate that collection did
not produce the requested key. Ambiguous exports remain ambiguous rather than
selecting the first match.

Budget exhaustion follows the documented request or helper behavior and must
not be mistaken for a conclusive negative result. Rustdoc on a public request or
query must distinguish every result it can return.

## Source and implementation references

Every request has a `TypeInferenceCodeReference`. It records the source file,
line, and symbol adjacent to the canonical `execute` method. Inference profiles
use this reference to lead maintainers from a reported source range to the
implementation that orchestrated the work.

Each recorded query execution retains its own implementation reference, so
multiple query bodies can share one family. Whole-module records retain the
implementation responsible for widening inference to complete tables. Code
references correspond to the Biome version that produced a report.

Do not read `ModuleInfo::path` from inside a tracked query for profiling. The
path is a Salsa input, so reading it changes the query's dependency set. The
outer request boundary snapshots document paths before entering tracked
queries. Query instrumentation records only work nested under a production
request and carries only module identities and `TextRange` values.

## Profiling inference

Use `--profile-type-inference` to inspect CLI inference work:

```shell
biome lint --profile-type-inference path/to/reproduction
biome check --profile-type-inference path/to/reproduction
```

The report groups records by source file. The default output shows the ten files
with the highest cumulative request time. Within each file, it shows the top
five request groups, top eight query groups, and top five whole-module groups.
It includes the total number of groups, source records, and completed executions
so omitted work remains visible.

Use `--verbose` to show detailed metrics and implementation references for the
top five request records, top eight query records, and top five whole-module
records in each file. Verbose output includes every file while retaining the
per-file limits.

Request and query timings are inclusive and must not be added together. A high
request count with few tracked query executions usually indicates Salsa reuse.
Ranges are zero-based, half-open UTF-8 byte offsets. Profiles contain relative
paths and timings, but not source text, inferred types, or import specifiers.

## Testing

Every behavior change must include correctness tests. Request or query boundary
changes must also include selectivity tests.

### Correctness matrix

Cover the cases relevant to the request or query:

- expressions and bindings in the same module;
- recursive declarations;
- named, default, and namespace imports;
- named, blanket, and namespace re-exports;
- missing and ambiguous exports;
- import and declaration cycles, including cycles through generic parameter
  constraints and defaults declared in different modules;
- disabled inference;
- unsupported input shapes;
- traversal budget exhaustion.

### Salsa execution tests

Use the Salsa event helpers from `biome_db::testing` to assert which tracked
query bodies executed. At minimum, a lookup request test should prove that
`infer_module_types` did not execute.

Test cache reuse separately from correctness:

1. execute the same request twice;
2. assert both consumer requests complete;
3. assert the tracked query body executes only when Salsa recomputes it;
4. edit an unrelated input and assert the query remains reusable;
5. edit a dependency and assert the query recomputes.

Salsa events are a deterministic recomputation oracle. They are not a production
timing mechanism because they do not include query completion events.

### Performance coverage

Benchmark workloads separately:

- cold lookup request;
- warm lookup request;
- dependency edit;
- recursive normalization;
- namespace expansion;
- deep import chain;
- cycle recovery;
- explicit whole-module inference.

Do not compare benchmark names alone. Confirm that the compared benchmarks
invoke the same inference boundary and infer the same tables.

## Review checklist

Before approving a request or query, verify:

- [ ] The consumer enters through a declared request.
- [ ] The request represents a reusable result contract, not a lint rule.
- [ ] The request has an exact source origin.
- [ ] Existing query primitives are reused where possible.
- [ ] New query keys are stable Salsa values.
- [ ] Dependency and invalidation boundaries are stated.
- [ ] Classifications use explicit tri-state results rather than `Option<bool>`.
- [ ] `Indeterminate`, `None`, `Unknown`, ambiguity, and exhaustion are
      distinguished.
- [ ] Recursive traversal has a budget, and recursively dependent tracked
      queries define cycle behavior.
- [ ] Every `ResolutionCtx` created during resolution sits behind a tracked
      query, a spent import or step budget, or the shared declaration
      evaluator.
- [ ] Namespace and import-chain breadth were reviewed.
- [ ] Whole-module inference is absent or explicitly justified.
- [ ] Cross-module data remains owned by its source module.
- [ ] Correctness and Salsa execution tests cover the flow.
- [ ] The metadata trait has a stable ID and label, and the request has an
      implementation reference.
